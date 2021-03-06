use std::cmp::max;
use std::env;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::iter::repeat;
use std::path::Path;


fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let path    = Path::new(&out_dir).join("entities.rs");

    let mut file = File::create(&path).unwrap();

    Entities::new()
        .with_component("body"     , "bodies"    , "Body"     )
        .with_component("broadcast", "broadcasts", "Broadcast")
        .with_component("maneuver" , "maneuvers" , "Maneuver" )
        .with_component("planet"   , "planets"   , "Planet"   )
        .with_component("ship"     , "ships"     , "Ship"     )
        .generate(&mut file)
        .unwrap();
}


struct Entities {
    components: Vec<(&'static str, &'static str, &'static str)>,
}

impl Entities {
    fn new() -> Entities {
        Entities {
            components: Vec::new(),
        }
    }

    fn with_component(
        mut self,
        name      : &'static str,
        collection: &'static str,
        type_name : &'static str,
    ) -> Self {
        self.components.push((name, collection, type_name));
        self
    }

    fn generate<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        let max_collection_length = self.components.iter().fold(
            0,
            |length, &(_, collection, _)|
                max(length, collection.chars().count())
        );

        let padding = |collection: &str| -> String {
            repeat(" ")
                .take(max_collection_length - collection.chars().count())
                .collect()
        };

        try!(writer.write_all(
b"use std::collections::{
    HashMap,
    HashSet,
};


pub type Components<T> = HashMap<EntityId, T>;


#[derive(Debug)]
pub struct Entities {
    next_id: u64,

    pub entities: HashSet<EntityId>,

"
        ));

        for &(_, collection, type_name) in &self.components {
            try!(write!(
                writer,
                "    pub {}{}: Components<{}>,\n",
                collection, padding(collection), type_name,
            ));
        }

        try!(writer.write_all(
b"
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            next_id: 0,

            entities: HashSet::new(),

"
        ));

        for &(_, collection, _) in &self.components {
            try!(write!(
                writer,
                "            {}{}: HashMap::new(),\n",
                collection, padding(collection),
            ));
        }

        try!(writer.write_all(
b"
        }
    }

    pub fn create_entity(&mut self) -> EntityBuilder {
        let id = self.next_id;
        self.next_id += 1;

        self.entities.insert(id);

        EntityBuilder {
            id: id,

"
        ));

        for &(_, collection, _) in &self.components {
            try!(write!(
                writer,
                "            {}{}: &mut self.{},\n",
                collection, padding(collection), collection,
            ));
        }

        try!(writer.write_all(
b"
        }
    }

    pub fn update_entity(&mut self, id: EntityId) -> EntityUpdater {
        EntityUpdater {
            id: id,
"
        ));

        for &(_, collection, _) in &self.components {
            try!(write!(
                writer,
                "            {}{}: &mut self.{},\n",
                collection, padding(collection), collection,
            ));
        }

        try!(writer.write_all(
b"
        }
    }

    pub fn destroy_entity(&mut self, id: &EntityId) {
"
        ));

        for &(_, collection, _) in &self.components {
            try!(write!(
                writer,
                "        self.{}.remove(id);\n",
                collection,
            ));
        }

        try!(writer.write_all(
b"
        self.entities.remove(id);
    }
}


pub struct EntityBuilder<'c> {
    id: EntityId,

"
        ));

        for &(_, collection, type_name) in &self.components {
            try!(write!(
                writer,
                "    {}{}: &'c mut Components<{}>,\n",
                collection, padding(collection), type_name,
            ));
        }

        try!(writer.write_all(
b"
}

impl<'c> EntityBuilder<'c> {
"
        ));

        for &(name, collection, type_name) in &self.components {
            try!(write!(
                writer,
"    pub fn with_{}(mut self, component: {}) -> EntityBuilder<'c> {{
        self.{}.insert(self.id, component);
        self
    }}
",
                name, type_name, collection,
            ));
        }

        try!(writer.write_all(
b"
    pub fn return_id(self) -> EntityId {
        self.id
    }
}


pub struct EntityUpdater<'c> {
    id: EntityId,

"
        ));

        for &(_, collection, type_name) in &self.components {
            try!(write!(
                writer,
                "    {}{}: &'c mut Components<{}>,\n",
                collection, padding(collection), type_name,
            ));
        }

        try!(writer.write_all(
b"
}

impl<'c> EntityUpdater<'c> {
"
        ));

        for &(name, collection, type_name) in &self.components {
            try!(write!(
                writer,
"    pub fn add_{}(mut self, component: {}) -> EntityUpdater<'c> {{
        self.{}.insert(self.id, component);
        self
    }}
",
                name, type_name, collection,
            ));
        }

        try!(writer.write_all(
b"
"
        ));

        for &(name, collection, _) in &self.components {
            try!(write!(
                writer,
"    pub fn remove_{}(mut self) -> EntityUpdater<'c> {{
        self.{}.remove(&self.id);
        self
    }}
",
                name, collection,
            ));
        }

        try!(writer.write_all(
b"}
"
        ));

        Ok(())
    }
}
