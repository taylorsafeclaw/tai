---
title: Install
description: Install tstack in one command
---

## Requirements

- macOS or Linux
- Claude Code CLI
- `pnpm` (for quality pipeline)
- `gh` CLI (for PR creation)

## Install

```bash
git clone https://github.com/tstack-framework/tstack.git ~/tstack && ~/tstack/setup
```

The `setup` script:
1. Symlinks every `commands/tstack-*.md` → `~/.claude/commands/`
2. Symlinks every `agents/tstack-*.md` → `~/.claude/agents/`
3. Symlinks every `skills/tstack-*/` → `~/.claude/skills/`
4. Prints a summary of installed commands, agents, and skills

After install, all `/tstack-*` commands are available in every Claude Code session.

## Verify

```bash
ls ~/.claude/commands/tstack-*.md
```

You should see 23 command symlinks.

## Uninstall

```bash
~/tstack/uninstall
```

Removes all `~/.claude/commands/tstack-*.md`, `~/.claude/agents/tstack-*.md`, and `~/.claude/skills/tstack-*/` symlinks. Does **not** touch any project-level `.claude/` files.

---

## Update

```bash
cd ~/tstack && git pull && ~/tstack/setup
```

`setup` is idempotent — re-running it refreshes all symlinks.

---

## Project templates

Templates install project-specific agents, commands, and skills into your project's `.claude/` directory. These override global tstack commands when they share a name.

### Using a template

```bash
~/tstack/templates/<template-name>/install
```

### Creating a template

Create a directory under `templates/` with an install script:

```
~/tstack/templates/<my-project>/
├── install                 ← copies agents + commands + skills to project .claude/
├── agents/
│   └── tstack-*.md            ← project-specific agent overrides
├── commands/
│   └── tstack-*.md            ← project-specific commands
└── skills/
    └── tstack-*/
        └── SKILL.md        ← project-specific skills
```

The `install` script should:
1. Find the project root (directory with `.claude/`)
2. `mkdir -p .claude/agents .claude/commands .claude/skills`
3. Copy `tstack-*.md` files and skill directories

See `templates/example/install` for a reference implementation.

---

## Runtime directory

Many tstack commands write to `.tstack/` in your project root:
- `.tstack/state.json` — mission progress state
- `.tstack/.quality-passed` — quality gate marker
- `.tstack/.agent-log` — agent coordination log

Add `.tstack/` to your project's `.gitignore`.
