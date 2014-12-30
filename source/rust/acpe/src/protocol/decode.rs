use std::io::BufReader;

use root::UPDATE;

use super::Decode;


pub fn decode<H, P>(message: &[u8], update: &mut Vec<P>) -> Result<H, String>
	where
		H: Decode,
		P: Decode,
{
	let mut reader = BufReader::new(message);

	let message = match reader.read_to_string() {
		Ok(message) =>
			message,
		Err(error) => {
			return Err(
				format!("Error converting message to string: {}", error)
			);
		},
	};

	let mut lines = message.split('\n');

	let header = match lines.next() {
		Some(header) =>
			header,
		None => {
			return Err(format!("Header line is missing"));
		},
	};

	let mut splits = header.splitn(1, ' ');
	let     _      = splits.next(); // Ignore header directive

	let header = match splits.next() {
		Some(header) => header,
		None         => return Err(format!("Invalid header")),
	};

	let header = match Decode::decode(header) {
		Ok(header) => header,
		Err(error) => return Err(format!("Error decoding header: {}", error)),
	};

	for line in lines {
		if line.len() == 0 {
			continue;
		}

		let mut splits = line.splitn(1, ' ');

		let directive = match splits.next() {
			Some(directive) =>
				directive,
			None =>
				return Err(format!("Invalid message line: Missing directive")),
		};

		let entity = match splits.next() {
			Some(entity) =>
				entity,
			None =>
				return Err(format!("Invalid message line: Missing entity")),
		};

		match directive {
			UPDATE => match Decode::decode(entity) {
				Ok(entity) =>
					update.push(entity),
				Err(error) =>
					return Err(format!("Error decoding entity: {}", error)),
			},

			_ => return Err(format!("Unknown directive: {}", directive)),
		}
	}

	Ok(header)
}
