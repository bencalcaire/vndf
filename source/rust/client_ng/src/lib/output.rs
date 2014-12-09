use serialize::json::{
	mod,
	DecodeResult,
};

use common::protocol::Broadcast;


#[deriving(Decodable, Encodable, Show)]
pub struct Frame {
	pub self_id   : String,
	pub input     : String,
	pub status    : Status,
	pub commands  : Vec<String>,
	pub broadcasts: Vec<Broadcast>,
}

impl Frame {
	pub fn from_json(json: &str) -> DecodeResult<Frame> {
		json::decode(json)
	}

	pub fn to_json(&self) -> String {
		json::encode(self)
	}
}


#[deriving(Decodable, Encodable, Eq, PartialEq, Show)]
pub enum Status {
	Notice(String),
	Error(String),
	None,
}

impl Status {
	pub fn is_notice(&self) -> bool {
		if let &Status::Notice(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_error(&self) -> bool {
		if let &Status::Error(_) = self {
			true
		}
		else {
			false
		}
	}

	pub fn is_none(&self) -> bool {
		if let &Status::None = self {
			true
		}
		else {
			false
		}
	}
}
