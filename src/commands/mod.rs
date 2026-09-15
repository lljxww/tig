pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod log;
pub mod tig_command;
pub mod write_tree;

pub fn get_hash(blob: &[u8]) -> String {
    sha1_smol::Sha1::from(blob).hexdigest()
}
