use std::path::Path;

use crate::{
    commands::get_hash,
    utils::{blob_util::get_blob, fs_util::save_to_file, ignore_util::matches_ignore},
};

pub fn write_tree(dir: &Path, ignore_rules: &[String]) -> anyhow::Result<String> {
    let mut entires = vec![];

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();

        if name == ".tig" {
            continue;
        }

        if path.is_dir() {
            let s = name.to_str().unwrap_or_default();
            if !s.is_empty() && matches_ignore(s, ignore_rules) {
                continue;
            }

            let tree_hash = write_tree(&path, ignore_rules)?;
            entires.push((name, "040000", tree_hash));
        } else {
            let content = std::fs::read(&path)?;
            let blob = get_blob(&content);
            let hash = get_hash(&blob);
            save_to_file(hash.split_at(2).0, hash.split_at(2).1, &blob)?;
            entires.push((name, "100644", hash));
        }
    }

    entires.sort_by(|(l_name, _, _), (r_name, _, _)| l_name.cmp(r_name));

    let mut entires_contents = vec![];

    for (name, mode, hash) in entires {
        let name = name
            .into_string()
            .map_err(|_| anyhow::anyhow!("不是合法的UTF-8文件"))?;

        entires_contents.extend_from_slice(format!("{} {}\0", mode, name).as_bytes());
        entires_contents.extend_from_slice(&hex::decode(&hash)?);
    }

    let mut object = format!("tree {}\0", entires_contents.len()).into_bytes();
    object.extend_from_slice(&entires_contents);

    let hash = get_hash(&object);
    save_to_file(hash.split_at(2).0, hash.split_at(2).1, &object)?;

    anyhow::Ok(hash)
}
