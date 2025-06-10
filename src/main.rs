#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        match trimmed_input {
            "exit 0" => break,
            "quit" => break,
            _ => parse_input(trimmed_input),
        }

        input.clear();
    }
}

fn parse_input(input: &str) {
    let input_str = input.to_string();

    if input_str.starts_with("echo") {
        let echo_data: Vec<String> = input.split(" ").map(|s| s.to_string()).collect();
        let parsed_data = &echo_data[1..].join(" ");
        println!("{}", parsed_data);
    } else {
        println!("{}: command not found", input)
    }
}
