use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{commit::Commit, tree::Tree},
    utils::repo_util::{get_all_branches, get_branch_commit_hash, set_head_to_branch},
};

pub struct Checkout {
    branch_name: String,
}

impl Checkout {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        if let Some(branch_name) = args.next() {
            anyhow::Ok(Checkout { branch_name })
        } else {
            bail!("请指定要checkout的branch名")
        }
    }
}

impl TigCommand for Checkout {
    fn get_name(&self) -> &'static str {
        "checkout"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let branches = get_all_branches()?;
        if !branches.iter().any(|b| b == &self.branch_name) {
            bail!("目标branch名不存在");
        }

        let branch_commit_hash = get_branch_commit_hash(&self.branch_name)?;
        let commit = Commit::from_hash(&branch_commit_hash)?;

        Tree::restore_to_dir(commit.tree_hash(), "./")?;
        set_head_to_branch(&self.branch_name)?;

        println!("Switched to branch '{}'", self.branch_name);

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
