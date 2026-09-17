use std::{env::Args, iter::Skip, path::Path};

use anyhow::bail;

use crate::commands::tig_command::TigCommand;
use crate::models::objects::blob::Blob;
use crate::models::objects::tig_object::TigObject;

pub struct HashObject {
    blob: Blob,
}

impl HashObject {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(target_file) = args.next() else {
            bail!("请为hash-object命令指定目标文件");
        };

        if !std::fs::exists(&target_file)? {
            bail!("找不到指定的文件: {}", target_file.as_str());
        }

        let blob = Blob::from_file(&target_file)?;

        anyhow::Ok(Self { blob })
    }
}

impl TigCommand for HashObject {
    fn get_name(&self) -> &'static str {
        "hash-object"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        // 检查tig存储库是否存在
        if !std::fs::exists(Path::new("./.tig/HEAD"))? {
            bail!("当前文件夹不是有效的tig存储库");
        }

        println!("hash: {}", self.blob.hash()?);

        self.blob.store()?;

        Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        let hash = self.blob.hash()?;
        let directory_name = hash.split_at(2).0;
        let file_name = hash.split_at(2).1;

        let dir_path = Path::new("./.tig/objects").join(directory_name);
        let file_path = dir_path.join(file_name);

        if std::fs::exists(&file_path)? {
            std::fs::remove_file(file_path)?;
            if dir_path.read_dir()?.next().is_none() {
                std::fs::remove_dir(dir_path)?;
            }
        }

        anyhow::Ok(())
    }
}
