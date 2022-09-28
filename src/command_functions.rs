pub mod command_functions {
	use std::io::Write;
	pub fn roll20() {
		println!("rolling... [ ]\r");
		stdout::flush();
	}
}