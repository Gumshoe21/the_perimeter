use std::io::{self, Write};

pub struct Command {
    pub verb: String,
    pub noun: String,
}

impl Command {
    pub fn new() -> Command {
        Command {
            verb: String::new(),
            noun: String::new(),
        }
    }

    fn parse(&mut self, input_str: &str) {
        // trim - Returns string slice with leading and trailing whitespace removed.
        // split_whitespace - splits string slice by whitespace
        let mut split_input_iter = input_str.trim().split_whitespace();

        self.verb = split_input_iter.next().unwrap_or_default().to_string();
        self.noun = split_input_iter.next().unwrap_or_default().to_string();
    }
}

pub fn get_input() -> Command {
    println!("");
    print!("> ");
    // unwrap called b/c flush returns a Result, which throws error if unhandled
    io::stdout().flush().unwrap();

    // String variable that holds the user entered string.
    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read move.");
    println!("");

    let mut command = Command::new();
    command.parse(input_str.as_str());

    command
}

pub fn update_state(command: &Command) -> String {
    let output: String;

    match command.verb.as_str() {
        "quit" => output = format!("Goodbye."),
        "look" => output = format!("You are in a forest."),
        "go" => output = format!("You don't know where to go so you don't move."),
        _ => output = format!("I'm not sure how to ''."),
    }
    output
}

pub fn update_screen(output: String) {
    println!("{}", output);
}
