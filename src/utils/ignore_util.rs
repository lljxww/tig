pub fn load_ignore() -> Vec<String> {
    let mut rules: Vec<String> = std::fs::read_to_string(".tigignore")
        .unwrap_or_default() // 文件不存在时当空字符串处理
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .map(String::from)
        .collect();

    rules.push(".tig".to_string());

    rules
}

pub fn matches_ignore(name: &str, rules: &[String]) -> bool {
    rules.iter().any(|rule| {
        if let Some(suffix) = rule.strip_prefix("*.") {
            name.ends_with(suffix)
        } else {
            name == rule
        }
    })
}
