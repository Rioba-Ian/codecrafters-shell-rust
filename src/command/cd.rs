use std::{env, io, path::Path};

use crate::command::CommandExtract;

#[derive(Debug, Clone, Default)]
pub struct Cd;

impl CommandExtract for Cd {
    fn execute(&self, args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        if args.len() > 1 {
            match change_directory(args[1]) {
                Ok(_) => {}
                Err(_) => println!("cd: {}: No such file or directory", args[1]),
            }
        }
        Ok(())
    }
}

fn change_directory(path: &str) -> io::Result<()> {
    // check if it starts with instead
    if path == "~" {
        let home_path = env::var("HOME").expect("could not get home variable");
        env::set_current_dir(home_path).expect("failed to set home var path");
    } else {
        let absolute_path = Path::new(path);
        env::set_current_dir(absolute_path)?;
    }
    Ok(())
}
