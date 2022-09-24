// use std::io;
mod command_line;
use command_line::command::Command;

fn main() {
    let test_command = Command {
        keyword: String::from("bagels"),
        callback: test_function,
    };
    // let new_test_command = Command::new(new_test_command, String::from("bagels"), test_function);
    let mut args : Vec<String> = Vec::new();
    args.push(String::from("bagels"));
    test_command.verify(args);
}

fn test_function() {
    println!("bagels? Where?!");
}