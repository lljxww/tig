use std::{env::Args, io::Read, iter::Skip, path::Path};

use anyhow::bail;

use crate::commands::get_hash;
use crate::commands::tig_command::TigCommand;
use crate::utils::blob_util::get_blob;
use crate::utils::object_util::zlib_and_save_to_file;

pub struct HashObject {
    content: Vec<u8>,
    hash: String,
}

impl HashObject {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(target_file) = args.next() else {
            bail!("请为hash-object命令指定目标文件");
        };

        if !std::fs::exists(&target_file)? {
            bail!("找不到指定的文件: {}", target_file.as_str());
        }

        let mut target_file = std::fs::File::open(target_file)?;

        let mut file_content: Vec<u8> = Vec::new();
        target_file.read_to_end(&mut file_content)?;

        let hash = get_hash(&get_blob(&file_content));

        anyhow::Ok(Self {
            content: file_content,
            hash,
        })
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

        println!("hash: {}", self.hash);

        let blob = get_blob(&self.content);
        zlib_and_save_to_file(&self.hash, &blob)?;

        Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        let directory_name = &self.hash.split_at(2).0;
        let file_name = &self.hash.split_at(2).1;

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
