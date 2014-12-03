use root::MAX_PACKET_SIZE;

use super::{
	decode,
	Encoder,
	Message,
	Part,
	Seq,
};


#[deriving(Clone, PartialEq, Show)]
pub struct Perception<Id, Percept> {
	pub header  : PerceptionHeader<Id>,
	pub percepts: Vec<Percept>,
}

impl<Id, Percept: Part> Perception<Id, Percept> {
	pub fn decode(message: &[u8]) -> Result<Perception<Id, Percept>, String> {
		let mut percepts = Vec::new();
		match decode(message, &mut percepts) {
			Ok(last_action) =>
				Ok(Perception {
					header: PerceptionHeader {
						confirm_action: last_action,
						// TODO: Add support for self id to encode/decode
						self_id       : None
					},
					percepts: percepts,
				}),
			Err(error) =>
				Err(error),
		}
	}

	/// This is a convenience method that makes encoding as easy as possible,
	/// ignoring performance and error handling. Please don't use this outside
	/// of test code.
	pub fn encode(self) -> Vec<u8> {
		let mut buffer  = [0, ..MAX_PACKET_SIZE];
		let mut encoder = Encoder::new();

		// TODO: Simplify generic arguments.
		let mut perception = encoder.message::<Perception<Id, _>, _, _>(self.header.confirm_action);
		for percept in self.percepts.iter() {
			perception.add(percept);
		}

		let message = perception
			.encode(&mut buffer)
			.unwrap_or_else(|error|
				panic!("Error encoding perception: {}", error)
			);

		message.to_vec()
	}
}

impl<Id, Percept: Part> Message<Seq, Percept> for Perception<Id, Percept> {}


#[deriving(Clone, PartialEq, Show)]
pub struct PerceptionHeader<Id> {
	pub confirm_action: Seq,
	pub self_id       : Option<Id>,
}
