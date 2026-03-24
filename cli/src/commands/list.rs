use anyhow::Result;
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
    let items = scan_md_items(
        &config.commands_dir(),
        &config.claude_commands_dir(),
        ItemType::Command,
        config.plugin_active,
    );

    ui::heading("commands");

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
    let items = scan_md_items(
        &config.agents_dir(),
        &config.claude_agents_dir(),
        ItemType::Agent,
        config.plugin_active,
    );

    ui::heading("agents");

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

    ui::heading("skills");

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

    ui::heading("hooks");

    if items.is_empty() {
        ui::info("No hooks found.");
        println!();
        return Ok(());
    }

    for item in &items {
        ui::list_item(
            ui::DIM,
            &item.name,
            &item.source_path.display().to_string(),
            "",
        );
    }
    println!();

    Ok(())
}

fn print_item(item: &TstackItem) {
    let status_color = match &item.status {
        LinkStatus::Linked => ui::GREEN,
        LinkStatus::Broken => ui::RED,
        LinkStatus::Missing => ui::DIM,
        LinkStatus::Conflict(_) => ui::YELLOW,
    };

    let model = item.model.as_deref().unwrap_or("");
    ui::list_item(status_color, &item.name, &item.description, model);
}
