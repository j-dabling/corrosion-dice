type Callback = fn();
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
			// remove this crusty debug stuff!!
			// println!("keyword for command: {self}, args being considered = {:?}", args);
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
}

pub mod command_line {
	use std::io::Write;
	pub struct CommandLine {
		pub command_list: Vec<crate::Command>,
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
			let initial_user_input = self.get_input();
			let intermediate_user_input = initial_user_input.split(' ');
			let mut user_input = Vec::new();
			for word in intermediate_user_input {
				user_input.push(String::from(word));
			}
			// Now that we have a vector of strings, we can iterate through our commands and verify each in turn
			for command in &self.command_list {
				//remove this crusty debug stuff!!!
				// println!("{command}");
				command.verify(user_input.clone());
			}

		}
	}
}