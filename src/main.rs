#[allow(unused_imports)]
use std::io::{self, Write};
use std::{
    fs::File,
    process::{Command as StdCommand, Stdio},
    str::FromStr,
};

use codecrafters_shell::{
    append_to_file,
    command::{Command as CommandDispatch, CommandExtract},
    find_cmd_in_path, parse_input, read_path_env,
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

        let mut path_redirect_output = Vec::new();
        let mut args_before_redirect = Vec::new();
        let mut output_err = false;
        let mut append_output = false;

        for (index, arg) in parsed_input.iter().enumerate() {
            if *arg == ">" || *arg == "1>" || *arg == "2>" || arg.ends_with(">>") {
                match *arg {
                    "2>" => output_err = true,
                    ">>" | "1>>" => append_output = true,
                    _ => {}
                }

                path_redirect_output = parsed_input[index + 1..].to_vec();
                args_before_redirect = parsed_input[1..index].to_vec();
                break;
            }
        }

        if !path_redirect_output.is_empty() {
            let output_file = File::create(path_redirect_output.join(" ")).unwrap();
            let cmd = parsed_input.first().unwrap();
            let mut command = StdCommand::new(cmd);

            command.args(args_before_redirect);

            if output_err {
                let output_err_file = File::create(path_redirect_output.join(" ")).unwrap();
                command.stderr(Stdio::from(output_err_file));
                command.spawn()?.wait()?;
            } else if append_output {
                let ouput = command.output().expect("failed to execute command");

                append_to_file(
                    path_redirect_output.join(" ").as_str(),
                    ouput.stdout.to_owned(),
                )
                .expect("failed to write to file");
            } else {
                command.stdout(Stdio::from(output_file));
                command.spawn()?.wait()?;
            }
        } else if let Some(cmd) = cmd {
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
