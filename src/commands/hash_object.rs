use std::{env::Args, io::Read, iter::Skip, path::Path};

use anyhow::bail;

use crate::commands::get_hash;
use crate::commands::tig_command::TigCommand;
use crate::utils::blob_util::get_blob;
use crate::utils::fs_util::save_to_file;

pub struct HashObject {
    content: Vec<u8>,
    directory_name: String,
    file_name: String,
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
            directory_name: hash.split_at(2).0.to_string(),
            file_name: hash.split_at(2).1.to_string(),
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
        save_to_file(&self.directory_name, &self.file_name, &blob)?;

        Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        let dir_path = Path::new("./.tig/objects").join(&self.directory_name);
        let file_path = dir_path.join(&self.file_name);

        if std::fs::exists(&file_path)? {
            std::fs::remove_file(file_path)?;
            if dir_path.read_dir()?.next().is_none() {
                std::fs::remove_dir(dir_path)?;
            }
        }

        anyhow::Ok(())
    }
}
