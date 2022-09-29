// use std::io;
mod command_line;
mod command_functions;
use command_line::command::Command;
use command_functions::command_functions::rolln;
// use command_line::command_line::CommandLine;

fn main() {
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

    rolln(6);
}

/* fn test_function() {
    println!("bagels? Where?!");
} */