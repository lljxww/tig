use crate::{models::objects::get_object_path, utils::zlib_util::encode};

pub trait TigObject {
    /// 获取对象类型
    fn object_type(&self) -> &'static str;

    /// 当前对象的原始内容(非对象文件的原始内容)
    fn content(&self) -> anyhow::Result<Vec<u8>>;

    /// 获取当前对象的hash
    fn hash(&self) -> anyhow::Result<String> {
        Ok(sha1_smol::Sha1::from(self.content()?).hexdigest())
    }

    /// 获取可写入当前对象文件的原始内容
    fn raw(&self) -> anyhow::Result<Vec<u8>> {
        let content = self.content()?;
        let mut raw = format!("{} {}\0", self.object_type(), content.len()).into_bytes();

        raw.extend_from_slice(&content);
        Ok(raw)
    }

    /// 将当前对象存储到object file
    fn store(&self) -> anyhow::Result<()> {
        let hash = self.hash()?;
        let hash_file_path = get_object_path(&hash);
        let compressed = encode(&self.raw()?)?;
        std::fs::write(hash_file_path, compressed)?;

        Ok(())
    }
}
