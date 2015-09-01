use nalgebra::{Vec2,Translation};
use shared::game::EntityId;
use client::interface::Frame;

/// Camera tracking types
#[derive(Debug,Clone,RustcDecodable,RustcEncodable,PartialEq)]
pub enum CameraTrack {
    Entity(Vec<EntityId>),
    Position,
    Default,
}

pub struct Camera {
    track: CameraTrack,
    pos: Vec2<f64>,
    speed: f64, // camera transition speed
    // TODO: consider camera easing
}

impl Camera {
    pub fn new () -> Camera {
        Camera {
            track: CameraTrack::Position,
            pos: Vec2::new(0.0,0.0),
            speed: 5.0,
        }
    }

    pub fn set (&mut self, tracking: CameraTrack) {
        self.track = tracking;
    }
    
    /// must be called to update camera positioning
    pub fn update (&mut self,
                   frame: &Frame,
                   offset: Option<Vec2<f64>>)
                   -> Vec2<f64> {
        let mut pos = Vec2::new(0.0,0.0);
        let mut vel = Vec2::new(0.0,0.0);
        
        match self.track {
            CameraTrack::Entity(ref v) => {                
                let (p,v) = Camera::get_average_pos(&v,&frame);
                pos = p;
                vel = v;
            },
            CameraTrack::Default => { 
                if let Some(id) = frame.ship_id {
                    self.track = CameraTrack::Entity(vec!(id));
                }
            },
            _ => (),
        }
        
        if let Some(offset) = offset {
            pos = pos+offset;
        }

        // NOTE: must invert each coordinate to track
        pos = pos.inv_translation();

        self.pos = pos;
        self.pos
    }

    /// gets the average position of multiple entities
    // NOTE: This assumes that frame will hold all entities (eg: ships & planets)
    pub fn get_average_pos (v: &Vec<EntityId>, frame: &Frame) -> (Vec2<f64>,Vec2<f64>) {
        let mut pos = Vec2::new(0.0,0.0);
        let mut vel = Vec2::new(0.0,0.0);
        let total_ships = v.len() as f64;
        let total = Vec2::new(total_ships,total_ships);
        
        // for now grab ships
        for n in v.iter() {
            if let Some(b) = frame.ships.get(&n) {
                pos = pos + b.position;
                vel = vel + b.velocity;
            }
        }

        (pos/total,
         vel/total)
    }

    pub fn get_pos (&self) -> Vec2<f64> {
        self.pos
    }
}
