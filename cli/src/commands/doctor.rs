use std::io::Write;
use anyhow::Result;
use crate::config::TstackConfig;
use crate::frontmatter::Frontmatter;
use crate::types::*;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;

    cliclack::intro("tstack doctor")?;

    // Check symlinks
    ui::separator("Symlinks");
    println!();

    let commands = scan_md_items(
        &config.commands_dir(),
        &config.claude_commands_dir(),
        ItemType::Command,
        config.plugin_active,
    );
    let agents = scan_md_items(
        &config.agents_dir(),
        &config.claude_agents_dir(),
        ItemType::Agent,
        config.plugin_active,
    );
    let skills = scan_skills(&config);

    check_items("commands", &commands);
    check_items("agents", &agents);
    check_items("skills", &skills);

    // Report broken/conflicting
    let all_items: Vec<&TstackItem> =
        commands.iter().chain(agents.iter()).chain(skills.iter()).collect();
    let broken: Vec<&&TstackItem> = all_items
        .iter()
        .filter(|i| matches!(i.status, LinkStatus::Broken))
        .collect();
    let conflicts: Vec<&&TstackItem> = all_items
        .iter()
        .filter(|i| matches!(i.status, LinkStatus::Conflict(_)))
        .collect();
    let missing: Vec<&&TstackItem> = all_items
        .iter()
        .filter(|i| matches!(i.status, LinkStatus::Missing))
        .collect();

    if !broken.is_empty() {
        for item in &broken {
            ui::error(&format!("{} → (target missing)", item.name));
        }
    }
    if !conflicts.is_empty() {
        for item in &conflicts {
            if let LinkStatus::Conflict(reason) = &item.status {
                ui::warn(&format!("{} — {}", item.name, reason));
            }
        }
    }
    if !missing.is_empty() {
        let mut stdout = std::io::stdout();
        write!(stdout, "  ").ok();
        ui::write_rgb(&mut stdout, &format!("{}", missing.len()), ui::YELLOW);
        ui::write_rgb(&mut stdout, " items not linked ", ui::DIM);
        ui::write_rgb(&mut stdout, "(run `tstack install`)", ui::DIM);
        writeln!(stdout).ok();
    }

    println!();

    // Check frontmatter
    ui::separator("Frontmatter");
    println!();

    let mut valid = 0;
    let mut warnings = Vec::new();

    for item in &all_items {
        let fm_path = if item.source_path.is_dir() {
            item.source_path.join("SKILL.md")
        } else {
            item.source_path.clone()
        };

        match Frontmatter::from_file(&fm_path) {
            Ok(fm) => {
                valid += 1;
                if fm.description.as_deref().unwrap_or("").is_empty() {
                    warnings.push(format!("{} missing description", fm_path.display()));
                }
            }
            Err(e) => {
                warnings.push(format!("{}: {e}", fm_path.display()));
            }
        }
    }

    ui::success(&format!("{valid} files       valid YAML"));
    for w in &warnings {
        ui::warn(w);
    }

    println!();

    // Check hooks
    ui::separator("Hooks");
    println!();

    let hooks = scan_hooks(&config);
    if hooks.is_empty() {
        ui::info("No hooks found.");
    } else {
        let mut stdout = std::io::stdout();
        write!(stdout, "  ").ok();
        ui::write_rgb(&mut stdout, "○ ", ui::DIM);
        ui::write_rgb(&mut stdout, &format!("{} available", hooks.len()), ui::WHITE);
        ui::write_rgb(&mut stdout, "    0 configured in settings.json", ui::DIM);
        writeln!(stdout).ok();
    }

    println!();

    // Check templates
    ui::separator("Templates");
    println!();

    let templates_dir = config.templates_dir();
    if templates_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(&templates_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .to_string();
                    let agent_count = count_files(&path.join("agents"), "*.md");
                    let skill_count = count_dirs_with_skill(&path.join("skills"));
                    let cmd_count = count_files(&path.join("commands"), "*.md");
                    ui::success(&format!(
                        "{name:<16} {agent_count} agents, {skill_count} skills, {cmd_count} commands"
                    ));
                }
            }
        }
    } else {
        ui::info("No templates directory found.");
    }

    println!();

    cliclack::outro("Diagnostics complete.")?;

    Ok(())
}

fn check_items(label: &str, items: &[TstackItem]) {
    let healthy = items.iter().filter(|i| i.status.is_healthy()).count();
    let total = items.len();

    if healthy == total {
        ui::success(&format!("{total:>2} {label:<12} all healthy"));
    } else {
        let issues = total - healthy;
        ui::warn(&format!("{total:>2} {label:<12} {issues} issue(s)"));
    }
}

fn count_files(dir: &std::path::Path, pattern_prefix: &str) -> usize {
    let suffix = pattern_prefix.split('*').next_back().unwrap_or("");

    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().to_string();
                    name.ends_with(suffix)
                })
                .count()
        })
        .unwrap_or(0)
}

fn count_dirs_with_skill(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_dir() && e.path().join("SKILL.md").exists())
                .count()
        })
        .unwrap_or(0)
}
