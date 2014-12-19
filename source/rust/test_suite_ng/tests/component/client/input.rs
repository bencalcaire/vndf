use test_tools::Client;


#[test]
fn it_should_display_typed_input_to_the_user() {
	let mut client = Client::start(34481);

	let input = "I'm typing, but not submitting yet, a command";
	client.partial_command(input);

	client.wait_until(|frame| frame.input.as_slice() == input);
}

#[test]
fn it_should_ignore_control_characters() {
	let mut client = Client::start(34481);

	let input = "abc\x11\x12\x13\x14";
	client.partial_command(input);

	client.wait_until(|frame| frame.input.len() == 3);
}

#[test]
fn it_should_remove_the_last_character_with_backspace() {
	let mut client = Client::start(34481);

	client.partial_command("abc\x7f");
	client.wait_until(|frame| frame.input.as_slice() == "ab");

	client.partial_command("\x7f\x7f\x7f\x7f");
	client.wait_until(|frame| {
		print!("{} {}\n", frame.input.as_slice() == "", frame.input.as_slice());
		frame.input.as_slice() == ""
	});
}

#[test]
fn it_should_display_an_error_when_entering_an_invalid_command() {
	let mut client = Client::start(34481);

	client.command("invalid-command");
	client.wait_until(|frame| frame.status.is_error());
}

#[test]
fn it_should_reset_the_error_after_a_successful_command() {
	let mut client = Client::start(34481);

	client.command("invalid-command");
	client.wait_until(|frame| frame.status.is_error());

	client.broadcast("This is a broadcast.");
	client.wait_until(|frame| frame.status.is_none());
}

#[test]
fn it_should_show_applicable_commands_depending_on_input() {
	let mut client = Client::start(34481);

	let frame = client.frame();
	assert!(frame.commands.contains(&"broadcast".to_string()));
	assert!(frame.commands.contains(&"stop-broadcast".to_string()));

	client.partial_command("bro");
	client.wait_until(|frame|
		!frame.commands.contains(&"stop-broadcast".to_string())
	);
	assert!(frame.commands.contains(&"broadcast".to_string()));
}

#[test]
fn it_should_autocomplete_on_tab() {
	let mut client = Client::start(34481);

	client.partial_command("broadcas");
	client.wait_until(|frame| frame.input == "broadcas".to_string());

	client.partial_command("\x09"); // Tab
	client.wait_until(|frame| frame.input == "broadcast ".to_string());
}
