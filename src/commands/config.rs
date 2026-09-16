use std::{env::Args, iter::Skip};

use anyhow::bail;

use crate::{
    commands::tig_command::TigCommand,
    utils::config_util::{config_get, config_set},
};

pub struct Config {
    mode: String,
    key: String,
    value: Option<String>,
}

impl Config {
    pub fn new(mut args: Skip<Args>) -> anyhow::Result<Self> {
        let Some(mode) = args.next() else {
            bail!("请输入操作: get/set");
        };

        let Some(key) = args.next() else {
            bail!("请输入要操作的配置键");
        };

        anyhow::Ok(Self {
            mode,
            key,
            value: args.next(),
        })
    }
}

impl TigCommand for Config {
    fn get_name(&self) -> &'static str {
        "config"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        match self.mode.trim().to_lowercase().as_str() {
            "get" => {
                let value = config_get(&self.key)?;
                println!("{}", value);
            }
            "set" => {
                if self.value.as_deref().is_none_or(|v| v.trim().is_empty()) {
                    bail!("请为 `{}` 指定值", self.key);
                }

                config_set(&self.key, self.value.as_deref().unwrap())?;

                println!(
                    "已存储设置: {}={}",
                    self.key,
                    self.value.as_deref().unwrap()
                );
            }
            _ => bail!("仅支持get/set"),
        }

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        anyhow::Ok(())
    }
}
