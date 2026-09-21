use std::{collections::HashMap, path::Path};

use crate::{
    models::objects::{blob::Blob, tig_object::TigObject},
    utils::ignore_util::matches_ignore,
};

// 扫描当前工作区
pub fn scan_working_dir<P>(
    dir: P,
    prefix: &str,
    ignore_rules: &[String],
    result: &mut HashMap<String, String>,
) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().to_string();
        if matches_ignore(&name, ignore_rules) {
            continue;
        }

        let path = entry.path();
        let rel = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{}/{}", prefix, name)
        };

        if path.is_dir() {
            scan_working_dir(&path, &rel, ignore_rules, result)?;
        } else {
            let blob = Blob::from_file(&path)?;
            result.insert(rel, blob.hash()?);
        }
    }

    anyhow::Ok(())
}
