pub trait TigCommand {
    fn get_name(&self) -> &'static str;

    fn exec(&mut self) -> anyhow::Result<()>;

    fn rollback(&mut self) -> anyhow::Result<()>;
}
