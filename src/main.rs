#[allow(unused_imports)]
use std::io::{self, Write};

const SHELL_COMMANDS: [&'static str; 3] = ["echo", "type", "exit"];

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let trimmed_input = input.trim();

        match trimmed_input
            .split_whitespace()
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["exit"] => break,
            ["exit", code] => std::process::exit(code.parse().unwrap()),
            ["echo", args @ ..] => cmd_echo(args),
            ["type", args @ ..] => cmd_type(args),
            [other] => println!("{}: command not found", other),
            _ => println!("unknown command"),
        }

        input.clear();
    }
}

fn cmd_echo(args: &[&str]) {
    println!("{}", args.join(" "))
}

fn cmd_type(args: &[&str]) {
    for item in args {
        if SHELL_COMMANDS.contains(item) {
            println!("{} is a shell builtin", item)
        } else {
            println!("{} not found", item)
        }
    }
}
