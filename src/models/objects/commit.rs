use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::bail;

use crate::{
    models::objects::{get_content_from_raw, get_object_path, tig_object::TigObject},
    utils::{
        config_util::{get_author, get_email},
        timestamp_util,
    },
};

pub struct Commit {
    author: String,
    tree_hash: String,
    parent_hash: Option<String>,
    message: String,
}

impl Commit {
    /// 创建一个commit对象
    pub fn new(
        tree_hash: String,
        parent_hash: Option<String>,
        message: String,
    ) -> anyhow::Result<Self> {
        let object_path = get_object_path(&tree_hash);
        if !std::fs::exists(object_path)? {
            bail!("非法的tree hash")
        }

        let name = get_author()?;
        let email = get_email()?;
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
        let timezone = chrono::Local::now().format("%z");
        let author = format!("{} <{}> {} {}", name, email, timestamp, timezone);

        Ok(Self {
            author,
            tree_hash,
            parent_hash,
            message,
        })
    }

    /// 从commit文件创建commit对象
    pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let raw = std::fs::read(path)?;
        let content = get_content_from_raw(&raw)?;

        let text = std::str::from_utf8(&content)?;

        let (headers, message) = text
            .split_once("\n\n")
            .ok_or_else(|| anyhow::anyhow!("错误的 commit 对象: 缺少正文分隔符"))?;

        let mut tree_hash = None;
        let mut author = None;
        let mut parent_hash = None;

        for line in headers.lines() {
            if let Some(value) = line.strip_prefix("tree ") {
                tree_hash = Some(value.to_owned());
            } else if let Some(value) = line.strip_prefix("author ") {
                author = Some(value.to_owned());
            } else if let Some(value) = line.strip_prefix("parent ") {
                parent_hash = Some(value.to_owned());
            }
        }

        let tree_hash =
            tree_hash.ok_or_else(|| anyhow::anyhow!("错误的 commit 对象: 缺少 tree"))?;

        let author = author.ok_or_else(|| anyhow::anyhow!("错误的 commit 对象: 缺少 author"))?;

        Ok(Self {
            author,
            tree_hash,
            parent_hash,
            message: message.to_owned(),
        })
    }

    /// 从commit hash创建commit对象
    pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
        let object_path = get_object_path(hash);
        Self::from_file(object_path)
    }

    /// 获取当前commit的parent hash信息
    pub fn parent_hash(&self) -> Option<&str> {
        self.parent_hash.as_deref()
    }

    /// 获取当前commit的tree hash
    pub fn tree_hash(&self) -> &str {
        &self.tree_hash
    }

    /// 返回当前commit的信息, 适用于log --oneline
    pub fn get_print_text(&self, is_one_line: bool) -> anyhow::Result<String> {
        if is_one_line {
            Self::print_oneline(self)
        } else {
            Self::print_full(self)
        }
    }

    fn print_full(&self) -> anyhow::Result<String> {
        let hash = self.hash()?;

        let Some((author_part, timezone)) = self.author.rsplit_once(' ') else {
            bail!("commit文件不正确");
        };

        let Some((formated_author, timestamp)) = author_part.rsplit_once(' ') else {
            bail!("commit文件不正确");
        };

        let formated_timestamp = timestamp_util::get_log_format(timestamp, timezone)?;

        let result = format!(
            "{} {}\nAuthor: {}\nDate: {}\n\n\t{}\n",
            self.object_type(),
            hash,
            formated_author,
            formated_timestamp,
            self.message
        );

        Ok(result)
    }

    fn print_oneline(&self) -> anyhow::Result<String> {
        let hash = self.hash()?;
        Ok(format!("{} {}\n", &hash[..7], self.message))
    }
}

impl TigObject for Commit {
    fn object_type(&self) -> &'static str {
        "commit"
    }

    fn content(&self) -> anyhow::Result<Vec<u8>> {
        let mut commit_str = format!("tree {}\n", self.tree_hash,);

        if let Some(parent_hash) = &self.parent_hash
            && !parent_hash.trim().is_empty()
        {
            commit_str.push_str(&format!("parent {parent_hash}\n"));
        }

        commit_str.push_str(&format!(
            "author {}\ncommitter {}\n\n{}",
            self.author, self.author, self.message
        ));

        Ok(commit_str.as_bytes().to_owned())
    }
}
