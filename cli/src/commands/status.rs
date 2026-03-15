use anyhow::Result;
use crate::config::TstackConfig;
use crate::types::*;
use crate::ui;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;
    let version = config.version();

    ui::print_logo(&version);

    let commands = scan_md_items(&config.commands_dir(), &config.claude_commands_dir(), ItemType::Command);
    let agents = scan_md_items(&config.agents_dir(), &config.claude_agents_dir(), ItemType::Agent);
    let skills = scan_skills(&config);
    let hooks = scan_hooks(&config);

    let cmd_healthy = commands.iter().filter(|i| i.status.is_healthy()).count();
    let agent_healthy = agents.iter().filter(|i| i.status.is_healthy()).count();
    let skill_healthy = skills.iter().filter(|i| i.status.is_healthy()).count();

    ui::status_line(commands.len(), "commands", cmd_healthy);
    ui::status_line(agents.len(), "agents", agent_healthy);
    ui::status_line(skills.len(), "skills", skill_healthy);
    ui::hook_status_line(hooks.len(), 0); // TODO: detect configured hooks

    println!();
    ui::info_line("root", &config.tstack_root.display().to_string());
    ui::info_line("home", &config.claude_dir.display().to_string());
    println!();

    Ok(())
}
