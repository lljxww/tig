use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{commands::tig_command::TigCommand, utils::commit_util::build_commit};

pub struct CommitTree {
    tree_hash: String,
    message: String,
    parent_hash: Option<String>,
}

impl CommitTree {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(tree_hash) = args.next() else {
            bail!("请为commit-tree命令指定目标tree hash");
        };

        let mut message: Option<String> = None;
        let mut parent: Option<String> = None;

        while let Some(key) = args.next() {
            match key.as_str() {
                "-m" => {
                    message = args.next();
                }
                "-p" => {
                    parent = args.next();
                }
                _ => {
                    bail!("不支持的命令: {}", key);
                }
            }
        }

        if message.is_none() {
            bail!("commit-tree时, 必须使用-m指定提交信息");
        }

        let message = message.unwrap();
        if message.trim().is_empty() {
            bail!("提交信息不能为空");
        }

        anyhow::Ok(Self {
            tree_hash,
            message,
            parent_hash: parent,
        })
    }
}

impl TigCommand for CommitTree {
    fn get_name(&self) -> &'static str {
        "commit-tree"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let hash = build_commit(&self.tree_hash, self.parent_hash.as_deref(), &self.message)?;
        println!("{}", hash);

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
