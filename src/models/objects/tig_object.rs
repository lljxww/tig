use crate::{models::objects::get_object_path, utils::zlib_util::encode};

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
        let hash_file_path = get_object_path(&hash);
        let compressed = encode(&self.raw()?)?;
        std::fs::write(hash_file_path, compressed)?;

        anyhow::Ok(())
    }
}
