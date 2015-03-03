use std::old_io::{
	stdin,
	IoResult,
};
use std::sync::mpsc::{
	channel,
	Receiver,
	TryRecvError,
};
use std::thread::spawn;
use std::vec::Drain;

use cli::Cli;
use client::interface::{
	Frame,
	InputEvent,
};
use ui::Ui;
use window::Window;


pub trait Interface {
	fn new() -> IoResult<Self>;
	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>>;
}


pub struct Player {
	ui: Ui,
}

impl Interface for Player {
	fn new() -> IoResult<Player> {
		let ui = try!(Ui::new());

		Ok(Player {
			ui: ui,
		})
	}

	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		self.ui.update(frame)
	}
}


pub struct CommandLine {
	events: Vec<InputEvent>,
	cli   : Cli,
	window: Window,
}

impl Interface for CommandLine {
	fn new() -> IoResult<CommandLine> {
		let cli    = try!(Cli::new());
		let window = Window::new();

		Ok(CommandLine {
			events: Vec::new(),
			cli   : cli,
			window: window,
		})
	}

	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		try!(self.cli.update(&mut self.events, frame));

		for _ in self.window.poll_events() {}
		if self.window.is_closed() {
			self.events.push(InputEvent::Quit);
		}

		Ok(self.events.drain())
	}
}


pub struct Headless {
	events  : Vec<InputEvent>,
	receiver: Receiver<InputEvent>,
}

impl Interface for Headless {
	fn new() -> IoResult<Headless> {
		let (sender, receiver) = channel();

		spawn(move || -> () {
			let mut stdin = stdin();

			loop {
				// TODO(83541252): This operation should time out to ensure
				//                 panic propagation between tasks.
				match stdin.read_line() {
					Ok(line) => match InputEvent::from_json(line.as_slice()) {
						Ok(event) =>
							match sender.send(event) {
								Ok(()) =>
									(),
								Err(error) =>
									panic!("Error sending input: {:?}", error),
							},
						Err(error) =>
							panic!("Error decoding input: {:?}", error),
					},
					Err(error) =>
						panic!("Error reading from stdin: {}", error),
				}
			}
		});

		Ok(Headless {
			events  : Vec::new(),
			receiver: receiver,
		})
	}

	fn update(&mut self, frame: &Frame) -> IoResult<Drain<InputEvent>> {
		loop {
			match self.receiver.try_recv() {
				Ok(event) =>
					self.events.push(event),
				Err(error) => match error {
					TryRecvError::Empty        => break,
					TryRecvError::Disconnected => panic!("Channel disconnected"),
				}
			}
		}

		print!("{}\n", frame.to_json());

		Ok(self.events.drain())
	}
}
