pub mod command_functions {
	use std::{thread, time};
	use std::io::Write;
	use rand::Rng;
	use colored::Colorize;
	// Roll a number between 1 and n, passed in as args[1]
	pub fn rolln(args: Vec<String>) {
		assert!(args.len() > 1); // We need to make sure that our vector is big enough
		let n_string = &args[1]; 
		let n = n_string.parse().unwrap();
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
			"[".white().dimmed(),
			dynamic_color(temp_result, n), // pass n as max
			"]".white().dimmed());
			std::io::stdout().flush().expect("couldn't flush the display");
			thread::sleep(time::Duration::from_millis(i * 30));
			i += 1;
		}
		let final_result = rand::thread_rng().gen_range(1..=n); // generate random integer between 1 and n
		println!("{} {} {} {}                 ",
		"result: ".blue().bold(),
		"[".white().bold(),
		dynamic_color(final_result, n),
		"]".white().bold());
	}

	// A clone of rolln that has less delay. It's to be used in multi rolling and advantage cases.
	// Since it is only used internally, it doesn't need to parse arguments or be public.
	// Since it is used internally by other functions, it has to return the rolled number.
	// during debug it is public, but I'll change that for release.
	pub fn quickroll(n: u64) -> u64 {
		print!("{}\r", "rolling...".black());
		std::io::stdout().flush().expect("couldn't flush the display");
		thread::sleep(time::Duration::from_millis(50));
		let mut target: u64 = rand::thread_rng().gen_range((n / 5)..=n);
		if target > 30 { // cap the number of re-rolls at 30 so we can safely roll a d100
			target = 30;
		}
		let mut i :u64 = 0;
		while i <= target {
			let temp_result = rand::thread_rng().gen_range(1..=n);
			print!("{} {} {} {}           \r",
			"rolling...".black(),
			"[".white().dimmed(),
			dynamic_color(temp_result, n), // pass n as max
			"]".white().dimmed());
			std::io::stdout().flush().expect("couldn't flush the display");
			thread::sleep(time::Duration::from_millis(i * 10));
			i += 1;
		}
		let final_result = rand::thread_rng().gen_range(1..=n); // generate random integer between 1 and n
		println!("{} {} {} {}                 ",
		"sub-result: ".black().bold(),
		"[".white().dimmed(),
		dynamic_color(final_result, n).dimmed(),
		"]".white().dimmed());
		final_result // return
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

	pub fn clear() {
		std::process::Command::new("clear").status().expect("couldn't");
	}

	pub fn display_help() {
		println!("{0:^13}|   {1:^20}   ", "Command".bold().magenta(), "Purpose".bold().magenta());
		println!("{0:^13}|", "");
		println!("{0:^13}|  clears the screen.", "clear".bold().green());
		println!("{0:^13}|", "");
		println!("{0:^13}|  exits the program. \"{1}\" will also work.", "exit".bold().green(), "quit".bold().green());
		println!("{0:^13}|", "");
		println!("{0:^13}|  displays this incredibly helpful screen.", "help".bold().green());
		println!("{0:^13}|", "");
		println!("{0:>8} {1:<4}|  rolls a die with {1} sides.
{2:^13}|     If no arguments are provided (i.e. you type \"roll\\n\"),
{2:^13}|     it defaults to a d20.", "roll".bold().green(), "n".bold().blue(), "");

	}

}