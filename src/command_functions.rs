pub mod command_functions {
	use std::{thread, time};
	use std::io::Write;
	use rand::Rng;
	use colored::Colorize;
	// Rolls a random number between 1 and 20, putting the result to the screen in a cool-looking way
	pub fn roll20() {
		print!("{}\r", "rolling...".black());
		std::io::stdout().flush().expect("couldn't flush the display");
		thread::sleep(time::Duration::from_millis(500));
		// This is the number of times that the 'die' will change before settling on the final result
		let target = rand::thread_rng().gen_range(4..=20);
		let mut i = 0;
		while i < target {
			let temp_result = rand::thread_rng().gen_range(1..=20);
			print!("{} {} {} {}           \r",
			"rolling...".black(),
			"[".white(),
			dynamic_color(temp_result, 20),
			"]".white()); // 20 hardcoded will have to change for rolln
			std::io::stdout().flush().expect("couldn't flush the display");
			thread::sleep(time::Duration::from_millis(i * 30));
			i += 1;
		}
		let final_result = rand::thread_rng().gen_range(1..=20); // generate random integer between 1 and 20
		println!("{} {} {} {}                 ",
		"result: ".blue().bold(),
		"[".white().bold(),
		dynamic_color(final_result, 20),
		"]".white().bold());
		// the extra spaces are to make sure to overwrite the previous output
	}

	pub fn rolln(n: u64) {
		print!("{}\r", "rolling...".black());
		std::io::stdout().flush().expect("couldn't flush the display");
		thread::sleep(time::Duration::from_millis(500));
		let mut target: u64 = rand::thread_rng().gen_range((n / 5)..=n);
		if target > 30 { // cap the number of re-rolls at 30 so we can safely roll a d100
			target = 30;
		}
		let mut i :u64 = 0;
		while i <= target {
			let temp_result = rand::thread_rng().gen_range(1..=n);
			print!("{} {} {} {}           \r",
			"rolling...".black(),
			"[".white(),
			dynamic_color(temp_result, n), // pass n as max
			"]".white());
			std::io::stdout().flush().expect("couldn't flush the display");
			thread::sleep(time::Duration::from_millis(i * 30));
			i += 1;
		}
		let final_result = rand::thread_rng().gen_range(1..=n); // generate random integer between 1 and 20
		println!("{} {} {} {}                 ",
		"result: ".blue().bold(),
		"[".white().bold(),
		dynamic_color(final_result, n),
		"]".white().bold());
	}

	pub fn dynamic_color(roll: u64, max: u64) -> String {
		if roll == 1 { // nat 1
			return roll.to_string().red().dimmed().bold().italic().to_string()
		}
		else if roll <= max / 3 { // bottom third
			return roll.to_string().red().bold().to_string()
		}
		else if roll <= max * 2 / 3 { // middle third
			return roll.to_string().yellow().bold().to_string()
		}
		else if roll == max { // nat whatever the max is
			return format!("{}{}", roll.to_string().green().underline().bold(), "!".green().bold())
		}
		else if roll >= max * 2 / 3 { // top third
			return roll.to_string().green().bold().to_string()
		}
		else { // anything else: leave unformatted; I don't anticipate this ever happening
			return roll.to_string()
		}
	}
}