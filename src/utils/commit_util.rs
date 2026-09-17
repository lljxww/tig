use crate::models::objects::{commit::Commit, tig_object::TigObject};

pub fn build_commit(
    tree_hash: &str,
    parent_hash: Option<&str>,
    message: &str,
) -> anyhow::Result<String> {
    let commit = Commit::new(
        tree_hash.to_string(),
        parent_hash.map(str::to_owned),
        message.to_string(),
    )?;

    commit.store()?;
    anyhow::Ok(commit.hash()?)
}
