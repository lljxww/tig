use std::path::{Path, PathBuf};

use anyhow::bail;

use crate::utils::zlib_util::decode;

pub mod blob;
pub mod commit;
pub mod tig_object;
pub mod tree;

pub fn get_object_path(hash: &str) -> PathBuf {
    Path::new("./.tig/objects")
        .join(&hash[..2])
        .join(&hash[2..])
}

pub fn get_content_from_raw(raw: &[u8]) -> anyhow::Result<Vec<u8>> {
    let raw = decode(raw)?;

    if !is_valid_object_file(&raw) {
        bail!("目标不是支持的tig Object对象");
    }

    let pos = raw
        .iter()
        .position(|&b| b == b'\0')
        .ok_or_else(|| anyhow::anyhow!("错误的对象: 缺少空字节"))?;

    let content = &raw[pos + 1..];

    anyhow::Ok(content.to_vec())
}

// 允许的object文件类型
const VALID_TYPES: &[&[u8]] = &[b"blob", b"tree", b"commit"];

pub fn is_valid_object_file(content: &[u8]) -> bool {
    VALID_TYPES.iter().any(|t| content.starts_with(t))
}
