// type Callback = fn();
// type ArgCallback = fn(args: Vec<String>); // Argument-based callbacks will have to do argument parsing
use std::fmt;

// Holds our command class attributes.
pub struct Command {
	pub keyword: String, // The word that will trigger the callback
	pub callback: crate::cli::command_line::Callback, // The function to be executed
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
	pub callback: crate::cli::command_line::ArgCallback, // the function to be executed
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