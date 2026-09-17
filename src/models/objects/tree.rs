use std::path::Path;

use crate::{
    models::objects::{blob::Blob, tig_object::TigObject},
    utils::{ignore_util::matches_ignore, tree_util::write_tree},
};

pub struct Tree {
    content: Vec<u8>,
}

impl Tree {
    pub fn new(dir: &Path, ignore_rules: &[String]) -> anyhow::Result<Self> {
        let mut entires = vec![];

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();

            let s = name.to_str().unwrap_or_default();
            if !s.is_empty() && matches_ignore(s, ignore_rules) {
                continue;
            }

            if path.is_dir() {
                let tree_hash = write_tree(&path, ignore_rules)?;
                entires.push((name, "040000", tree_hash));
            } else {
                let blob = Blob::from_file(&path)?;
                blob.store()?;

                //TODO 其他mode处理
                entires.push((name, "100644", blob.hash()?));
            }
        }

        entires.sort_by(|(l_name, _, _), (r_name, _, _)| l_name.cmp(r_name));

        let mut entires_contents = vec![];

        for (name, mode, hash) in entires {
            let name = name
                .into_string()
                .map_err(|_| anyhow::anyhow!("不是合法的UTF-8文件"))?;

            entires_contents.extend_from_slice(format!("{} {}\0", mode, name).as_bytes());
            // 20位
            entires_contents.extend_from_slice(&hex::decode(&hash)?);
        }

        anyhow::Ok(Self {
            content: entires_contents,
        })
    }

    // pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    // where
    //     P: AsRef<Path>,
    // {
    //     let raw = std::fs::read(&path)?;
    //     let content = get_content_from_raw(&raw)?;
    //     anyhow::Ok(Self { content })
    // }

    // pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
    //     let object_path = object_path(hash);
    //     anyhow::Ok(Self::from_file(object_path)?)
    // }
}

impl TigObject for Tree {
    fn object_type(&self) -> &'static str {
        "tree"
    }

    fn content(&self) -> anyhow::Result<Vec<u8>> {
        anyhow::Ok(self.content.clone())
    }
}
