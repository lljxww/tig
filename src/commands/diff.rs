use std::collections::HashMap;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{blob::Blob, commit::Commit, tree::Tree},
    utils::{
        diff_util::{DiffLine, diff_lines},
        ignore_util::load_ignore,
        ptl_util::{Color, print_colored},
        repo_util::get_last_commit_hash,
        working_dir_util::scan_working_dir,
    },
};

pub struct Diff {}

impl Diff {
    pub fn new() -> Self {
        Diff {}
    }
}

impl TigCommand for Diff {
    fn get_name(&self) -> &'static str {
        "diff"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        // 读上次 commit
        let Some(commit_hash) = get_last_commit_hash()? else {
            println!("(no commits yet)");
            return anyhow::Ok(());
        };
        let commit = Commit::from_hash(&commit_hash)?;

        // 展开 committed tree：path -> committed_blob_hash
        let committed = Tree::list_files(commit.tree_hash())?;

        // 扫描工作区：path -> working_blob_hash（不写磁盘）
        let ignore_rules = load_ignore();
        let mut working: HashMap<String, String> = HashMap::new();
        scan_working_dir("./", "", &ignore_rules, &mut working)?;

        // 只处理两边都有但 hash 不同的文件（modified）
        // 以及只在 committed 有的（deleted）
        // 以及只在 working 有的（new file）
        let mut all_paths: Vec<&String> = committed.keys().chain(working.keys()).collect();
        all_paths.sort();
        all_paths.dedup();

        for path in all_paths {
            let committed_hash = committed.get(path);
            let working_hash = working.get(path);

            match (committed_hash, working_hash) {
                (Some(c_hash), Some(w_hash)) if c_hash == w_hash => {
                    // 未修改，跳过
                }
                (Some(c_hash), Some(_)) => {
                    // modified
                    print_file_diff(path, Some(c_hash), Some(path))?;
                }
                (None, Some(_)) => {
                    // new file
                    print_file_diff(path, None, Some(path))?;
                }
                (Some(c_hash), None) => {
                    // deleted
                    print_file_diff(path, Some(c_hash), None)?;
                }
                (None, None) => unreachable!(),
            }
        }

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}

/// 打印单个文件的 diff。
///
/// - `old_hash`: committed blob 的 hash，None 表示新文件
/// - `working_path`: 工作区路径，None 表示已删除
fn print_file_diff(
    path: &str,
    old_hash: Option<&str>,
    working_path: Option<&str>,
) -> anyhow::Result<()> {
    println!("--- a/{}", path);
    println!("+++ b/{}", path);

    let old_text = match old_hash {
        Some(hash) => match Blob::from_hash(hash)?.text() {
            Ok(t) => t,
            Err(_) => {
                println!("Binary file: {}", path);
                return anyhow::Ok(());
            }
        },
        None => String::new(),
    };

    let new_text = match working_path {
        Some(p) => match String::from_utf8(std::fs::read(format!("./{}", p))?) {
            Ok(t) => t,
            Err(_) => {
                println!("Binary file: {}", path);
                return anyhow::Ok(());
            }
        },
        None => String::new(),
    };

    let old_lines: Vec<&str> = old_text.lines().collect();
    let new_lines: Vec<&str> = new_text.lines().collect();

    let diff = diff_lines(&old_lines, &new_lines);

    for line in diff {
        match line {
            DiffLine::Context(l) => println!(" {}", l),
            DiffLine::Added(l) => print_colored(format!("+{}", l).as_str(), Color::Green),
            DiffLine::Removed(l) => print_colored(format!("-{}", l).as_str(), Color::Red),
        }
    }

    anyhow::Ok(())
}
