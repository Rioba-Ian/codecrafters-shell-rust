use crate::command::CommandExtract;

#[derive(Debug, Clone, Default)]
pub struct Pwd;

impl CommandExtract for Pwd {
    fn execute(&self, _args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;

        println!("{}", current_dir.display());

        Ok(())
    }
}
