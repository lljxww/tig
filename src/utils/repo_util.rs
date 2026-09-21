use std::path::Path;

use anyhow::bail;

fn get_head_content() -> anyhow::Result<String> {
    anyhow::Ok(std::fs::read_to_string("./.tig/HEAD")?)
}

pub fn get_current_branch_name() -> anyhow::Result<String> {
    let head = get_head_content()?;

    let ref_path = head
        .strip_prefix("ref: ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("存储库HEAD出现错误"))?;

    let branch_name = ref_path
        .rsplit_once('/')
        .map(|(_, name)| name)
        .ok_or_else(|| anyhow::anyhow!("HEAD引用路径格式错误"))?;

    Ok(branch_name.to_owned())
}

pub fn get_branch_file_path() -> anyhow::Result<String> {
    let head = get_head_content()?;

    let ref_path = head
        .strip_prefix("ref: ")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("存储库HEAD出现错误"))?;

    let branch_file = format!(".tig/{}", ref_path);

    anyhow::Ok(branch_file)
}

pub fn save_branch(name: &str, commit_hash: String) -> anyhow::Result<()> {
    std::fs::write(Path::new("./.tig/refs/heads").join(name), commit_hash)?;
    anyhow::Ok(())
}

pub fn get_last_commit_hash() -> anyhow::Result<Option<String>> {
    let branch_file = get_branch_file_path()?;

    let parent = std::fs::read_to_string(&branch_file)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    anyhow::Ok(parent)
}

pub fn get_all_branches() -> anyhow::Result<Vec<String>> {
    let branch_files = std::fs::read_dir("./.tig/refs/heads")
        .map_err(|e| anyhow::anyhow!("branch文件读取错误: {}", e))?;

    let files = branch_files
        .into_iter()
        .map(|entry| {
            let entry = entry?;
            Ok(entry.file_name().to_string_lossy().into_owned())
        })
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    anyhow::Ok(files)
}

#[allow(dead_code)]
pub fn set_head_to_branch(branch_name: &str) -> anyhow::Result<()> {
    let branches = get_all_branches()?;

    if !branches.iter().any(|b| b == branch_name) {
        bail!("不存在分支: {}", branch_name);
    }

    std::fs::write("./.tig/HEAD", format!("ref: refs/heads/{}\n", branch_name))?;
    anyhow::Ok(())
}
