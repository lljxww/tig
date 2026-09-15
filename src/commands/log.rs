use crate::commands::tig_command::TigCommand;

pub struct Log {}

impl Log {
    pub fn new() -> Self {
        Self {}
    }
}

impl TigCommand for Log {
    fn get_name(&self) -> &'static str {
        "log"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        println!("log");

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
