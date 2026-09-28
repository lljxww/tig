use std::path::{Path, PathBuf};

use anyhow::bail;
use ini::Ini;

/// 初始化tig的全局配置文件
pub fn init_config_file() -> anyhow::Result<()> {
    let Some(home) = dirs::home_dir() else {
        bail!("用户目录获取失败!")
    };

    let config_file_path = home.join(".tigconfig");
    if !std::fs::exists(&config_file_path)? {
        _ = std::fs::File::create_new(config_file_path);
    }

    Ok(())
}

/// 获取tig的配置文件所在目录
fn get_config_path() -> anyhow::Result<PathBuf> {
    let Some(home) = dirs::home_dir() else {
        bail!("用户目录获取失败!")
    };

    Ok(Path::new(&home).join(".tigconfig"))
}

/// 设置指定的设置项并存储
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

    Ok(())
}

/// 读取指定的设置项
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

    Ok(value)
}

/// 读取设置中的作者信息
pub fn get_author() -> anyhow::Result<String> {
    let author = config_get("user.name")?;

    if author.is_empty() {
        bail!("请设置tig用户: tig config set user.name REPLACE_WITH_YOUR_NAME");
    }

    Ok(author)
}

/// 读取设置中的作者邮件信息
pub fn get_email() -> anyhow::Result<String> {
    let author = config_get("user.email")?;

    if author.is_empty() {
        bail!("请设置tig用户: tig config set user.email REPLACE_WITH_YOUR_EMAIL");
    }

    Ok(author)
}
