use std::collections::HashMap;

use crate::{
    commands::tig_command::TigCommand,
    models::objects::{commit::Commit, tree::Tree},
    utils::{
        ignore_util::load_ignore, repo_util::get_last_commit_hash,
        working_dir_util::scan_working_dir,
    },
};

pub struct Status {}

impl Status {
    pub fn new() -> Self {
        Status {}
    }
}

impl TigCommand for Status {
    fn get_name(&self) -> &'static str {
        "status"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        // 读上次的commit
        let Some(commit_hash) = get_last_commit_hash()? else {
            println!("(no commits yet)");
            return anyhow::Ok(());
        };
        let commit = Commit::from_hash(&commit_hash)?;

        // 展开committed tree
        let committed = Tree::list_files(commit.tree_hash())?;

        // 扫描工作区(不写磁盘)
        let ignore_rules = load_ignore();
        let mut working = HashMap::new();

        scan_working_dir("./", "", &ignore_rules, &mut working)?;

        let new_files = working
            .keys()
            .filter(|k| !committed.contains_key(*k))
            .collect::<Vec<_>>();

        new_files.iter().for_each(|f| println!("new file: {}", *f));

        let deleted_files = committed
            .keys()
            .filter(|k| !working.contains_key(*k))
            .collect::<Vec<_>>();

        deleted_files
            .iter()
            .for_each(|f| println!("deleted: {}", *f));

        let modified_files = working.keys().filter(|k| {
            if let Some(working_file) = working.get(*k)
                && let Some(committed_file) = committed.get(*k)
            {
                return working_file != committed_file;
            }

            false
        });

        modified_files.for_each(|f| println!("modified: {}", *f));

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
