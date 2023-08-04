mod wow;
mod history;

use history::CommandHistory;
use std::io::{ self, Write };
use colored::*;

fn main() {
    let mut command_history = CommandHistory::new();

    loop {
        print!("{}", "# ".bright_magenta());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => {
                return;
            }
            "wow" => wow::run(),
            "history" => command_history.show(),
            _ => println!("{}: command not found", input),
        }

        if input != "history" {
            command_history.add(input);
        }
    }
}
