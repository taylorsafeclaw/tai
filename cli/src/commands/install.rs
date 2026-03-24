use anyhow::Result;
use std::time::Instant;
use crate::config::TstackConfig;
use crate::symlink;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;
    let start = Instant::now();

    cliclack::intro("tstack install")?;

    // Ensure target directories exist
    std::fs::create_dir_all(config.claude_commands_dir())?;
    std::fs::create_dir_all(config.claude_agents_dir())?;
    std::fs::create_dir_all(config.claude_skills_dir())?;

    // Link commands
    let spin = cliclack::spinner();
    spin.start("Linking commands...");
    let cmd_count = link_md_files(&config.commands_dir(), &config.claude_commands_dir())?;
    spin.stop(format!("Commands   {} linked", cmd_count));

    // Link agents
    let spin = cliclack::spinner();
    spin.start("Linking agents...");
    let agent_count = link_md_files(&config.agents_dir(), &config.claude_agents_dir())?;
    spin.stop(format!("Agents     {} linked", agent_count));

    // Link skills
    let spin = cliclack::spinner();
    spin.start("Linking skills...");
    let skill_count = link_skill_dirs(&config.skills_dir(), &config.claude_skills_dir())?;
    spin.stop(format!("Skills     {} linked", skill_count));

    let elapsed = start.elapsed();
    cliclack::outro(format!("Done in {}ms", elapsed.as_millis()))?;

    Ok(())
}

fn link_md_files(source_dir: &std::path::Path, target_dir: &std::path::Path) -> Result<usize> {
    let mut count = 0;
    link_md_files_recursive(source_dir, target_dir, &mut count)?;
    Ok(count)
}

fn link_md_files_recursive(
    current_dir: &std::path::Path,
    target_dir: &std::path::Path,
    count: &mut usize,
) -> Result<()> {
    let entries = match std::fs::read_dir(current_dir) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries.flatten() {
        let path = entry.path();

        if path.is_dir() && !path.is_symlink() {
            link_md_files_recursive(&path, target_dir, count)?;
            continue;
        }

        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        if !name.ends_with(".md") {
            continue;
        }

        let dest = target_dir.join(&name);
        match symlink::create(&path, &dest)? {
            true => *count += 1,
            false => {
                ui::warn(&format!("SKIP {name} (non-symlink file exists)"));
            }
        }
    }

    Ok(())
}

fn link_skill_dirs(
    source_dir: &std::path::Path,
    target_dir: &std::path::Path,
) -> Result<usize> {
    let mut count = 0;

    let entries = match std::fs::read_dir(source_dir) {
        Ok(e) => e,
        Err(_) => return Ok(0),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() || !path.join("SKILL.md").exists() {
            continue;
        }

        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();
        let dest = target_dir.join(&name);
        match symlink::create(&path, &dest)? {
            true => count += 1,
            false => {
                ui::warn(&format!("SKIP skills/{name} (non-symlink directory exists)"));
            }
        }
    }

    Ok(count)
}
