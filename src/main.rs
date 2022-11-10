mod cli;
mod lines_codec;

use std::thread;

use crate::cli::command_line::CommandLine;
use crate::cli::command::{ArgCommand, Command};
use crate::cli::command_functions::{rolln, clear, display_help, welcome};

use crate::lines_codec::server::start_corrosion_server;

fn main() {
    // Spawns the server process and begins listening for inputs on port 3333.
    thread::spawn(move || start_corrosion_server(Some("127.0.0.1:4000")));

     // clear the display initially
    std::process::Command::new("clear").status().expect("couldn't");
    let cl = setup_command_line();
    cl.input_cycle();
    // debug();
}

// hooks up functions and commands
fn setup_command_line() -> CommandLine {
    // instantiate the command line
    let mut cl = CommandLine {
        command_list: Vec::new(),
        arg_command_list: Vec::new(),
        prompt: String::from(" > "),
    };
    // instantiate the commands
    let mut roll_n_default_args = Vec::new();
    roll_n_default_args.push(String::from("DEFAULT:")); // so default_args[1] is the value we want
    roll_n_default_args.push(String::from("20"));
    let roll_command = ArgCommand {
        keyword: String::from("roll"),
        callback: rolln,
        default_arguments: roll_n_default_args,
    };
    let clear_command = Command {
        keyword: String::from("clear"),
        callback: clear,
    };
    let help_command = Command {
        keyword: String::from("help"),
        callback: display_help,
    };
    // connect them
    cl.arg_command_list.push(roll_command);
    cl.command_list.push(clear_command);
    cl.command_list.push(help_command);
    // welcome the user
    welcome();
    cl
}