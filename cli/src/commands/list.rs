use anyhow::Result;
use owo_colors::OwoColorize;
use crate::cli::ListKind;
use crate::config::TstackConfig;
use crate::types::*;
use crate::ui;

pub fn run(kind: ListKind) -> Result<()> {
    let config = TstackConfig::detect()?;

    match kind {
        ListKind::Commands => list_commands(&config)?,
        ListKind::Agents => list_agents(&config)?,
        ListKind::Skills => list_skills(&config)?,
        ListKind::Hooks => list_hooks(&config)?,
        ListKind::All => {
            list_commands(&config)?;
            list_agents(&config)?;
            list_skills(&config)?;
            list_hooks(&config)?;
        }
    }

    Ok(())
}

fn list_commands(config: &TstackConfig) -> Result<()> {
    let items = scan_md_items(&config.commands_dir(), &config.claude_commands_dir(), ItemType::Command, config.plugin_active);

    ui::heading("tstack commands");

    if items.is_empty() {
        ui::info("No commands found.");
        println!();
        return Ok(());
    }

    for item in &items {
        print_item(item);
    }
    println!();

    Ok(())
}

fn list_agents(config: &TstackConfig) -> Result<()> {
    let items = scan_md_items(&config.agents_dir(), &config.claude_agents_dir(), ItemType::Agent, config.plugin_active);

    ui::heading("tstack agents");

    if items.is_empty() {
        ui::info("No agents found.");
        println!();
        return Ok(());
    }

    for item in &items {
        print_item(item);
    }
    println!();

    Ok(())
}

fn list_skills(config: &TstackConfig) -> Result<()> {
    let items = scan_skills(config);

    ui::heading("tstack skills");

    if items.is_empty() {
        ui::info("No skills found.");
        println!();
        return Ok(());
    }

    for item in &items {
        print_item(item);
    }
    println!();

    Ok(())
}

fn list_hooks(config: &TstackConfig) -> Result<()> {
    let items = scan_hooks(config);

    ui::heading("tstack hooks");

    if items.is_empty() {
        ui::info("No hooks found.");
        println!();
        return Ok(());
    }

    for item in &items {
        let name = format!("{:<24}", item.name);
        println!("  {}  {}", name.bold(), item.source_path.display().to_string().dimmed());
    }
    println!();

    Ok(())
}

fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max_chars {
        let truncated: String = chars[..max_chars].iter().collect();
        format!("{truncated}…")
    } else {
        s.to_string()
    }
}

fn print_item(item: &TstackItem) {
    let status_icon = match &item.status {
        LinkStatus::Linked => "●".green().to_string(),
        LinkStatus::Broken => "●".red().to_string(),
        LinkStatus::Missing => "○".dimmed().to_string(),
        LinkStatus::Conflict(_) => "●".yellow().to_string(),
    };

    let name = format!("{:<24}", item.name);
    let desc = truncate_str(&item.description, 40);
    let model = item.model.as_deref().unwrap_or("");

    println!(
        "  {} {}  {:<42} {}",
        status_icon,
        name.bold(),
        desc.dimmed(),
        model.dimmed()
    );
}
