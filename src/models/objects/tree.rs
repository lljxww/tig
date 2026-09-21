use std::{collections::HashMap, path::Path};

use anyhow::bail;

use crate::{
    models::objects::{blob::Blob, get_content_from_raw, get_object_path, tig_object::TigObject},
    utils::ignore_util::matches_ignore,
};

pub struct TreeEntry {
    mode: String,
    name: String,
    hash: [u8; 20],
}

impl TreeEntry {
    pub fn new(mode: String, name: String, hash: [u8; 20]) -> Self {
        Self { mode, name, hash }
    }
}

pub struct Tree {
    content: Vec<u8>,
    entires: Vec<TreeEntry>,
}

impl Tree {
    pub fn new<P>(dir: P, ignore_rules: &[String]) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let mut entires = vec![];

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let name = entry.file_name();

            let s = name.to_str().unwrap_or_default();
            if !s.is_empty() && matches_ignore(s, ignore_rules) {
                continue;
            }

            let name = name
                .into_string()
                .map_err(|_| anyhow::anyhow!("不是合法的UTF-8数据"))?;

            if path.is_dir() {
                let tree = Self::new(&path, ignore_rules)?;
                tree.store()?;
                let tree_hash = tree.hash()?;

                let array = hex::decode(&tree_hash)?;
                let arr: [u8; 20] = array
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("长度必须是 20"))?;

                entires.push(TreeEntry::new("040000".to_string(), name, arr));
            } else {
                let blob = Blob::from_file(&path)?;
                blob.store()?;

                let array = hex::decode(&blob.hash()?)?;
                let arr: [u8; 20] = array
                    .try_into()
                    .map_err(|_| anyhow::anyhow!("长度必须是 20"))?;

                //TODO 其他mode处理
                entires.push(TreeEntry::new("100644".to_string(), name, arr));
            }
        }

        entires.sort_by(|l, r| l.name.cmp(&r.name));

        let mut entires_contents = vec![];

        for tree_entry in &entires {
            let name = tree_entry.name.clone();

            entires_contents
                .extend_from_slice(format!("{} {}\0", tree_entry.mode, name).as_bytes());
            // 20位
            entires_contents.extend_from_slice(&tree_entry.hash);
        }

        anyhow::Ok(Self {
            content: entires_contents,
            entires,
        })
    }

    pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let raw = std::fs::read(&path)?;
        let content = get_content_from_raw(&raw)?;
        let mut entires = Vec::new();
        let mut pos = 0;

        while pos < content.len() {
            let space = content[pos..]
                .iter()
                .position(|&b| b == b' ')
                .ok_or_else(|| anyhow::anyhow!("invalid tree: missing space"))?
                + pos;

            let mode = std::str::from_utf8(&content[pos..space])?.to_owned();
            pos = space + 1;

            let nul = content[pos..]
                .iter()
                .position(|&b| b == 0)
                .ok_or_else(|| anyhow::anyhow!("invalid tree: missing NUL"))?
                + pos;

            let name = std::str::from_utf8(&content[pos..nul])?.to_owned();
            pos = nul + 1;
            let hash: [u8; 20] = content[pos..pos + 20].try_into()?;
            pos += 20;

            entires.push(TreeEntry::new(mode, name, hash));
        }

        anyhow::Ok(Self { content, entires })
    }

    pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
        let object_path = get_object_path(hash);
        anyhow::Ok(Self::from_file(object_path)?)
    }

    pub fn list_files(hash: &str) -> anyhow::Result<HashMap<String, String>> {
        let files = Self::parse_content(hash, None)?;
        anyhow::Ok(files)
    }

    fn parse_content(
        tree_hash: &str,
        path_prefix: Option<&str>,
    ) -> anyhow::Result<HashMap<String, String>> {
        let tree = Self::from_hash(tree_hash)?;

        let mut files: HashMap<String, String> = HashMap::new();

        tree.entires.iter().try_for_each(|entry| {
            let mode = entry.mode.as_str();

            if mode == "040000" {
                let tree_hash = &hex::encode(entry.hash);

                let sub_prefix = match path_prefix {
                    Some(p) => format!("{}/{}", p, entry.name),
                    None => entry.name.clone(),
                };

                let sub_contents = Self::parse_content(tree_hash, Some(&sub_prefix))?;
                files.extend(sub_contents);
            } else if mode == "100644" {
                let hash = hex::encode(entry.hash);

                let mut name = entry.name.clone();

                if let Some(path_prefix) = path_prefix {
                    name = Path::new(path_prefix)
                        .join(name)
                        .to_str()
                        .ok_or_else(|| anyhow::anyhow!("路径不是合法 UTF-8"))?
                        .to_string();
                }

                files.insert(name, hash);
            } else {
                bail!("错误的tree object文件")
            }

            Ok(())
        })?;

        anyhow::Ok(files)
    }
}

impl TigObject for Tree {
    fn object_type(&self) -> &'static str {
        "tree"
    }

    fn content(&self) -> anyhow::Result<Vec<u8>> {
        anyhow::Ok(self.content.clone())
    }
}
