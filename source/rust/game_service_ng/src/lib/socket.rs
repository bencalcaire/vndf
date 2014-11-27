use std::comm::TryRecvError;
use std::io::IoErrorKind;
use std::io::net::ip::{
	Port,
	SocketAddr,
};
use std::io::net::udp::UdpSocket;

use protocol_ng::Action;


pub struct Socket {
	receiver  : Receiver<Option<(Action, SocketAddr)>>,
	pub socket: UdpSocket,
}

impl Socket {
	pub fn new(port: Port) -> Socket {
		let (sender, receiver) = channel();

		let mut socket       = UdpSocket::bind(("0.0.0.0", port)).unwrap();
		let     socket_field = socket.clone();

		print!("Listening on port {}\n", port);

		spawn(proc() {
			let mut should_run = true;
			let mut buffer     = [0u8, ..512];

			while should_run {
				socket.set_read_timeout(Some(20));
				let message = match socket.recv_from(&mut buffer) {
					Ok((len, address)) => {
						let action =
							Action::from_json(
								String::from_utf8(
									buffer[.. len].to_vec()
								)
								.unwrap()
								.as_slice()
							)
							// TODO(83503278): Handle decoding errors.
							.unwrap();

						Some((action, address))
					},

					Err(error) => {
						match error.kind {
							IoErrorKind::TimedOut =>
								(),
							_ =>
								print!("Error receiving data: {}\n", error),
						}

						None
					},
				};

				match sender.send_opt(message) {
					Ok(()) => (),
					Err(_) => should_run = false,
				}
			}
		});

		Socket {
			receiver: receiver,
			socket  : socket_field,
		}
	}

	pub fn send_to(&mut self, perception: &[u8], address: SocketAddr) {
		match self.socket.send_to(perception, address) {
			Ok(())     => (),
			Err(error) =>
				print!(
					"Error sending data to {}: {}",
					address, error
				),
		}
	}

	pub fn recv_from(&self) -> Option<(Action, SocketAddr)> {
		match self.receiver.try_recv() {
			Ok(message) => message,

			Err(error) => match error {
				TryRecvError::Empty        => return None,
				TryRecvError::Disconnected => panic!("Channel disconnected"),
			}
		}
	}
}
