use anyhow::Result;
use crate::config::TstackConfig;
use crate::symlink;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;

    cliclack::intro("tstack uninstall")?;

    // Count what will be removed
    let mut targets = Vec::new();
    for dir_name in ["commands", "agents"] {
        let dir = config.claude_dir.join(dir_name);
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                if name.ends_with(".md")
                    && path.is_symlink()
                    && points_to_tstack(&path, &config.tstack_root)
                {
                    targets.push((path, format!("{dir_name}/{name}")));
                }
            }
        }
    }
    let skills_dir = config.claude_skills_dir();
    if let Ok(entries) = std::fs::read_dir(&skills_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
            if path.is_symlink() && points_to_tstack(&path, &config.tstack_root) {
                targets.push((path, format!("skills/{name}")));
            }
        }
    }

    if targets.is_empty() {
        cliclack::outro("No tstack symlinks found.")?;
        return Ok(());
    }

    let should_proceed: bool = cliclack::confirm(format!(
        "Remove {} symlinks from ~/.claude?",
        targets.len()
    ))
    .interact()?;

    if !should_proceed {
        cliclack::outro_cancel("Cancelled.")?;
        return Ok(());
    }

    let spin = cliclack::spinner();
    spin.start("Removing symlinks...");

    let mut removed = 0;
    for (path, _label) in &targets {
        if symlink::remove(path)? {
            removed += 1;
        }
    }

    spin.stop(format!("{removed} symlinks removed"));

    cliclack::log::info("Project-level .claude/ files are untouched.")?;
    cliclack::outro("Uninstall complete.")?;

    Ok(())
}

fn points_to_tstack(symlink_path: &std::path::Path, tstack_root: &std::path::Path) -> bool {
    let Ok(target) = std::fs::read_link(symlink_path) else {
        return false;
    };
    let target_canonical = std::fs::canonicalize(&target).unwrap_or(target);
    let root_canonical =
        std::fs::canonicalize(tstack_root).unwrap_or_else(|_| tstack_root.to_path_buf());
    target_canonical.starts_with(&root_canonical)
}
