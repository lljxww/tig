use std::path::Path;

use crate::models::objects::{get_content_from_raw, get_object_path, tig_object::TigObject};

pub struct Blob {
    content: Vec<u8>,
}

impl Blob {
    /// 创建Blob对象
    pub fn new(content: Vec<u8>) -> Self {
        Self { content }
    }

    /// 从blob文件创建blob对象
    pub fn from_file<P>(path: P) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
    {
        let content = std::fs::read(&path)?;
        Ok(Self::new(content))
    }

    /// 从blob hash创建commit对象
    pub fn from_hash(hash: &str) -> anyhow::Result<Self> {
        let path = get_object_path(hash);
        let raw = std::fs::read(path)?;
        let content = get_content_from_raw(&raw)?;
        Ok(Self::new(content))
    }

    /// 获取获取当前blob对象存储的String内容
    pub fn text(&self) -> anyhow::Result<String> {
        Ok(String::from_utf8(self.content.clone())?)
    }
}

impl TigObject for Blob {
    fn object_type(&self) -> &'static str {
        "blob"
    }

    fn content(&self) -> anyhow::Result<Vec<u8>> {
        Ok(self.content.clone())
    }
}
