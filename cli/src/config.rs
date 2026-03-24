use anyhow::{Context, Result};
use std::path::PathBuf;

pub struct TstackConfig {
    pub tstack_root: PathBuf,
    pub claude_dir: PathBuf,
    pub plugin_active: bool,
}

impl TstackConfig {
    pub fn detect() -> Result<Self> {
        let home_dir = dirs::home_dir().context("Could not determine home directory")?;
        let claude_dir = home_dir.join(".claude");
        let cwd = std::env::current_dir().ok();

        // TSTACK_ROOT: env var > VERSION in cwd/exe-parent > plugin symlink > ~/tstack
        let (tstack_root, plugin_active) = if let Ok(root) = std::env::var("TSTACK_ROOT") {
            let is_plugin = Self::check_plugin_at(&cwd, &PathBuf::from(&root));
            (PathBuf::from(root), is_plugin)
        } else {
            // 1. Check binary parent and cwd for VERSION file (running from tstack repo)
            let exe_parent = std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().and_then(|p| p.parent().map(|p| p.to_path_buf())));

            let version_candidates = [exe_parent, cwd.clone()];

            if let Some(root) = version_candidates.into_iter().flatten().find(|p| p.join("VERSION").exists()) {
                // If cwd IS the tstack root, treat as active (we're in the source repo)
                let cwd_is_root = cwd.as_ref()
                    .and_then(|c| std::fs::canonicalize(c).ok())
                    .and_then(|c| std::fs::canonicalize(&root).ok().map(|r| c == r))
                    .unwrap_or(false);
                let is_plugin = cwd_is_root || Self::check_plugin_at(&cwd, &root);
                (root, is_plugin)
            } else {
                // 2. Check for plugin symlink in project's .claude/plugins/tstack
                let plugin_path = cwd.as_ref().map(|p| p.join(".claude/plugins/tstack"));
                if let Some(resolved) = plugin_path
                    .filter(|p| p.exists())
                    .and_then(|p| std::fs::canonicalize(&p).ok())
                    .filter(|r| r.join("VERSION").exists())
                {
                    (resolved, true)
                } else {
                    // 3. Fallback to ~/tstack
                    let fallback = home_dir.join("tstack");
                    (fallback, false)
                }
            }
        };

        Ok(Self {
            tstack_root,
            claude_dir,
            plugin_active,
        })
    }

    /// Check if a plugin symlink exists in the project pointing to tstack_root
    fn check_plugin_at(cwd: &Option<PathBuf>, tstack_root: &PathBuf) -> bool {
        if let Some(cwd) = cwd {
            let plugin_path = cwd.join(".claude/plugins/tstack");
            if plugin_path.exists() {
                if let (Ok(resolved), Ok(root)) = (
                    std::fs::canonicalize(&plugin_path),
                    std::fs::canonicalize(tstack_root),
                ) {
                    return resolved == root;
                }
            }
        }
        false
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
