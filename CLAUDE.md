# CLAUDE.md — tstack Framework

This is the tstack dev framework repository. tstack is a lightweight, installable workflow for Claude Code — three work tiers, opinionated quality pipeline, plug-and-play agents and skills.

## Repo structure

```
tstack/
├── cli/                ← Rust CLI binary (`tstack install`, `tstack doctor`, etc.)
│   ├── Cargo.toml
│   └── src/
├── commands/           ← slash commands → ~/.claude/commands/tstack-*.md
├── agents/             ← subagents → ~/.claude/agents/tstack-*.md
├── skills/             ← skills → ~/.claude/skills/tstack-*/SKILL.md
├── hooks/              ← hook scripts (quality gate, branch guard, etc.)
├── templates/          ← project-specific extensions
│   └── example/        ← minimal template stub (reference implementation)
│       ├── install     ← copies agents + commands + skills to project .claude/
│       ├── agents/     ← example project-specific agent
│       └── commands/   ← example project-specific command
├── extensions/         ← personal add-ons (gitignored, not in repo)
├── docs/               ← full documentation
├── setup               ← symlinks commands + agents + skills to ~/.claude/
├── uninstall           ← removes all symlinks
├── VERSION             ← semver
└── CLAUDE.md           ← this file
```

## Install / uninstall

```bash
# Via CLI (preferred):
tstack install      # symlinks commands + agents + skills to ~/.claude/
tstack uninstall    # removes all symlinks

# Or via bash scripts:
./setup          # same as tstack install
./uninstall      # same as tstack uninstall
```

Both are idempotent — safe to re-run after adding commands, agents, or skills.

### Building the CLI

```bash
cd cli && cargo install --path .   # puts `tstack` on PATH
# or: cd cli && cargo build --release
```

## Conventions

- All tstack files are prefixed `tstack-` — prevents collisions with other frameworks (`gsd-*`, `gstack-*`)
- Commands: `commands/tstack-*.md` with frontmatter (`name`, `description`, `argument-hint`, `model`)
- Agents: `agents/tstack-*.md` with frontmatter (`name`, `description`, `model`, `tools`, `maxTurns`)
- Skills: `skills/tstack-*/SKILL.md` with frontmatter (`name`, `description`, `user-invocable`)
- Project templates: `templates/<project>/` with an `install` script

## Adding a new command

```bash
# Via CLI:
tstack add command my-thing   # scaffolds commands/tstack-my-thing.md
tstack install                # refresh symlinks

# Via slash command:
/tstack-new-command

# Or manually:
# 1. Create commands/tstack-<name>.md with frontmatter
# 2. Run tstack install (or ./setup)
```

Frontmatter format:
```yaml
---
name: tstack-<name>
description: <one-line description>
argument-hint: "<hint for the user>"
model: sonnet | opus | haiku
---
```

## Adding a new agent

```bash
# Scaffold with the built-in tool:
/tstack-new-agent

# Or create manually:
# 1. Create agents/tstack-<name>.md with frontmatter
# 2. Run ./setup to refresh symlinks
```

Agent frontmatter:
```yaml
---
name: tstack-<name>
description: <one-line description>
model: sonnet | opus | haiku
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---
```

## Adding a new skill

```bash
# 1. Create skills/tstack-<name>/SKILL.md with frontmatter
# 2. Run ./setup to refresh symlinks
```

Skill frontmatter:
```yaml
---
name: tstack-<name>
description: <one-line description>
user-invocable: true | false
---
```

## The three tiers

| Tier | Command | Scope | Model |
|------|---------|-------|-------|
| Task | `/tstack-task` | Minutes, 1–3 files, single commit, no PR | sonnet |
| Feature | `/tstack-feature` | Hours, 3–10 files, Agent Team, PR | opus → sonnet |
| Mission | `/tstack-mission` | Days/weeks, multiple features, multiple PRs | opus → sonnet → haiku |

## Model strategy

- **opus** — thinking: context (`/tstack-context`), planning (`/tstack-plan`), missions, scoping, debugging
- **sonnet** — building: implementation (`/tstack-task`, `/tstack-execute`), review, refactoring, committing
- **haiku** — running: validation (`/tstack-validate`), status, help

## Quality pipeline

Every tier runs after implementation — no opt-out:
```
pnpm lint → pnpm build → pnpm test → [browser: smart detect]
Stop on first failure. Never commit broken code.
```

## Extension system

Commands, agents, and skills can live in three places (highest → lowest priority):
1. `<project>/.claude/commands|agents|skills/tstack-*.md` — project-specific
2. `~/tstack/extensions/tstack-*.md` — personal add-ons (gitignored)
3. `~/tstack/commands|agents|skills/tstack-*.md` — core (this repo)

## Documentation

Full docs in `docs/`:
- `docs/cli.md` — Rust CLI reference (`tstack`, `tstack doctor`, etc.)
- `docs/tiers.md` — tier breakdown and decision guide
- `docs/commands.md` — all slash commands with args, model, behavior
- `docs/agents.md` — agents reference (global + project template agents)
- `docs/skills.md` — skills system and all available skills
- `docs/hooks.md` — hook scripts and configuration
- `docs/quality-pipeline.md` — pipeline details and failure behavior
- `docs/missions.md` — state format, ROADMAP.md, feature loop
- `docs/agent-teams.md` — Agent Team coordination model
- `docs/extensions.md` — extension system and priority resolution
- `docs/install.md` — install, uninstall, project templates
