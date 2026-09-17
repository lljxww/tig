use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{get_content_from_raw, get_object_raw_by_hash},
};

pub struct CatFile {
    hash: String,
}

impl CatFile {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(hash) = args.next() else {
            bail!("请指定文件hash");
        };

        if hash.len() != 40 {
            bail!("给定的hash格式不正确");
        }

        anyhow::Ok(Self { hash })
    }
}

impl TigCommand for CatFile {
    fn get_name(&self) -> &'static str {
        "cat-file"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let content = get_object_raw_by_hash(&self.hash)?;
        let raw = get_content_from_raw(&content)?;

        print!("{}", str::from_utf8(&raw)?);

        Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
