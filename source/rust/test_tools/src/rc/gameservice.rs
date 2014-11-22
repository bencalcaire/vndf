use std::os;

use acceptance::{
	random_port,
	Process,
};

use game_service::initialstate::InitialState;


pub struct GameService {
	pub port   : u16,
	pub process: Process
}

impl GameService {
	pub fn start(initial_state: &InitialState) -> GameService {
		let port = random_port(40000, 50000);

		let mut state_file_name = "initial-state-".to_string();
		state_file_name.push_str(port.to_string().as_slice());
		state_file_name.push_str(".json");

		let mut state_file_path = os::tmpdir();
		state_file_path.push(state_file_name);

		initial_state.to_file(&state_file_path);

		let mut process = Process::start(
			"vndf-game-service",
			&[
				format!("--port={}", port).as_slice(),
				format!("--frame-time=10").as_slice(),
				format!("--initial-state={}",
					state_file_path.to_c_str()
				).as_slice(),
			]
		);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}
