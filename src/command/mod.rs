mod cd;
mod echo;
mod exit;
mod pwd;
mod r#type;

use crate::command::{cd::Cd, echo::Echo, exit::Exit, pwd::Pwd, r#type::Type};
use enum_dispatch::enum_dispatch;
use strum_macros::EnumString;

#[enum_dispatch]
pub trait CommandExtract {
    fn execute(&self, args: &[&str], path: &[String]) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(CommandExtract)]
pub enum Command {
    Cd(Cd),
    Exit(Exit),
    Echo(Echo),
    Pwd(Pwd),
    Type(Type),
}
