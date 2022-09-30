type Callback = fn();
type ArgCallback = fn(args: Vec<String>); // Argument-based callbacks will have to do argument parsing
pub mod command {
	use std::fmt;
	// Holds our command class attributes.
	pub struct Command {
		pub keyword: String, // The word that will trigger the callback
		pub callback: crate::command_line::Callback, // The function to be executed
	}
	impl Command {
		// Command::verify
		// 	Checks the first word in the argued vector of arguments.
		// 	If there is a match, it will call the callback and return true, which the 
		// 	CommandLine object will use to know that a match has been found and stop 
		//	looking.
		// 	If no match is found, false is returned, used by the CommandLine object
		// 	to keep looking.
		pub fn verify(&self, args: Vec<String>) -> bool {
			if args[0].to_lowercase() == self.keyword {
				(self.callback)();
				return true
			}
			else {
				return false
			}
		}
	}
	impl fmt::Display for Command {
		fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
			write!(f, "{}", self.keyword)
		}
	}

	pub struct ArgCommand {
		pub keyword: String, // word to recognize the command 
		pub callback: crate::command_line::ArgCallback, // the function to be executed
		pub default_arguments: Vec<String>, // must be handled the same way as any other arguments
	}
	impl ArgCommand {
		pub fn verify(&self, args: Vec<String>) -> bool {
			// args includes the first word in the command string
			if args[0].to_lowercase() == self.keyword {
				if args.len() > 1 { // If another argument was passed
					(self.callback)(args);
				}
				else if args.len() == 1 {
					(self.callback)(self.default_arguments.clone());
				}
				return true
			}
			return false
		}
	}
}

pub mod command_line {
	use std::io::Write;

use colored::Colorize;

	pub struct CommandLine {
		// Two seperate command lists is a quick and dirty solution
		// I don't want to deal with inheritance in Rust.
		pub command_list: Vec<crate::Command>,
		pub arg_command_list: Vec<crate::ArgCommand>,
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
					println!("{}", "command not found".red())
				}
			}
		}
	}
}