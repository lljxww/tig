use std::path::Path;

use crate::utils::{object_util::get_directory_name_and_file_name, zlib_util::encode};

pub trait TigObject {
    fn object_type(&self) -> &'static str;
    fn content(&self) -> anyhow::Result<Vec<u8>>;

    fn hash(&self) -> anyhow::Result<String> {
        anyhow::Ok(sha1_smol::Sha1::from(self.content()?).hexdigest())
    }

    fn raw(&self) -> anyhow::Result<Vec<u8>> {
        let content = self.content()?;
        let mut raw = format!("{} {}\0", self.object_type(), content.len()).into_bytes();

        raw.extend_from_slice(&content);
        anyhow::Ok(raw)
    }

    fn store(&self) -> anyhow::Result<()> {
        let hash = self.hash()?;
        let (directory_name, file_name) = get_directory_name_and_file_name(&hash);

        let dir_path = Path::new("./.tig/objects").join(directory_name);
        std::fs::create_dir_all(&dir_path)?;

        let compressed = encode(&self.raw()?)?;
        std::fs::write(dir_path.join(file_name), compressed)?;

        anyhow::Ok(())
    }
}
