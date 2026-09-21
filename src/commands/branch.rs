use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    utils::{
        ptl_util,
        repo_util::{get_all_branches, get_current_branch_name, get_last_commit_hash, save_branch},
    },
};

pub struct Branch {
    branch_name: Option<String>,
}

impl Branch {
    pub fn new(mut args: Skip<Args>) -> Self {
        Branch {
            branch_name: args.next(),
        }
    }
}

impl TigCommand for Branch {
    fn get_name(&self) -> &'static str {
        "branch"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let branches = get_all_branches()?;

        if let Some(branch_name) = &self.branch_name {
            if branches.iter().any(|b| b == branch_name) {
                bail!("已存在此分支: {}", branch_name);
            }

            let Some(current_commit_hash) = get_last_commit_hash()? else {
                bail!("必须有提交记录后, 才可创建branch")
            };

            save_branch(branch_name, current_commit_hash)?;

            println!("已创建分支: {}", branch_name);
            return anyhow::Ok(());
        } else {
            let current_branch = get_current_branch_name()?;

            for branch in branches {
                if branch == current_branch {
                    ptl_util::print_colored(
                        format!("* {}", branch).as_str(),
                        ptl_util::Color::Green,
                    );
                } else {
                    println!("  {}", branch);
                }
            }
        }

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
