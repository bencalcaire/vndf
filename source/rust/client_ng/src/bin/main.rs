#![feature(default_type_params)]
#![feature(slicing_syntax)]


extern crate getopts;
extern crate libc;

extern crate acpe;

extern crate client;
extern crate common;


use std::io::timer::sleep;
use std::time::Duration;

use acpe::protocol::Encoder;

use action_assembler::ActionAssembler;
use args::Args;
use client::network::Socket;
use client::render::{
	Frame,
	Status,
};
use common::protocol::{
	Percept,
	Step,
};
use input::{
	Command,
	CommandError,
	Input,
};
use render::{
	HeadlessRenderer,
	Render,
	Renderer,
};


mod action_assembler;
mod args;
mod input;
mod render;
mod termios;


fn main() {
	let args = Args::parse(std::os::args().as_slice());

	if args.headless {
		match HeadlessRenderer::new() {
			Ok(output) => run(args, output),
			Err(error) => panic!("Error initializing output: {}", error),
		}
	}
	else {
		match Renderer::new() {
			Ok(output) => run(args, output),
			Err(error) => panic!("Error initializing output: {}", error),
		}
	}
}


fn run<R: Render>(args: Args, mut renderer: R) {
	let mut frame = Frame {
		self_id   : String::new(),
		input     : String::new(),
		status    : Status::None,
		commands  : vec![],
		broadcasts: vec![],
	};

	let mut input            = Input::new();
	let mut action_assembler = ActionAssembler::new();
	let mut server           = Socket::new(args.server);
	let mut encoder          = Encoder::new();

	action_assembler.add_step(Step::Login);

	loop {
		for result in input.read_commands().into_iter() {
			match result {
				Ok(command) => {
					let mut reset_status = true;

					match command {
						Command::Broadcast(message) =>
							action_assembler.add_step(Step::Broadcast(message)),
						Command::StopBroadcast =>
							action_assembler.add_step(Step::StopBroadcast),
						Command::Help(text) => {
							frame.status = Status::Notice(text.to_string());
							reset_status = false;
						},
					}

					if reset_status {
						frame.status = Status::None;
					}
				},
				Err(error) => match error {
					CommandError::Incomplete(partial_command, applicable) => {
						frame.input    = partial_command;
						frame.commands = applicable;
					},
					CommandError::Invalid(error, command) =>
						frame.status = Status::Error(
							format!("\"{}\": {}", command, error)
						),
				}
			}
		}

		for perception in  server.recv_from().into_iter() {
			if let Some(self_id) = perception.header.self_id {
				frame.self_id = self_id;
			}

			frame.broadcasts = perception.update
				.into_iter()
				.map(|percept|
					match percept {
						Percept::Broadcast(broadcast) => broadcast,
					}
				)
				.collect();
			action_assembler.process_receipt(perception.header.confirm_action);
		}

		let message = action_assembler.assemble(&mut encoder);

		server.send_to(message);
		if let Err(error) = renderer.render(&frame) {
			// TODO: Improve error handling
			panic!("Error writing output: {}", error);
		}

		sleep(Duration::milliseconds(20));
	}
}
