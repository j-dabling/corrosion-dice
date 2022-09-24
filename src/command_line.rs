type Callback = fn();
pub mod command {
    // use std::vec;

	pub struct Command {
		pub keyword: String,
		pub callback: crate::command_line::Callback,
	}
	impl Command {
		/* pub fn new(&mut self, keyword: String, callback: crate::command_line::Callback) {
			self.keyword = keyword;
			self.callback = callback;
		} */
		pub fn verify(&self, checkstring: Vec<String>) -> bool {
			if checkstring[0] == self.keyword {
				(self.callback)();
				return true
			}
			else {
				return false
			}
		}
	}
}