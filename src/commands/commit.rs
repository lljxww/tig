use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{tig_object::TigObject, tree::Tree},
    utils::{
        ignore_util,
        repo_util::{get_current_branch_file_path, get_last_commit_hash},
    },
};

pub struct Commit {
    message: String,
}

impl Commit {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        if let Some(tag) = args.next()
            && tag == "-m"
        {
            if let Some(message) = args.next()
                && !message.trim().is_empty()
            {
                anyhow::Ok(Self { message })
            } else {
                bail!("提交信息不能为空")
            }
        } else {
            bail!("使用-m COMMIT_MESSAGE提交")
        }
    }
}

impl TigCommand for Commit {
    fn get_name(&self) -> &'static str {
        "commit"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let ignore_rules = &ignore_util::load_ignore();
        let tree = Tree::new("./", ignore_rules)?;
        tree.store()?;

        let branch_file = get_current_branch_file_path()?;
        let parent = get_last_commit_hash()?;

        let commit = crate::models::objects::commit::Commit::new(
            tree.hash()?,
            parent.as_deref().map(str::to_owned),
            self.message.to_string(),
        )?;

        commit.store()?;

        let commit_hash = commit.hash()?;

        std::fs::write(&branch_file, &commit_hash)?;

        let Some(branch_name) = branch_file.rsplit('/').next() else {
            bail!("存储库数据错误");
        };

        println!(
            "[{} {}] {}",
            branch_name,
            &commit_hash.as_str()[..7],
            self.message,
        );

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
