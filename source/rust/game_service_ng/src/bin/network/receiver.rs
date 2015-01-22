use std::collections::HashMap;
use std::io::net::ip::SocketAddr;
use std::rand::random;

use acpe::protocol::Seq;
use time::precise_time_s;

use common::protocol::Step;
use game_service::{
	ReceiveResult,
	Socket,
};

use super::{
	Client,
	Clients,
};


pub struct Receiver {
	received: Vec<ReceiveResult>,
}

impl Receiver {
	pub fn new() -> Receiver {
		Receiver {
			received: Vec::new(),
		}
	}

	pub fn receive(&mut self, socket: &mut Socket, clients: &mut Clients, last_actions: &mut HashMap<SocketAddr, Seq>) {
		socket.receive(&mut self.received);

		for result in self.received.drain() {
			match result {
				Ok((mut action, address)) => {
					let action_id = action.header.id;

					for (_, step) in action.drain_update_items() {
						match step {
							Step::Login => {
								clients.insert(address, Client {
									id           : generate_id(),
									last_active_s: precise_time_s(),
									broadcast    : None,
								});
								last_actions.insert(address, action_id);
							},
							Step::Broadcast(broadcast) => {
								match clients.get_mut(&address) {
									Some(client) =>
										client.broadcast = Some(broadcast),
									None =>
										continue, // invalid, ignore
								}
							},
							Step::StopBroadcast => {
								match clients.get_mut(&address) {
									Some(client) =>
										client.broadcast = None,
									None =>
										continue, // invalid, ignore
								}
							},
						}
					}

					match clients.get_mut(&address) {
						Some(client) => {
							client.last_active_s = precise_time_s();
						},
						None =>
							continue, // invalid, ignore
					}
					last_actions.insert(address, action.header.id);
				},
				Err((error, address)) => {
					print!(
						"Error receiving message from {}: {}\n",
						address, error
					);
					clients.remove(&address);
				},
			}
		}
	}
}


// TODO(85374284): The generated id should be guaranteed to be unique.
fn generate_id() -> String {
	fn random_char(min: char, max: char) -> char {
		let min = min as u8;
		let max = max as u8;

		((random::<u8>() % (max + 1 - min)) + min) as char
	}
	fn random_letter() -> char {
		random_char('A', 'Z')
	}
	fn random_letter_or_number() -> char {
		if random() {
			random_letter()
		}
		else {
			random_char('0', '9')
		}
	}

	let mut id = String::new();

	for _ in range(0u8, 3) {
		id.push(random_letter());
	}
	id.push('-');
	for _ in range(0u8, 5) {
		id.push(random_letter_or_number());
	}

	id
}
