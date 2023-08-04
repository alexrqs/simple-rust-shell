use std::collections::HashMap;
use std::io::{ self, Write };
use std::process::Command;
use colored::*;

// Define a type for a function that takes no arguments and returns nothing.
type CommandFn = fn();

fn main() {
    let mut commands: HashMap<&str, CommandFn> = HashMap::new();

    // Insert the "wow" command into the map.
    commands.insert("wow", wow_command);

    loop {
        print!("{} ", "#".magenta().bold());

        io::stdout().flush().unwrap(); // Print the '>' character immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        // Check if the command exists in the map.
        match commands.get(command) {
            Some(&function) => function(),
            None => {
                let output = Command::new(command)
                    .args(args)
                    .output()
                    .unwrap_or_else(|e| panic!("failed to execute process: {}", e));

                if !output.stdout.is_empty() {
                    println!("{}", String::from_utf8_lossy(&output.stdout));
                }

                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
        }
    }
}

fn wow_command() {
    println!("This is a command in simple rust shell, wow!");
}
