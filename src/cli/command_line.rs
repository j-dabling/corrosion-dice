pub type Callback = fn();
pub type ArgCallback = fn(args: Vec<String>); // Argument-based callbacks will have to do argument parsing

use std::io::Write;
use colored::Colorize;

pub struct CommandLine {
	// Two seperate command lists is a quick and dirty solution
	// I don't want to deal with inheritance in Rust.
	pub command_list: Vec<crate::cli::command::Command>,
	pub arg_command_list: Vec<crate::cli::command::ArgCommand>,
	pub prompt: String,
}
impl CommandLine {
	pub fn get_input(&self) -> String {
		let mut user_input = String::new();
		print!("{}", self.prompt);
		std::io::stdout().flush().expect("Couldn't flush the display");
		std::io::stdin()
			.read_line(&mut user_input)
			.expect("Error reading line");
		// user_input is now "<whatever_the_user_entered>\n"
		//strip the trailing newline
		user_input.pop();
		// strip any trailing spaces
		loop {
			let check_char = user_input.pop().unwrap();
			if check_char != ' ' {
				user_input.push(check_char);
				break;
			}
		}
		// Now just "<whatever_the_user_entered>"
		user_input
	}
	pub fn input_cycle(&self) {
		loop {
			let initial_user_input = self.get_input();
			let intermediate_user_input = initial_user_input.split(' ');
			let mut user_input = Vec::new();
			let mut command_found = false;
			
			for word in intermediate_user_input {
				user_input.push(String::from(word));
			}
			// Now that we have a vector of strings, we can iterate through our commands and verify each in turn
			// first, check if we should quit
			if user_input[0] == "quit" || user_input[0] == "exit" || user_input[0] == "q" {
				break;
			}
			// check all of our commands
			for command in &self.command_list {
				command_found = command.verify(user_input.clone());
				if command_found {
					break
				}
			}
			// move on to the next iteration if we found something
			if command_found {
				continue;
			}
			// if not, check our argcommands
			for arg_command in &self.arg_command_list {

				command_found = arg_command.verify(user_input.clone());
				if command_found {
					break
				}
			}
			if !command_found {
				println!("{} command not found", "ERROR:".bold().italic().red());
			}
		}
	}
}