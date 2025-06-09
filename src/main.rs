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

        if trimmed_input == "exit" {
            println!("Exiting program.");
            break;
        }

        println!("{}: command not found", trimmed_input);
        input.clear();
    }
}
