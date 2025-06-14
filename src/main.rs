#[allow(unused_imports)]
use std::io::{self, Write};
use std::{process::Command as StdCommand, str::FromStr};

use codecrafters_shell::{
    command::Command as CommandDispatch, command::CommandExtract, find_cmd_in_path, parse_input,
    read_path_env,
};

fn main() -> Result<(), anyhow::Error> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("$ ");
    stdout.flush().unwrap();

    let path = read_path_env();

    loop {
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parsed_input = parse_input(input);
        let parsed_input = parsed_input.iter().map(|s| s.as_ref()).collect::<Vec<_>>();

        let cmd = parsed_input
            .first()
            .and_then(|cmd| CommandDispatch::from_str(cmd).ok());

        if let Some(cmd) = cmd {
            cmd.execute(&parsed_input, &path)?;
        } else {
            let cmd = parsed_input.first().unwrap();
            let path_of_cmd = find_cmd_in_path(cmd, &path);

            if path_of_cmd.is_some() {
                let mut command = StdCommand::new(cmd);

                if parsed_input.len() > 1 {
                    command.args(&parsed_input[1..]);
                }

                let output = command.output().expect("failed to execute process");
                print!(
                    "{}",
                    String::from_utf8(output.stdout).expect("Invalid output utf8")
                );
            } else {
                println!("{}: command not found", cmd);
            }
        }

        print!("$ ");
        stdout.flush().unwrap();
    }
}
