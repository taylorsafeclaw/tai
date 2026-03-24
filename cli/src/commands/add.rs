use anyhow::{bail, Result};
use crate::cli::AddKind;
use crate::config::TstackConfig;

pub fn run(kind: AddKind, name: String) -> Result<()> {
    let config = TstackConfig::detect()?;

    let name = name.strip_prefix("tstack-").unwrap_or(&name);

    cliclack::intro("tstack add")?;

    match kind {
        AddKind::Command => add_command(&config, name)?,
        AddKind::Agent => add_agent(&config, name)?,
        AddKind::Skill => add_skill(&config, name)?,
    }

    cliclack::log::info("Run `tstack install` to symlink the new item.")?;
    cliclack::outro("Created successfully.")?;

    Ok(())
}

fn add_command(config: &TstackConfig, name: &str) -> Result<()> {
    let path = config.commands_dir().join(format!("{name}.md"));

    if path.exists() {
        bail!("Command already exists: {}", path.display());
    }

    let description: String = cliclack::input("Description:")
        .placeholder("What does this command do?")
        .interact()?;

    let model: String = cliclack::select("Model:")
        .item("sonnet", "Sonnet", "fast, good for implementation")
        .item("opus", "Opus", "deep thinking, planning")
        .item("haiku", "Haiku", "lightweight, validation")
        .interact().map(|s: &str| s.to_string())?;

    let spin = cliclack::spinner();
    spin.start("Creating command...");

    let desc = if description.is_empty() {
        "TODO — describe what this command does".to_string()
    } else {
        description
    };

    let content = format!(
        r#"---
name: {name}
description: {desc}
argument-hint: "<args>"
model: {model}
---

You are the {name} command.

## Task

$ARGUMENTS
"#
    );

    std::fs::write(&path, content)?;
    spin.stop(format!("Created {}", path.display()));

    Ok(())
}

fn add_agent(config: &TstackConfig, name: &str) -> Result<()> {
    let path = config.agents_dir().join(format!("{name}.md"));

    if path.exists() {
        bail!("Agent already exists: {}", path.display());
    }

    let description: String = cliclack::input("Description:")
        .placeholder("What does this agent do?")
        .interact()?;

    let model: String = cliclack::select("Model:")
        .item("sonnet", "Sonnet", "fast, good for implementation")
        .item("opus", "Opus", "deep thinking, planning")
        .item("haiku", "Haiku", "lightweight, validation")
        .interact().map(|s: &str| s.to_string())?;

    let spin = cliclack::spinner();
    spin.start("Creating agent...");

    let desc = if description.is_empty() {
        "TODO — describe what this agent does".to_string()
    } else {
        description
    };

    let content = format!(
        r#"---
name: {name}
description: {desc}
model: {model}
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---

You are the {name} agent.

## Responsibilities

TODO
"#
    );

    std::fs::write(&path, content)?;
    spin.stop(format!("Created {}", path.display()));

    Ok(())
}

fn add_skill(config: &TstackConfig, name: &str) -> Result<()> {
    let dir = config.skills_dir().join(name);
    let path = dir.join("SKILL.md");

    if dir.exists() {
        bail!("Skill already exists: {}", dir.display());
    }

    let description: String = cliclack::input("Description:")
        .placeholder("What does this skill do?")
        .interact()?;

    let spin = cliclack::spinner();
    spin.start("Creating skill...");

    let desc = if description.is_empty() {
        "TODO — describe what this skill does".to_string()
    } else {
        description
    };

    std::fs::create_dir_all(&dir)?;

    let content = format!(
        r#"---
name: {name}
description: {desc}
user-invocable: true
---

# {name}

TODO — skill instructions here.
"#
    );

    std::fs::write(&path, content)?;
    spin.stop(format!("Created {}", path.display()));

    Ok(())
}
