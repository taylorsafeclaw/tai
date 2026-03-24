# Contributing to tstack

tstack is a collection of markdown files that extend Claude Code. Contributing is as simple as adding or editing a `.md` file.

## What you can contribute

- **Commands** (`commands/<category>/<name>.md`) — new slash commands
- **Agents** (`agents/<category>/<name>.md`) — new specialized agents
- **Skills** (`skills/<name>/SKILL.md`) — new reusable skill modules
- **Hooks** (`hooks/*.js`) — new PreToolUse/PostToolUse hooks
- **Docs** (`docs/*.md`) — documentation improvements
- **CLI** (`cli/src/`) — Rust CLI improvements

## Adding a command

1. Create `commands/<category>/<name>.md` with required frontmatter:

```yaml
---
name: tstack:<name>
description: <one-line description>
argument-hint: "<hint for the user>"
model: sonnet | opus
---
```

2. Test it by installing the plugin (`claude plugin add /path/to/tstack`) and using it in Claude Code
3. Add a line to `docs/commands.md`
4. Open a PR

## Adding an agent

1. Create `agents/<category>/<name>.md` with required frontmatter:

```yaml
---
name: <name>
description: <one-line description>
model: sonnet | opus
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---
```

2. Test by installing the plugin and spawning the agent from a command
3. Add a section to `docs/agents.md`
4. Open a PR

## Adding a skill

1. Create `skills/<name>/SKILL.md` with frontmatter:

```yaml
---
name: <name>
description: "<when to use this skill>"
user-invocable: true | false
---
```

2. The plugin discovers skills automatically — no setup needed
3. Add a section to `docs/skills.md`
4. Open a PR

## Guidelines

- **Namespacing** — commands use `name: tstack:<name>` in frontmatter; agents and skills use plain names
- **One responsibility per file** — commands orchestrate, agents implement
- **Document the Return contract** in every agent — orchestrators depend on it
- **No secrets or credentials** in any files
- **Test before PR** — install the plugin and exercise your addition in a real Claude Code session

## CLI development

The CLI is a Rust binary in `cli/`. To build:

```bash
cd cli && cargo build
```

To run tests:

```bash
cd cli && cargo test
```

To run clippy:

```bash
cd cli && cargo clippy
```

## Opening a PR

1. Fork the repo
2. Create a branch: `git checkout -b feat/my-thing`
3. Make your changes
4. Open a PR against `main`
5. Describe what the command/agent/skill does and when it should be used
