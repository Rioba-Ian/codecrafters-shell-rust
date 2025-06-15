use crate::command::CommandExtract;

#[derive(Debug, Clone, Default)]
pub struct Echo;

impl CommandExtract for Echo {
    fn execute(&self, args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        if args.len() > 1 {
            println!("{}", args[1..].join(" "))
        }
        Ok(())
    }
}
