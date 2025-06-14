mod echo;
mod exit;
mod pwd;
mod r#type;

use crate::algebra::CommandExt;

use crate::command::exit::Exit;
use crate::command::{echo::Echo, pwd::Pwd, r#type::Type};
use enum_dispatch::enum_dispatch;
use strum_macros::EnumString;

#[derive(Debug, Clone, EnumString, strum_macros::Display)]
#[strum(serialize_all = "lowercase")]
#[enum_dispatch(CommandExt)]
pub enum Command {
    Exit(Exit),
    Echo(Echo),
    Pwd(Pwd),
    Type(Type),
}
