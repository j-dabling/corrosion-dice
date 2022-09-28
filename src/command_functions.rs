pub mod command_functions {
	// use std::io::Write;
	// use std::thread::sleep;
	// use std::time;
	use std::io::Write;
	use rand::Rng;
	pub fn roll20() {
		println!("rolling... [ ]\r");
		std::io::stdout().flush().expect("couldn't flush the display");
		// initially, we won't do the fancy stuff
		let final_result = rand::thread_rng().gen_range(1..=20); // generate random integer between 1 and 20
		println!("result: [{final_result}]");
	}
}