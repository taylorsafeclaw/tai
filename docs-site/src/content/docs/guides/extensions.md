---
title: Extensions
description: Add project-specific or personal commands, agents, and skills to tstack
---

tstack is extended by dropping `tstack-*.md` files in specific directories. No registration, no config — Claude Code discovers them automatically.

## Directory priority

```
<project>/.claude/agents/tstack-*.md       ← highest (project override)
<project>/.claude/commands/tstack-*.md
    ↓
~/tstack/extensions/                       ← personal add-ons (gitignored)
    ↓
~/tstack/agents/                           ← core global agents
~/tstack/commands/                         ← core global commands (lowest)
```

A project-level `tstack-<name>.md` overrides the global one. This is how project templates work — they install project-specific agents that know about your schema, auth patterns, and conventions.

## Adding a command

**Global (available in all projects):**
```bash
cp my-command.md ~/tstack/commands/tstack-my-command.md
# Re-run setup to refresh symlink:
~/tstack/setup
```

**Personal add-on (global but not in git):**
```bash
mkdir -p ~/tstack/extensions
cp my-command.md ~/tstack/extensions/tstack-my-command.md
```

**Project-only:**
```bash
cp my-command.md <project>/.claude/commands/tstack-my-command.md
```

## Adding an agent

Same pattern:
```bash
# Project-only (most common):
cp my-agent.md <project>/.claude/agents/tstack-my-agent.md

# Global:
cp my-agent.md ~/tstack/agents/tstack-my-agent.md
~/tstack/setup
```

## Naming convention

All tstack files must be prefixed `tstack-`. This prevents collisions with other frameworks (`gsd-*`, `gstack-*`, etc.) and makes them instantly recognizable.

Examples: `tstack-my-command.md`, `tstack-stripe.md`, `tstack-rails.md`

## Scaffolding

Use the built-in scaffolders instead of writing from scratch:

```
/tstack-new-command    → asks what the command should do, writes the file
/tstack-new-agent      → asks about domain + bootstrap, writes the file
```

## Project templates

For reusable project setups, create a template directory:

```
~/tstack/templates/<project-name>/
├── install                 ← copies agents + commands to project .claude/
├── agents/
│   └── tstack-*.md
└── commands/
    └── tstack-*.md
```

The `install` script should:
1. Find the project root (directory with `.claude/`)
2. `mkdir -p .claude/agents .claude/commands`
3. Copy `tstack-*.md` files

See `templates/example/install` for a reference implementation.

## Promotion path

Agents start project-specific and are promoted to global when they prove useful across multiple projects:

1. Identify what's project-specific vs generic in the agent
2. Extract project-specific bootstrap/patterns into the project override
3. Move the generic version to `~/tstack/agents/`
4. The project keeps its override — the global version is the fallback

Example: A project-specific `tstack-validate` agent that knows your exact `pnpm` commands could be generalized to auto-detect the package manager and test runner, then promoted to global.
