use std::{env::Args, io::Read, iter::Skip, path::Path};

use anyhow::bail;

use crate::{commands::tig_command::TigCommand, utils::zlib_util::decode};

pub struct CatFile {
    directory_name: String,
    file_name: String,
}

impl CatFile {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(hash) = args.next() else {
            bail!("请指定文件hash");
        };

        if hash.len() != 40 {
            bail!("给定的hash格式不正确");
        }

        anyhow::Ok(Self {
            directory_name: hash.split_at(2).0.to_string(),
            file_name: hash.split_at(2).1.to_string(),
        })
    }
}

impl TigCommand for CatFile {
    fn get_name(&self) -> &'static str {
        "cat-file"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let directory_path = Path::new("./.tig/objects").join(&self.directory_name);
        let file_path = directory_path.join(&self.file_name);

        if !std::fs::exists(&file_path)? {
            bail!("找不到指定的文件");
        }

        let mut target_file = std::fs::File::open(file_path)?;
        let mut blob: Vec<u8> = Vec::new();
        target_file.read_to_end(&mut blob)?;

        let raw_content = decode(&blob)?;

        if !raw_content.starts_with(String::from("blob").as_bytes()) {
            bail!("存储库信息错误, 请考虑重新生成存储库");
        }

        let pos = raw_content
            .iter()
            .position(|&b| b == 0)
            .ok_or_else(|| anyhow::anyhow!("错误的对象: 缺少空字节"))?;

        let content = &raw_content[pos + 1..];

        print!("{}", std::str::from_utf8(content)?);

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
