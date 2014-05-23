extern crate collections;
extern crate getopts;
extern crate libc;
extern crate time;

extern crate freetype;
extern crate gl;
extern crate glfw;
extern crate stb_image;

extern crate common;


use common::io::{
	Frame,
	Input,
	InputHandler,
	Renderer
};
use common::physics::{
	Body,
	Vec2
};

use gamestate::GameState;
use network::Network;


mod args;
mod error;
mod gamestate;
mod headless;
mod network;
mod ui;


#[link(name = "stb-image", kind = "static")]
extern {}


pub struct InputSender {
	pub time_of_next_send: u64,
	pub input_to_send    : Input
}


fn main() {
	let args = match args::parse() {
		Some(args) => args,
		None       => error::exit(format!("Failed to parse arguments"))
	};

	let mut network = Network::connect(
		args.address.as_slice(),
		args.port.as_slice());

	let (mut input_handler, mut renderer) = if args.headless {
		headless::init()
	}
	else {
		ui::init()
	};

	let mut game_state = GameState::new();

	let mut camera = Vec2::zero();

	let mut game_input = InputSender {
		time_of_next_send: 0,
		input_to_send    : Input::default()
	};

	let mut should_close = false;
	while !should_close {
		game_state.receive_updates(&mut network);

		let input = input_handler.input();
		should_close = input.exit;

		update_game_input(&mut game_input, input, &mut network, args.period as u64);

		let ships = interpolate_ships_and_camera(&mut game_state, &mut camera);

		let frame = Frame {
			input   : input,
			camera  : camera,
			ships   : ships,
			missiles: game_state.missiles.iter().map(|(_, &body)| body).collect()
		};

		renderer.render(&frame);
	}
}

fn update_game_input(game_input: &mut InputSender, input: Input, network: &mut Network, period_in_ms: u64) {
	game_input.input_to_send.attitude = input.attitude;
	if time::precise_time_ns() >= game_input.time_of_next_send {
		network.send(input);
		game_input.time_of_next_send =
			time::precise_time_ns() + period_in_ms * 1000 * 1000;
	}
}

fn interpolate_ships_and_camera(game_state: &mut GameState, camera: &mut Vec2) -> Vec<Body> {
	let i = {
		let diff = (game_state.current_time - game_state.previous_time) as f64;
		if diff <= 0.0 {
			0.0
		}
		else {
			(time::precise_time_ns() - game_state.current_time) as f64 / diff
		}
	};

	let mut ships = Vec::new();
	for (&ship_id, &current) in game_state.current_ships.iter() {
		match game_state.previous_ships.find(&ship_id) {
			Some(&previous) => {
				let mut body = current.clone();
				body.position = previous.position + (current.position - previous.position) * i;
				ships.push(body);

				match game_state.self_id {
					Some(id) => if id == ship_id {
						*camera = body.position;
					},

					None => ()
				}
			},

			None => ()
		}
	}

	ships
}
