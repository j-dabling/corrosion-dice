type Callback = fn();
pub mod command {
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
			if args[0] == self.keyword {
				(self.callback)();
				return true
			}
			else {
				return false
			}
		}
	}
}

