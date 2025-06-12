#[allow(unused_imports)]
use std::io::{self, Write};
use std::{env, path::PathBuf};

// const SHELL_COMMANDS: [&'static str; 3] = ["echo", "type", "exit"];

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
    let length_args = args.len();

    if length_args > 1 {
        println!("Too many arguments.");
    }

    match args[0] {
        "exit" | "echo" | "type" => println!("{} is a shell builtin", args[0]),
        val => {
            if let Some(path) = find_file_in_path(val) {
                println!("{} is {}", val, path.display());
            } else {
                println!("{}: not found", val);
            }
        } // _ => println!("{} not found", args[0]),
    }
}

fn find_file_in_path(file_name: &str) -> Option<PathBuf> {
    let path_env = env::var("PATH").ok()?;

    for path_component in env::split_paths(&path_env) {
        let full_path = path_component.join(file_name);
        if full_path.is_file() {
            return Some(full_path);
        }
    }
    None
}

/*
 * When you come back to continue use this for reference and help
 * https://forum.codecrafters.io/t/i-have-a-problem-with-mg5/6946
 */
