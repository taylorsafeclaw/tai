use anyhow::{bail, Result};
use crate::cli::AddKind;
use crate::config::TstackConfig;
use crate::ui;

pub fn run(kind: AddKind, name: String) -> Result<()> {
    let config = TstackConfig::detect()?;

    // Normalize name: strip tstack- prefix if provided
    let name = name.strip_prefix("tstack-").unwrap_or(&name);
    let tstack_name = format!("tstack-{name}");

    match kind {
        AddKind::Command => add_command(&config, name, &tstack_name)?,
        AddKind::Agent => add_agent(&config, name, &tstack_name)?,
        AddKind::Skill => add_skill(&config, name, &tstack_name)?,
    }

    println!();
    ui::info("Run `tstack install` to symlink the new item.");
    println!();

    Ok(())
}

fn add_command(config: &TstackConfig, _name: &str, tstack_name: &str) -> Result<()> {
    let path = config.commands_dir().join(format!("{tstack_name}.md"));

    if path.exists() {
        bail!("Command already exists: {}", path.display());
    }

    let content = format!(
        r#"---
name: {tstack_name}
description: TODO — describe what this command does
argument-hint: "<args>"
model: sonnet
---

You are the {tstack_name} command.

## Task

$ARGUMENTS
"#
    );

    std::fs::write(&path, content)?;
    ui::heading("tstack add command");
    ui::success(&format!("Created {}", path.display()));

    Ok(())
}

fn add_agent(config: &TstackConfig, _name: &str, tstack_name: &str) -> Result<()> {
    let path = config.agents_dir().join(format!("{tstack_name}.md"));

    if path.exists() {
        bail!("Agent already exists: {}", path.display());
    }

    let content = format!(
        r#"---
name: {tstack_name}
description: TODO — describe what this agent does
model: sonnet
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---

You are the {tstack_name} agent.

## Responsibilities

TODO
"#
    );

    std::fs::write(&path, content)?;
    ui::heading("tstack add agent");
    ui::success(&format!("Created {}", path.display()));

    Ok(())
}

fn add_skill(config: &TstackConfig, _name: &str, tstack_name: &str) -> Result<()> {
    let dir = config.skills_dir().join(tstack_name);
    let path = dir.join("SKILL.md");

    if dir.exists() {
        bail!("Skill already exists: {}", dir.display());
    }

    std::fs::create_dir_all(&dir)?;

    let content = format!(
        r#"---
name: {tstack_name}
description: TODO — describe what this skill does
user-invocable: true
---

# {tstack_name}

TODO — skill instructions here.
"#
    );

    std::fs::write(&path, content)?;
    ui::heading("tstack add skill");
    ui::success(&format!("Created {}", path.display()));

    Ok(())
}
