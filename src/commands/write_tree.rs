use std::path::Path;

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{tig_object::TigObject, tree::Tree},
    utils::ignore_util::load_ignore,
};

pub struct WriteTree {}

impl WriteTree {
    pub fn new() -> anyhow::Result<Self> {
        anyhow::Ok(Self {})
    }
}

impl TigCommand for WriteTree {
    fn get_name(&self) -> &'static str {
        "write-tree"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        // 检查tig存储库是否存在
        if !std::fs::exists(Path::new("./.tig/HEAD"))? {
            bail!("当前文件夹不是有效的tig存储库");
        }

        let ignore_rules = load_ignore();

        let tree = Tree::new("./", &ignore_rules)?;
        tree.store()?;

        println!("{}", tree.hash()?);
        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
