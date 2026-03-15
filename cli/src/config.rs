use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct TstackConfig {
    pub tstack_root: PathBuf,
    pub claude_dir: PathBuf,
}

impl TstackConfig {
    pub fn detect() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Could not determine home directory")?;
        let claude_dir = home_dir.join(".claude");

        // TSTACK_ROOT: env var > parent of binary > ~/tstack
        let tstack_root = if let Ok(root) = std::env::var("TSTACK_ROOT") {
            PathBuf::from(root)
        } else {
            // Try to find tstack root by looking for VERSION file
            let candidates = [
                std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|p| p.parent().map(|p| p.to_path_buf())).flatten()),
                Some(std::env::current_dir().unwrap_or_default()),
                Some(home_dir.join("tstack")),
            ];

            candidates
                .into_iter()
                .flatten()
                .find(|p| p.join("VERSION").exists())
                .unwrap_or_else(|| home_dir.join("tstack"))
        };

        Ok(Self {
            tstack_root,
            claude_dir,
        })
    }

    pub fn commands_dir(&self) -> PathBuf {
        self.tstack_root.join("commands")
    }

    pub fn agents_dir(&self) -> PathBuf {
        self.tstack_root.join("agents")
    }

    pub fn skills_dir(&self) -> PathBuf {
        self.tstack_root.join("skills")
    }

    pub fn hooks_dir(&self) -> PathBuf {
        self.tstack_root.join("hooks")
    }

    pub fn templates_dir(&self) -> PathBuf {
        self.tstack_root.join("templates")
    }

    pub fn claude_commands_dir(&self) -> PathBuf {
        self.claude_dir.join("commands")
    }

    pub fn claude_agents_dir(&self) -> PathBuf {
        self.claude_dir.join("agents")
    }

    pub fn claude_skills_dir(&self) -> PathBuf {
        self.claude_dir.join("skills")
    }

    pub fn version(&self) -> String {
        std::fs::read_to_string(self.tstack_root.join("VERSION"))
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string()
    }
}
