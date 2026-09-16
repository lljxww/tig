use std::{
    env::Args,
    iter::Skip,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::bail;

use crate::{
    commands::{get_hash, tig_command::TigCommand},
    utils::{
        config_util::{get_author, get_email},
        object_util::{is_object_file_exist, zlib_and_save_to_file},
    },
};

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
        // 验证tree hash
        if !is_object_file_exist(&self.tree_hash)? {
            bail!("非法的tree hash")
        }

        let name = get_author()?;
        let email = get_email()?;

        let now = SystemTime::now();
        let timestamp = now.duration_since(UNIX_EPOCH)?.as_secs();
        let timezone = chrono::Local::now().format("%z");

        let info = format!("{} <{}> {} {}", name, email, timestamp, timezone);

        let mut commit_str = format!("tree {}\n", self.tree_hash,);

        if !self
            .parent_hash
            .as_deref()
            .is_none_or(|h| h.trim().is_empty())
        {
            commit_str.push_str(&format!(
                "parent {}\n",
                self.parent_hash.as_deref().unwrap()
            ));
        }

        commit_str.push_str(&format!(
            "author {}\ncommitter {}\n\n{}",
            info, info, self.message
        ));

        let commit_bytes = commit_str.as_bytes();

        // 构造commit文件头
        let mut object = format!("commit {}\0", commit_bytes.len()).into_bytes();
        object.extend_from_slice(commit_bytes);

        let hash = get_hash(&object);
        zlib_and_save_to_file(&hash, &object)?;

        println!("{}", hash);

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
