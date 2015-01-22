use std::collections::HashMap;
use std::io::net::ip::{
	Port,
	SocketAddr,
};

use acpe::protocol::Seq;

use game_service::Socket;

use self::receiver::Receiver;
use self::sender::Sender;


mod receiver;
mod sender;


pub type Clients = HashMap<SocketAddr, Client>;


pub struct Client {
	pub id           : String,
	// TODO: This field can be removed as soon as acpe is replaced, maybe
	//       sooner.
	pub last_action  : Seq,
	pub last_active_s: f64,
	pub broadcast    : Option<String>,
}


pub struct Network {
	pub clients : Clients,
	pub socket  : Socket,
	pub receiver: Receiver,
	pub sender  : Sender,
}

impl Network {
	pub fn new(port: Port) -> Network {
		Network {
			clients : HashMap::new(),
			socket  : Socket::new(port),
			receiver: Receiver::new(),
			sender  : Sender::new(),
		}
	}

	pub fn receive(&mut self) {
		self.receiver.receive(&mut self.socket, &mut self.clients);
	}
}
