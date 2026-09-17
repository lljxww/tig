use std::path::Path;

pub fn is_object_file_exist(hash: &str) -> anyhow::Result<bool> {
    let (directory_name, file_name) = get_directory_name_and_file_name(hash);
    anyhow::Ok(std::fs::exists(
        Path::new("./.tig/objects")
            .join(directory_name)
            .join(file_name),
    )?)
}

pub fn get_directory_name_and_file_name(hash: &str) -> (&str, &str) {
    let directory_name = hash.split_at(2).0;
    let file_name = hash.split_at(2).1;

    (directory_name, file_name)
}

fn get_head_content() -> anyhow::Result<String> {
    anyhow::Ok(std::fs::read_to_string("./.tig/HEAD")?)
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

pub fn get_last_commit_hash() -> anyhow::Result<Option<String>> {
    let branch_file = get_branch_file_path()?;

    let parent = std::fs::read_to_string(&branch_file)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    anyhow::Ok(parent)
}
