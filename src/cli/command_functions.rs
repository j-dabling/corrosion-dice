use std::io;
use std::io::Write;
use std::str::FromStr;
use std::{thread, time};
use std::net::{Ipv4Addr, TcpStream};

use rand::Rng;
use colored::Colorize;

use crate::lines_codec::LinesCodec;

static mut addr: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
static mut connect_to_server: bool = false;

// Roll a number between 1 and n, passed in as args[1]
pub fn rolln(args: Vec<String>) {
	assert!(args.len() > 1); // We need to make sure that our vector is big enough
	let n_string = &args[1];
	
	// If they just type `roll dis` or similar
	if args.len() == 2 {
		if args[1] == String::from("dis") || args[1] == String::from("adv") {
			roll_with_advantage_state(20, args[1].clone());
			return;
		}
	}
	// check if args[1] is a number argument
	// stolen from: https://turreta.com/2019/09/13/rust-how-to-check-if-a-string-is-numeric/
	for char in n_string.chars() {
		if !char.is_numeric() {
			return;
		}
		if char == ' ' {
			return;
		}
	}
	let n = n_string.parse::<u64>().expect(&(format!("{} argument is not a number", "ERROR:".bold().italic().red())));
	if args.len() > 2 {
		if args[2] == String::from("dis") || args[2] == String::from("adv") {
			roll_with_advantage_state(n, args[2].clone());
			return;
		}
	}
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
	let result_msg: String = format!("{} {} {} {}                 ",
		"result: ".blue().bold(),
		"[".white().bold(),
		dynamic_color(final_result, n),
		"]".white().bold());
	println!("{}", result_msg);
	// send_to_server(result_msg);
	let server_msg : String = format!("{{\"name\":\"test\",\"roll\":{},\"n\":{}}}", final_result, n);

	unsafe {
		if connect_to_server {
			send_to_server(&server_msg)
				.expect("Could not send roll result to peers.");
		}
	}
}

// A clone of rolln that has less delay. It's to be used in multi rolling and advantage cases.
// Since it is only used internally, it doesn't need to parse arguments or be public.
// Since it is used internally by other functions, it has to return the rolled number.
fn quickroll(n: u64) -> u64 {
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

pub fn roll_with_advantage_state(size_dice: u64, mut adv_state: String) {
	adv_state = adv_state.to_lowercase();
	assert!(adv_state == String::from("adv") || adv_state == String::from("dis"));
	if adv_state == String::from("adv") {
		let roll_a = quickroll(size_dice);
		let roll_b = quickroll(size_dice);
		let kept_roll;
		if roll_a >= roll_b { kept_roll = roll_a; }
		else { kept_roll = roll_b; }
		println!("{} {}{}{} {} {} {}",
	"result".bold().blue(),
	"(".bold().blue(),
	"advantage".green(),
	"):".bold().blue(),
	"[".bold().white(),
	dynamic_color(kept_roll, size_dice),
	"]".bold().white());
	}
	else if adv_state == String::from("dis") {
		let roll_a = quickroll(size_dice);
		let roll_b = quickroll(size_dice);
		let kept_roll;
		if roll_a <= roll_b { kept_roll = roll_a; }
		else { kept_roll = roll_b; }
		println!("{} {}{}{} {} {} {}",
	"result".bold().blue(),
	"(".bold().blue(),
	"disadvantage".red(),
	"):".bold().blue(),
	"[".bold().white(),
	dynamic_color(kept_roll, size_dice),
	"]".bold().white());
	}
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
	println!("{0:^16}|{1:^23}   ", "Command".bold().magenta(), "Purpose".bold().magenta());
	println!("{0:^16}|", "");
	println!("{0:^16}|  clears the screen.", "clear".bold().green());
	println!("{0:^16}|", "");
	println!("{0:^16}|  exits the program. \"{1}\" will also work.", "exit".bold().green(), "quit".bold().green());
	println!("{0:^16}|", "");
	println!("{0:^16}|  displays this incredibly helpful screen.", "help".bold().green());
	println!("{0:^16}|", "");
	println!("{0} {4} {2:<3} |  The main event: rolls a die (i.e. selects a random number 
		{3:^16}|     between 1 and {1}). Both {1} and {2} are optional. 
		{3:^16}|     {1} represents the size of the die you are rolling,
		{3:^16}|     and {2} represents the advantage state of the roll.
		{3:^16}|     {2} can be either `adv` or `dis`, representing
		{3:^16}|     advantage or disadvantage, respectively.",
		"roll".bold().green(), "[n]".bold().blue(), "[a]".bold().yellow(), "", "[n=20]".bold().blue());
}

pub fn welcome() {
	let mut addr_str = String::new();
	println!("Do you want to connect to another corrosion-dice client? [Enter IP or ENTER to continue without connecting.]");
	println!("ex. 127.0.0.1");
	io::stdin().read_line(&mut addr_str).expect("Could not read input.");
	if let Ok(test_addr) = addr_str.trim().parse::<Ipv4Addr>() {
		unsafe {
			addr = test_addr;
			connect_to_server = true;
		}
	}

	println!("{}\nType {} to get started or {} for more information.", 
	" == Welcome to corrosion-dice! == ".bold().truecolor(150, 150, 150), "roll".green(), "help".green())
}

fn send_to_server(message: &str) -> io::Result<()> {
	let stream = TcpStream::connect("localhost:4000")?;

	// let mut codec = LinesCodec::new(stream)?;
	let mut codec = LinesCodec::new(stream)?;

	// codec.send_message("Hello")?;
	codec.send_message(&message)
		.expect("Could not send message as expected.");

	println!("{}", codec.read_message()?);
	Ok(())
}