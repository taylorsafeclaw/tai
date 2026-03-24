use anyhow::Result;
use crate::config::TstackConfig;
use crate::symlink;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;

    ui::heading("tstack uninstall");

    let mut removed = 0;

    // Remove command/agent symlinks that point into tstack
    for dir_name in ["commands", "agents"] {
        let dir = config.claude_dir.join(dir_name);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if name.ends_with(".md") && path.is_symlink() && points_to_tstack(&path, &config.tstack_root)
                    && symlink::remove(&path)? {
                        ui::error(&format!("{dir_name}/{name}"));
                        removed += 1;
                }
            }
        }
    }

    // Remove skill symlinks that point into tstack
    let skills_dir = config.claude_skills_dir();
    if let Ok(entries) = std::fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if path.is_symlink() && points_to_tstack(&path, &config.tstack_root)
                && symlink::remove(&path)? {
                    ui::error(&format!("skills/{name}"));
                    removed += 1;
            }
        }
    }

    println!();
    println!("  Removed {removed} symlinks.");
    println!("  Project-level .claude/ files are untouched.");
    println!();

    Ok(())
}

/// Check if a symlink target is inside the tstack root directory
fn points_to_tstack(symlink_path: &std::path::Path, tstack_root: &std::path::Path) -> bool {
    let Ok(target) = std::fs::read_link(symlink_path) else { return false };
    let target_canonical = std::fs::canonicalize(&target).unwrap_or(target);
    let root_canonical = std::fs::canonicalize(tstack_root).unwrap_or_else(|_| tstack_root.to_path_buf());
    target_canonical.starts_with(&root_canonical)
}
