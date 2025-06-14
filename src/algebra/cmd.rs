use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait CommandExt {
    fn execute(&self, args: &[&str], path: &[String]) -> anyhow::Result<()>;
}
