use std::path::Path;

use crate::{
    commands::get_hash,
    utils::{blob_util::get_blob, ignore_util::matches_ignore, object_util::zlib_and_save_to_file},
};

pub fn write_tree(dir: &Path, ignore_rules: &[String]) -> anyhow::Result<String> {
    let mut entires = vec![];

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();

        let s = name.to_str().unwrap_or_default();
        if !s.is_empty() && matches_ignore(s, ignore_rules) {
            continue;
        }

        if path.is_dir() {
            let tree_hash = write_tree(&path, ignore_rules)?;
            entires.push((name, "040000", tree_hash));
        } else {
            let content = std::fs::read(&path)?;
            let blob = get_blob(&content);
            let hash = get_hash(&blob);
            zlib_and_save_to_file(&hash, &blob)?;
            //TODO 其他mode处理
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
        // 20位
        entires_contents.extend_from_slice(&hex::decode(&hash)?);
    }

    // 构造tree文件头
    let mut object = format!("tree {}\0", entires_contents.len()).into_bytes();
    object.extend_from_slice(&entires_contents);

    let hash = get_hash(&object);
    zlib_and_save_to_file(&hash, &object)?;

    anyhow::Ok(hash)
}
