use std::path::Path;

use crate::utils::zlib_util::encode;

pub fn save_to_file(directory_name: &str, file_name: &str, content: &[u8]) -> anyhow::Result<()> {
    let dir_path = Path::new("./.tig/objects").join(directory_name);
    std::fs::create_dir_all(&dir_path)?;

    let compressed = encode(content)?;
    std::fs::write(dir_path.join(file_name), compressed)?;

    anyhow::Ok(())
}
