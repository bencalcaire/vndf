use collections::Deque;
use collections::RingBuf;
use std::f64;
use std::libc;

use common::vec::Vec3;

use clients::Clients;

extern {
	fn close(fd: libc::c_int) -> libc::c_int;
}


pub struct Events {
	buffer: ~Deque<Event>
}

pub enum Event {
	Connect(libc::c_int),
	Disconnect(uint),
	Update
}

impl Events {
	pub fn new() -> ~Events {
		~Events {
			buffer: ~RingBuf::<Event>::new() }
	}

	pub fn push(&mut self, event: Event) {
		self.buffer.push_back(event)
	}

	pub fn pull(&mut self) -> Option<Event> {
		self.buffer.pop_front()
	}
}


pub fn handle_events(events: &mut Events, clients: &mut Clients, frameTimeInMs: uint) {
	loop {
		match events.pull() {
			Some(event) =>
				match event {
					Connect(clientFD)    => on_connect(clientFD, clients),
					Disconnect(clientId) => on_disconnect(clientId, clients, events),
					Update               => on_update(clients, events, frameTimeInMs as f64 / 1000.0)
				},

			None => break
		}
	}
}

fn on_connect(clientFD: libc::c_int, clients: &mut Clients) {
	let distance = 100.0;

	let alpha = 90.0 / 180.0 * f64::consts::PI;

	let pos = Vec3 {
		x: distance * f64::cos(alpha),
		y: distance * f64::sin(alpha),
		z: 0.0 };

	let vel = Vec3 {
		x: 30.0,
		y: 0.0,
		z: 0.0 };

	if !clients.add(clientFD, pos, vel) {
		unsafe {
			close(clientFD);
		}
	}
}

fn on_disconnect(clientId: uint, clients: &mut Clients, events: &mut Events) {
	clients.remove(clientId);

	clients.each(|client| {
		let status = ::protocol::send_remove(
			client.socketFD,
			clientId);

		if status < 0 {
			events.push(Disconnect(client.id));
		}
	})
}

fn on_update(clients: &mut Clients, events: &mut Events, dTimeInS: f64) {
	clients.mut_each(|client| {
		client.ship.pos = client.ship.pos + client.ship.vel * dTimeInS;
	});

	clients.each(|clientA| {
		clients.each(|clientB| {
			let status = ::protocol::send_update(
				clientA.socketFD,
				clientB.id,
				clientB.ship.pos.x,
				clientB.ship.pos.y);

			if status < 0 {
				events.push(Disconnect(clientA.id));
			}
		})
	});
}
