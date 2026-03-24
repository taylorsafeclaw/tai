# Extension System

tstack is a Claude Code plugin — installed via `claude plugin add /path/to/tstack`. It is extended by dropping `*.md` files in specific directories. No registration, no config — the plugin system discovers them automatically.

## Directory priority

```
<project>/.claude/agents/*.md              ← highest (project override)
<project>/.claude/commands/*.md
    ↓
<tstack>/extensions/                       ← personal add-ons (gitignored)
    ↓
<tstack>/agents/                           ← core plugin agents
<tstack>/commands/                         ← core plugin commands (lowest)
```

A project-level `<name>.md` overrides the global one. This is how project templates work — they install project-specific agents that know about your schema, auth patterns, and conventions.

## Adding a command

**Global (available in all projects):**
```bash
cp my-command.md <tstack>/commands/<category>/my-command.md
```

**Personal add-on (global but not in git):**
```bash
mkdir -p <tstack>/extensions
cp my-command.md <tstack>/extensions/my-command.md
```

**Project-only:**
```bash
cp my-command.md <project>/.claude/commands/my-command.md
```

## Adding an agent

Same pattern:
```bash
# Project-only (most common):
cp my-agent.md <project>/.claude/agents/my-agent.md

# Global:
cp my-agent.md <tstack>/agents/<category>/my-agent.md
```

## Naming convention

Filenames are bare — e.g., `task.md`, `implementer.md`, not `tstack-task.md`. The plugin system handles namespacing automatically. In frontmatter, the `name:` field uses a `tstack:` colon namespace (e.g., `name: tstack:task`), but filenames themselves need no prefix.

Examples: `my-command.md`, `stripe.md`, `rails.md`

## Scaffolding

Use the built-in scaffolders instead of writing from scratch:

```
/new-command    → asks what the command should do, writes the file
/new-agent      → asks about domain + bootstrap, writes the file
```

## Project templates

For reusable project setups, create a template directory:

```
<tstack>/templates/<project-name>/
├── install                 ← copies agents + commands to project .claude/
├── agents/
│   └── *.md
└── commands/
    └── *.md
```

The `install` script should:
1. Find the project root (directory with `.claude/`)
2. `mkdir -p .claude/agents .claude/commands`
3. Copy `*.md` files

See `templates/example/install` for a reference implementation.

## Promotion path

Agents start project-specific and are promoted to global when they prove useful across multiple projects:

1. Identify what's project-specific vs generic in the agent
2. Extract project-specific bootstrap/patterns into the project override
3. Move the generic version to `<tstack>/agents/<category>/`
4. The project keeps its override — the global version is the fallback

Example: A project-specific `validate` agent that knows your exact `pnpm` commands could be generalized to auto-detect the package manager and test runner, then promoted to global.
