use crate::commands::tig_command::TigCommand;

pub struct Init {
    paths: Vec<String>,
}

impl Init {
    pub fn new() -> Self {
        let mut paths = vec![
            "objects".to_string(),
            "refs/heads".to_string(),
            "refs/tags".to_string(),
        ];

        paths.extend((0..=255).map(|i| format!("objects/{i:02x}")));

        Self { paths }
    }
}

impl TigCommand for Init {
    fn get_name(&self) -> &'static str {
        "init"
    }

    fn exec(&mut self) -> anyhow::Result<()> {
        let tig_path = std::env::current_dir()?.join(".tig");

        if tig_path.exists() {
            println!("当前目录已经初始化tig仓库");
            return anyhow::Ok(());
        }

        for path in &self.paths {
            std::fs::create_dir_all(tig_path.join(path))?;
        }

        std::fs::write(tig_path.join("HEAD"), "ref: refs/heads/main\n")?;

        println!("已初始化tig仓库");

        anyhow::Ok(())
    }

    fn rollback(&mut self) -> anyhow::Result<()> {
        let tig_path_buf = std::env::current_dir()?.join(".tig");
        std::fs::remove_dir_all(tig_path_buf)?;
        anyhow::Ok(())
    }
}
