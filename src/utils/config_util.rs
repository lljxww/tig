use std::path::{Path, PathBuf};

use anyhow::bail;
use ini::Ini;

pub fn init_config_file() -> anyhow::Result<()> {
    let Some(home) = dirs::home_dir() else {
        bail!("用户目录获取失败!")
    };

    let config_file_path = home.join(".tigconfig");
    if !std::fs::exists(&config_file_path)? {
        _ = std::fs::File::create_new(config_file_path);
    }

    anyhow::Ok(())
}

fn get_config_path() -> anyhow::Result<PathBuf> {
    let Some(home) = dirs::home_dir() else {
        bail!("用户目录获取失败!")
    };

    anyhow::Ok(Path::new(&home).join(".tigconfig"))
}

pub fn config_set(key: &str, value: &str) -> anyhow::Result<()> {
    if !key.contains('.') {
        bail!("未知的设置项: {}", key);
    }

    init_config_file()?;

    let (section, name) = key
        .split_once('.')
        .ok_or_else(|| anyhow::anyhow!("配置项格式错误: {key}"))?;

    let config_path = get_config_path()?;
    let mut config = Ini::load_from_file(&config_path)?;

    //TODO 特定key的值的格式验证, 比如邮件

    config.with_section(Some(section)).set(name, value);
    config.write_to_file(&config_path)?;

    anyhow::Ok(())
}

pub fn config_get(key: &str) -> anyhow::Result<String> {
    if !key.contains('.') {
        bail!("未知的设置项: {}", key);
    }

    init_config_file()?;

    let (section, name) = key
        .split_once('.')
        .ok_or_else(|| anyhow::anyhow!("配置项格式错误: {key}"))?;

    let config_path = get_config_path()?;
    let config = Ini::load_from_file(&config_path)?;

    let value = config
        .section(Some(section))
        .and_then(|section| section.get(name))
        .unwrap_or_default()
        .to_owned();

    anyhow::Ok(value)
}

pub fn get_author() -> anyhow::Result<String> {
    let author = config_get("user.name")?;

    if author.is_empty() {
        bail!("请设置tig用户: tig config set user.name REPLACE_WITH_YOUR_NAME");
    }

    anyhow::Ok(author)
}

pub fn get_email() -> anyhow::Result<String> {
    let author = config_get("user.email")?;

    if author.is_empty() {
        bail!("请设置tig用户: tig config set user.email REPLACE_WITH_YOUR_EMAIL");
    }

    anyhow::Ok(author)
}
