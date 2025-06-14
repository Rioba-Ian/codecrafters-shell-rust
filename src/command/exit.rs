use crate::command::CommandExtract;

#[derive(Debug, Clone, Default)]
pub struct Exit;

impl CommandExtract for Exit {
    fn execute(&self, _args: &[&str], _path: &[String]) -> anyhow::Result<()> {
        std::process::exit(0)
    }
}
