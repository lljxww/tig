use std::path::Path;

use crate::utils::zlib_util::encode;

pub fn zlib_and_save_to_file(hash: &str, content: &[u8]) -> anyhow::Result<()> {
    let (directory_name, file_name) = get_directory_name_and_file_name(hash);

    let dir_path = Path::new("./.tig/objects").join(directory_name);
    std::fs::create_dir_all(&dir_path)?;

    let compressed = encode(content)?;
    std::fs::write(dir_path.join(file_name), compressed)?;

    anyhow::Ok(())
}

pub fn is_object_file_exist(hash: &str) -> anyhow::Result<bool> {
    let (directory_name, file_name) = get_directory_name_and_file_name(hash);
    anyhow::Ok(std::fs::exists(
        Path::new("./.tig/objects")
            .join(directory_name)
            .join(file_name),
    )?)
}

fn get_directory_name_and_file_name(hash: &str) -> (&str, &str) {
    let directory_name = hash.split_at(2).0;
    let file_name = hash.split_at(2).1;

    (directory_name, file_name)
}

// 允许的object文件类型
const VALID_TYPES: &[&[u8]] = &[b"blob", b"tree", b"commit"];

pub fn is_valid_object_file(content: &[u8]) -> bool {
    VALID_TYPES.iter().any(|t| content.starts_with(t))
}
