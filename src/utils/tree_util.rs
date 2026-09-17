use std::path::Path;

use crate::models::objects::{tig_object::TigObject, tree::Tree};

pub fn write_tree(dir: &Path, ignore_rules: &[String]) -> anyhow::Result<String> {
    let tree = Tree::new(dir, ignore_rules)?;
    tree.store()?;

    anyhow::Ok(tree.hash()?)
}
