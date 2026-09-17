use crate::{
    commands::tig_command::TigCommand, models::objects::commit::Commit,
    utils::object_util::get_last_commit_hash,
};

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
        let hash = get_last_commit_hash()?;

        let Some(hash) = hash else {
            println!("(no commits yet)");
            return anyhow::Ok(());
        };

        let mut commit = Commit::from_hash(&hash)?;
        println!("{}", commit.get_print_text()?);

        while let Some(parent_hash) = commit.parent_hash() {
            commit = Commit::from_hash(parent_hash)?;
            println!("{}", commit.get_print_text()?);
        }

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
