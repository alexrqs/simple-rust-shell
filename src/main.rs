mod commands {
    pub mod wow;
}
mod history;

use colored::*;
use commands::wow;
use history::CommandHistory;
use rustyline::error::ReadlineError;
use rustyline::{ DefaultEditor, Result };

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut history = CommandHistory::new();

    loop {
        let readline = rl.readline("# ".magenta().bold().to_string().as_str());
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                let line = line.trim();
                history.add(&line);

                match line {
                    "wow" => wow::run(),
                    "history" => history.show(),
                    "exit" => {
                        break;
                    }
                    _ => println!("Unknown command: {}", line),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())
}
