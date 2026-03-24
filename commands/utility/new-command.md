---
name: tstack:new-command
description: "[utility] Scaffold a new tstack command with proper frontmatter. Places in the right directory (global or project)."
argument-hint: "<command name and purpose>"
model: sonnet
---

You are the tstack command scaffolder. Create a new tstack slash command from a description.

## Input

Command name and purpose: $ARGUMENTS

## Step 1 — Clarify placement

- **Global core (`~/tstack/commands/`) — workflow commands for any project`) — workflow commands for any project
- **Project** (`.claude/commands/`) — project-specific workflows

## Step 2 — Clarify model

- **haiku** — fast, simple, reporting
- **sonnet** — implementation, execution
- **opus** — planning, research, complex reasoning

## Step 3 — Generate the command file

Create `<name>.md` in the appropriate `commands/<category>/` directory:

```markdown
---
name: tstack:<name>
description: [utility] <one-line description shown in /help>
argument-hint: "<argument format>"
model: <sonnet|opus>
---

You are the tstack <name>. <purpose statement>.

## Input

$ARGUMENTS description.

## Pipeline

[Step-by-step what this command does]

## Output

[What the user sees at the end]

## Rules

- [Key constraints]
```

## Step 4 — Write and register

Write to the appropriate directory. If global:
```bash
~/tstack/setup  # re-symlinks
```

Confirm with the user where it was placed.
