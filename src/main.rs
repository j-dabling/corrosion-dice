// use std::io;
mod command_line;
mod command_functions;
use command_line::command::{Command, ArgCommand};
use command_line::command_line::CommandLine;
use command_functions::command_functions::{rolln, clear};
// use command_line::command_line::CommandLine;

fn main() {
    // clear the display initially
    std::process::Command::new("clear").status().expect("couldn't");
    /* let test_command = Command {
        keyword: String::from("bagels"),
        callback: test_function,
    };

    // let new_test_command = Command::new(new_test_command, String::from("bagels"), test_function);
    let mut test_command_list: Vec<Command> = Vec::new(); 
    test_command_list.push(test_command);
    let test_cli = CommandLine {
        command_list: test_command_list,
        prompt: String::from("enter your input: "),
    };
    test_cli.input_cycle(); */
    let cl = setup_command_line();
    cl.input_cycle();
}

/* fn test_function() {
    println!("bagels? Where?!");
} */

// hooks up functions and commands
fn setup_command_line() -> CommandLine {
    // instantiate the command line
    let mut cl = CommandLine {
        command_list: Vec::new(),
        arg_command_list: Vec::new(),
        prompt: String::from("> "),
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
    // connect them
    cl.arg_command_list.push(roll_command);
    cl.command_list.push(clear_command);
    cl
}