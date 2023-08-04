pub fn run(history: &[String]) {
    for (i, command) in history.iter().enumerate() {
        println!("{}: {}", i, command);
    }
}
