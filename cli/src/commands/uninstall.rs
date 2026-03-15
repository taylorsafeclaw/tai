use anyhow::Result;
use crate::config::TstackConfig;
use crate::symlink;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;

    ui::heading("tstack uninstall");

    let mut removed = 0;

    // Remove command symlinks
    for dir_name in ["commands", "agents"] {
        let dir = config.claude_dir.join(dir_name);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if name.starts_with("tstack-") && name.ends_with(".md") && path.is_symlink() {
                    if symlink::remove(&path)? {
                        ui::error(&format!("{dir_name}/{name}"));
                        removed += 1;
                    }
                }
            }
        }
    }

    // Remove skill symlinks
    let skills_dir = config.claude_skills_dir();
    if let Ok(entries) = std::fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if name.starts_with("tstack-") && path.is_symlink() {
                if symlink::remove(&path)? {
                    ui::error(&format!("skills/{name}"));
                    removed += 1;
                }
            }
        }
    }

    println!();
    println!("  Removed {removed} symlinks.");
    println!("  Project-level .claude/ files are untouched.");
    println!();

    Ok(())
}
