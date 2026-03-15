use anyhow::Result;
use std::time::Instant;
use crate::config::TstackConfig;
use crate::symlink;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;
    let start = Instant::now();

    ui::heading("tstack install");

    // Ensure target directories exist
    std::fs::create_dir_all(config.claude_commands_dir())?;
    std::fs::create_dir_all(config.claude_agents_dir())?;
    std::fs::create_dir_all(config.claude_skills_dir())?;

    // Symlink commands
    let cmd_count = link_md_files(&config.commands_dir(), &config.claude_commands_dir(), "commands")?;

    // Symlink agents
    let agent_count = link_md_files(&config.agents_dir(), &config.claude_agents_dir(), "agents")?;

    // Symlink skills (directory-level)
    let skill_count = link_skill_dirs(&config.skills_dir(), &config.claude_skills_dir())?;

    println!();
    ui::success(&format!("commands   {cmd_count} linked"));
    ui::success(&format!("agents     {agent_count} linked"));
    ui::success(&format!("skills     {skill_count} linked"));

    let elapsed = start.elapsed();
    println!();
    ui::info(&format!("Done in {}ms", elapsed.as_millis()));
    println!();

    Ok(())
}

fn link_md_files(source_dir: &std::path::Path, target_dir: &std::path::Path, label: &str) -> Result<usize> {
    let mut count = 0;

    let entries = match std::fs::read_dir(source_dir) {
        Ok(e) => e,
        Err(_) => return Ok(0),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        if !name.starts_with("tstack-") || !name.ends_with(".md") {
            continue;
        }

        let dest = target_dir.join(&name);
        match symlink::create(&path, &dest)? {
            true => count += 1,
            false => {
                ui::warn(&format!("SKIP {label}/{name} (non-symlink file exists)"));
            }
        }
    }

    Ok(count)
}

fn link_skill_dirs(source_dir: &std::path::Path, target_dir: &std::path::Path) -> Result<usize> {
    let mut count = 0;

    let entries = match std::fs::read_dir(source_dir) {
        Ok(e) => e,
        Err(_) => return Ok(0),
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        if !name.starts_with("tstack-") {
            continue;
        }

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
