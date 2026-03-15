---
title: Project Templates
description: Bundle project-specific agents, commands, and skills for reuse across your codebase
---

Project templates let you package a set of domain-specific agents, commands, and skills and install them into any project's `.claude/` directory with a single command. They're the right tool when a project needs agents that know about its schema, auth patterns, or framework conventions.

## When to use a template

- Your project has a specific backend (e.g., Convex, Rails, Django) that deserves a dedicated agent
- You want to share a set of custom commands across multiple projects that use the same stack
- You want a reproducible `.claude/` setup that can be installed fresh after cloning

## Template structure

```
~/tstack/templates/<my-project>/
├── install                 ← bash script that copies files to project .claude/
├── agents/
│   └── tstack-<name>.md       ← project-specific agents
├── commands/
│   └── tstack-<name>.md       ← project-specific commands
└── skills/
    └── tstack-<name>/
        └── SKILL.md        ← project-specific skills
```

## Creating a template

### 1. Create the template directory

```bash
mkdir -p ~/tstack/templates/my-project/agents
mkdir -p ~/tstack/templates/my-project/commands
mkdir -p ~/tstack/templates/my-project/skills
```

### 2. Add your agents

Create `~/tstack/templates/my-project/agents/tstack-backend.md`:

```markdown
---
name: tstack-backend
description: Backend implementation agent for my-project
model: sonnet
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---

You are the tstack backend agent for my-project.

## Bootstrap
Read these files first:
- src/db/schema.ts
- src/lib/auth.ts
- CLAUDE.md

## Behavior
Follow the existing mutation patterns in src/db/mutations/.
Run pnpm build and pnpm test before committing.
Commit atomically with conventional commit format.

## Return contract
Return: what was implemented, files modified, API shape exposed.
```

### 3. Write the install script

Create `~/tstack/templates/my-project/install`:

```bash
#!/usr/bin/env bash
set -euo pipefail

# Find project root (directory containing .claude/)
PROJECT_ROOT=$(pwd)
while [[ "$PROJECT_ROOT" != "/" && ! -d "$PROJECT_ROOT/.claude" ]]; do
  PROJECT_ROOT=$(dirname "$PROJECT_ROOT")
done

if [[ ! -d "$PROJECT_ROOT/.claude" ]]; then
  echo "Error: no .claude/ directory found. Run from inside a project."
  exit 1
fi

TEMPLATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

mkdir -p "$PROJECT_ROOT/.claude/agents"
mkdir -p "$PROJECT_ROOT/.claude/commands"
mkdir -p "$PROJECT_ROOT/.claude/skills"

# Copy agents
if [[ -d "$TEMPLATE_DIR/agents" ]]; then
  cp "$TEMPLATE_DIR/agents"/tstack-*.md "$PROJECT_ROOT/.claude/agents/" 2>/dev/null || true
fi

# Copy commands
if [[ -d "$TEMPLATE_DIR/commands" ]]; then
  cp "$TEMPLATE_DIR/commands"/tstack-*.md "$PROJECT_ROOT/.claude/commands/" 2>/dev/null || true
fi

# Copy skills
if [[ -d "$TEMPLATE_DIR/skills" ]]; then
  for skill_dir in "$TEMPLATE_DIR/skills"/tstack-*/; do
    skill_name=$(basename "$skill_dir")
    mkdir -p "$PROJECT_ROOT/.claude/skills/$skill_name"
    cp "$skill_dir/SKILL.md" "$PROJECT_ROOT/.claude/skills/$skill_name/"
  done
fi

echo "Installed $(basename "$TEMPLATE_DIR") template into $PROJECT_ROOT/.claude/"
```

Make it executable:

```bash
chmod +x ~/tstack/templates/my-project/install
```

### 4. Install into a project

Navigate to your project root (where `.claude/` lives) and run:

```bash
~/tstack/templates/my-project/install
```

Or use the CLI:

```bash
tstack template install my-project
```

## How project overrides work

When Claude Code resolves a command or agent name, it checks project-local files first:

```
<project>/.claude/agents/tstack-backend.md   ← wins (installed by template)
~/.claude/agents/tstack-implementer.md       ← fallback (global)
```

A project agent named `tstack-backend` will be used by any tstack command that looks for a backend agent, because tstack commands check `.claude/agents/` via Glob before falling back to global agents.

## Reference implementation

See `~/tstack/templates/example/` for a minimal working template. It includes a sample agent, command, and install script with comments explaining each step.

## Listing and managing templates

```bash
tstack template list              # show available templates
tstack template install <name>    # install a template
tstack doctor                     # check template inventory
```

## Tips

- Keep agents focused: one agent per domain (backend, frontend, infra)
- Include `## Bootstrap` in every agent — list the files it should read first
- Test the install script on a clean project before sharing
- Use `tstack doctor` to verify all installed agents are healthy after install
