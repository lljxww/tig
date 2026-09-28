use std::{env::Args, iter::Skip};

use crate::{
    commands::tig_command::TigCommand, models::objects::commit::Commit,
    utils::repo_util::get_last_commit_hash,
};

pub struct Log {
    is_one_line: bool,
}

impl Log {
    pub fn new(args: Skip<Args>) -> Self {
        let mut is_one_line = false;

        for exist_param in args {
            if exist_param.eq("--oneline") {
                is_one_line = true;
                break;
            }
        }

        Self { is_one_line }
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
            return Ok(());
        };

        let mut commit = Commit::from_hash(&hash)?;
        println!("{}", commit.get_print_text(self.is_one_line)?);

        while let Some(parent_hash) = commit.parent_hash() {
            commit = Commit::from_hash(parent_hash)?;
            println!("{}", commit.get_print_text(self.is_one_line)?);
        }

        Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        Ok(())
    }
}
