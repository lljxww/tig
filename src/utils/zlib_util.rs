use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};

/// 使用zlib对内容进行压缩
pub fn encode(content: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish()?)
}

/// 解压zlib压缩内容
pub fn decode(content: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(content);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

#[cfg(test)]
mod tests {
    use crate::utils::zlib_util::{decode, encode};

    #[test]
    fn encode_and_decode() -> anyhow::Result<()> {
        let blob = String::from("abcdefg").into_bytes();

        let encoded = encode(&blob)?;
        let decoded = decode(&encoded)?;

        assert_eq!(blob, decoded);

        Ok(())
    }
}
