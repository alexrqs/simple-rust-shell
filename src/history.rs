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
        // Remove the old instance of the command, if it exists
        self.commands.retain(|c| c != command);

        // Add the command to the end of the history
        self.commands.push(command.to_string());
    }

    pub fn show(&self) {
        for (i, command) in self.commands.iter().enumerate() {
            println!("{}: {}", i, command);
        }
    }
}
