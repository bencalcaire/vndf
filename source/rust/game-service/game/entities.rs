use common::ecs::Components;
use common::physics::{
	Body,
	Radians,
	Vec2
};

use game::data::Ship;
use network::ClientId;


pub struct Entities {
	pub bodies: Components<Body>,
	pub ships : Components<Ship>
}

impl Entities {
	pub fn create_ship(&mut self, id: ClientId) {
		let velocity = Vec2(30.0, 10.0);
		self.bodies.insert(id, Body {
			position: Vec2::zero(),
			velocity: velocity,
			attitude: Radians::from_vec(velocity)
		});

		self.ships.insert(id, Ship {
			missile_index: 0
		});
	}
}
