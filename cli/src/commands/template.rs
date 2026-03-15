use anyhow::{bail, Result};
use crate::cli::TemplateAction;
use crate::config::TstackConfig;
use crate::ui;

pub fn run(action: TemplateAction) -> Result<()> {
    let config = TstackConfig::detect()?;

    match action {
        TemplateAction::Install { name } => install_template(&config, &name)?,
        TemplateAction::List => list_templates(&config)?,
    }

    Ok(())
}

fn install_template(config: &TstackConfig, name: &str) -> Result<()> {
    let template_dir = config.templates_dir().join(name);

    if !template_dir.exists() {
        bail!("Template '{}' not found in {}", name, config.templates_dir().display());
    }

    let install_script = template_dir.join("install");
    if !install_script.exists() {
        bail!("Template '{}' has no install script", name);
    }

    ui::heading(&format!("tstack template install {name}"));

    let status = std::process::Command::new("bash")
        .arg(&install_script)
        .status()?;

    if status.success() {
        println!();
        ui::success(&format!("Template '{name}' installed successfully"));
    } else {
        ui::error(&format!("Template '{name}' install script failed"));
    }

    println!();
    Ok(())
}

fn list_templates(config: &TstackConfig) -> Result<()> {
    ui::heading("tstack templates");

    let templates_dir = config.templates_dir();
    if !templates_dir.exists() {
        ui::info("No templates directory found.");
        println!();
        return Ok(());
    }

    let mut found = false;
    if let Ok(entries) = std::fs::read_dir(&templates_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                found = true;
                let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();
                let has_install = path.join("install").exists();
                let status = if has_install { "ready" } else { "no install script" };

                // Count contents
                let agents = count_in_dir(&path.join("agents"));
                let skills = count_in_dir(&path.join("skills"));
                let commands = count_in_dir(&path.join("commands"));

                use owo_colors::OwoColorize;
                println!("  {}  {}  {}",
                    format!("{name:<16}").bold(),
                    format!("{agents}a {skills}s {commands}c").dimmed(),
                    if has_install {
                        format!("● {status}").to_string()
                    } else {
                        format!("○ {status}").to_string()
                    }
                );
            }
        }
    }

    if !found {
        ui::info("No templates found.");
    }

    println!();
    Ok(())
}

fn count_in_dir(dir: &std::path::Path) -> usize {
    std::fs::read_dir(dir)
        .map(|entries| entries.flatten().count())
        .unwrap_or(0)
}
