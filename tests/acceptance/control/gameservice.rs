use rand;

use util::Process;


pub struct GameService {
	pub port   : u16,
	pub process: Process
}

impl GameService {
	pub fn start() -> GameService {
		let port = rand::random::<u16>() % 10000 + 40000;

		let mut process = Process::start(
			"output/bin/vndf-game-service",
			[
				~"--port", port.to_str(),
				~"--frame-time", ~"10"]);
		process.read_stdout_line(); // Make sure it's ready

		GameService {
			port   : port,
			process: process
		}
	}
}
