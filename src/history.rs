pub struct CommandHistory {
    commands: Vec<String>,
}

impl CommandHistory {
    pub fn new() -> CommandHistory {
        CommandHistory {
            commands: Vec::new(),
        }
    }

    pub fn add(&mut self, command: &str) {
        if let Some(last_command) = self.commands.last() {
            if command == last_command {
                return;
            }
        }
        self.commands.push(command.to_string());
    }

    pub fn show(&self) {
        for (i, command) in self.commands.iter().enumerate() {
            println!("{}: {}", i, command);
        }
    }
}
