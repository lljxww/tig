use std::{env::Args, iter::Skip};

use crate::commands::{
    cat_file::CatFile, commit_tree::CommitTree, config::Config, hash_object::HashObject,
    init::Init, log::Log, tig_command::TigCommand, write_tree::WriteTree,
};

mod commands;
mod models;
mod utils;

fn main() {
    let args = std::env::args().skip(1);

    if args.len() == 0 {
        show_help();
        std::process::exit(0);
    }

    let mut command = parse_command(args).unwrap_or_else(|err| {
        eprintln!("操作失败, 原因: {}", err);
        std::process::exit(1);
    });

    if let Err(err) = command.exec() {
        eprintln!(
            "命令: {} 执行失败, 原因: {}, 正在回滚...",
            command.get_name(),
            err
        );

        if let Err(err) = command.rollback() {
            eprintln!("{} 回滚失败, 原因: {}", command.get_name(), err);
        }
    }
}

fn show_help() {
    println!("tig: a simple Git implementation written in Rust.");
}

fn parse_command(mut args: Skip<Args>) -> anyhow::Result<Box<dyn TigCommand>> {
    let name = args.next().unwrap_or_else(|| {
        show_help();
        std::process::exit(1);
    });

    let command: Box<dyn TigCommand> = match name.trim().to_lowercase().as_str() {
        "init" => Box::new(Init::new()),
        "log" => Box::new(Log::new()),
        "hash-object" => Box::new(HashObject::new(args)?),
        "cat-file" => Box::new(CatFile::new(args)?),
        "write-tree" => Box::new(WriteTree::new()?),
        "commit-tree" => Box::new(CommitTree::new(args)?),
        "config" => Box::new(Config::new(args)?),
        _ => {
            show_help();
            std::process::exit(1);
        }
    };

    anyhow::Ok(command)
}
