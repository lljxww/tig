pub fn get_blob(content: &[u8]) -> Vec<u8> {
    let mut blob = format!("blob {}\0", content.len()).into_bytes();
    blob.extend_from_slice(content);

    blob
}
