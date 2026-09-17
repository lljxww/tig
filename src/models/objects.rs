use std::path::{Path, PathBuf};

use anyhow::bail;

use crate::utils::{object_util::get_directory_name_and_file_name, zlib_util::decode};

pub mod blob;
pub mod commit;
pub mod tig_object;
pub mod tree;

pub fn get_object_raw_by_hash(hash: &str) -> anyhow::Result<Vec<u8>> {
    let (directory_name, file_name) = get_directory_name_and_file_name(hash);

    let content = std::fs::read(
        Path::new("./.tig/objects")
            .join(directory_name)
            .join(file_name),
    )?;

    anyhow::Ok(content)
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

pub fn object_path(hash: &str) -> PathBuf {
    let (dir, file) = get_directory_name_and_file_name(hash);
    Path::new("./.tig/objects").join(dir).join(file)
}
