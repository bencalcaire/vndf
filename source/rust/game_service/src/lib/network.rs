use std::collections::HashMap;
use std::comm::{
	Disconnected,
	Empty
};
use std::io;

use epoll;
use epoll::EPoll;
use net::{
	Acceptor,
	Connection,
	ConnId,
};
use protocol::Action;

use super::events::{
	mod,
	GameEvent,
	NetworkEvent,
};


pub struct Network {
	pub events: Sender<NetworkEvent>,

	incoming   : Receiver<NetworkEvent>,
	epoll      : EPoll,
	acceptor   : Acceptor,
	connections: HashMap<ConnId, Connection>
}

impl Network {
	pub fn new(port: &str) -> Network {
		let epoll = match EPoll::create() {
			Ok(epoll)  => epoll,
			Err(error) => panic!("Error initializing epoll: {}", error)
		};

		let acceptor = match Acceptor::new(port) {
			Ok(acceptor) => acceptor,
			Err(error)   => panic!("Error creating acceptor: {}", error)
		};

		match epoll.add(acceptor.fd, epoll::ffi::EPOLLIN) {
			Ok(()) => (),

			Err(error) =>
				panic!("Error registering server socket with epoll: {}", error)
		}

		let (sender, receiver) = channel();

		Network {
			events     : sender,
			incoming   : receiver,
			epoll      : epoll,
			acceptor   : acceptor,
			connections: HashMap::new()

		}
	}

	pub fn update(&mut self, timeout_in_ms: u32, game: &mut Sender<GameEvent>) {
		loop {
			match self.incoming.try_recv() {
				Ok(event) => match event {
					NetworkEvent::Message(recipients, message) => {
						for &id in recipients.iter() {
							let connection = match self.connections.get(&id) {
								Some(connection) => connection,
								None             => return
							};

							match connection.send_message(message.to_string().as_slice()) {
								Ok(())     => (),
								Err(error) => self.events.send(NetworkEvent::Close(id, error))
							}
						}
					},

					NetworkEvent::Close(id, error) => match self.connections.remove(&id) {
						Some(conn) => {
							let _ =
								write!(
									&mut io::stderr(),
									"Closing connection due to error: {}\n",
									error
								);
							conn.close();
							game.send(GameEvent::Leave(id));
						},

						None => ()
					}
				},

				Err(error) => match error {
					Empty        => break,
					Disconnected => panic!("Unexpected error: {}", error)
				}
			}
		}

		let mut to_accept = Vec::new();

		match self.epoll.wait(timeout_in_ms) {
			Ok(fds) => for &fd in fds.iter() {
				if fd == self.acceptor.fd {
					to_accept.push(fd);
				}
				else {
					let client_id = fd as ConnId;

					let conn = match self.connections.get_mut(&client_id) {
						Some(result) => result,
						None         => return
					};

					let result = conn.receive_messages(|raw_message| {
						let action = match Action::from_string(raw_message.as_slice()) {
							Ok(message) => message,

							Err(error) =>
								panic!("Error decoding message: {}", error)
						};

						game.send(GameEvent::Action(fd as ConnId, action));
					});

					match result {
						Ok(())     => (),
						Err(error) => self.events.send(NetworkEvent::Close(client_id, error))
					}
				}
			},

			Err(error) => panic!("Error while waiting for events: {}", error)
		}

		for _ in to_accept.iter() {
			let connection = match self.acceptor.accept() {
				Ok(connection) => connection,

				Err(error) =>
					panic!("Error accepting connection: {}", error)
			};

			match self.epoll.add(connection.fd, epoll::ffi::EPOLLIN) {
				Ok(()) => (),

				Err(error) =>
					panic!("Error adding to epoll: {}", error)
			}

			let client_id = connection.fd as ConnId;
			self.connections.insert(client_id, connection);
			game.send(GameEvent::Enter(client_id));
		}
	}
}
