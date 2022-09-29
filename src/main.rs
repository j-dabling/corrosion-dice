// use std::io;
mod command_line;
mod command_functions;
use command_line::command::Command;
use command_line::command_line::CommandLine;
use command_functions::command_functions::{/* rolln,  */roll20, clear};
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
        prompt: String::from("> "),
    };
    // instantiate the commands
    let roll20_command = Command {
        keyword: String::from("roll"),
        callback: roll20,
    };
    let clear_command = Command {
        keyword: String::from("clear"),
        callback: clear,
    };
    // connect them
    cl.command_list.push(roll20_command);
    cl.command_list.push(clear_command);
    cl
}