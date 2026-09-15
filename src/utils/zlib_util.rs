use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use std::io::{Read, Write};

pub fn encode(content: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    anyhow::Ok(encoder.finish()?)
}

pub fn decode(content: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(content);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    anyhow::Ok(decompressed)
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

        anyhow::Ok(())
    }
}
