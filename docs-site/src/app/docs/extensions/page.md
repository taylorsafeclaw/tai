---
title: Extensions
---

Customize and extend tstack with project-level overrides. {% .lead %}

---

## Extension points

All extension points support project-level overrides at `.claude/<type>/`. Priority resolution: project > personal (`extensions/`) > core.

| Extension point | Location | Purpose |
|----------------|----------|---------|
| Commands | `commands/<category>/<name>.md` | Slash commands |
| Agents | `agents/<category>/<name>.md` | Subagents |
| Skills | `skills/<name>/SKILL.md` | Reusable skill modules |
| Hooks | `hooks/hooks.json` + `hooks/*.js` | Pre/post event scripts |
| CLAUDE.md | `CLAUDE.md` | Project-level instructions |
| Settings | `settings.json` | Claude behavior defaults |
| Rules | `rules/` | Path-scoped rule files |
| MCP | `.mcp.json` | MCP server configuration |
| LSP | `.lsp.json` | LSP server configuration |
| Output styles | `output-styles/` | Named response format presets |
| Agent teams | `agents/` composition | Multi-agent coordination |

---

## Adding a command

Drop a `.md` file into `commands/<category>/`:

```yaml
---
name: my-command
description: What it does
argument-hint: "<arg>"
model: sonnet
---
```

Commands are auto-discovered — no registration needed.

---

## Adding an agent

Drop a `.md` file into `agents/<category>/`:

```yaml
---
name: my-agent
description: What it does
model: sonnet
tools: Read, Grep, Glob, Edit, Write, Bash
domain: backend
maxTurns: 30
---
```

The `domain` field enables dynamic discovery. Commands like `/feature` dispatch agents by domain.

---

## Adding a skill

Create `skills/<name>/SKILL.md`:

```yaml
---
name: my-skill
description: What it does
user-invocable: true
---
```

---

## Personal extensions

The `extensions/` directory (gitignored) holds personal add-ons. Same structure as core — commands, agents, skills. Lower priority than project-level overrides but higher than core.

---

## Project templates

Templates in `templates/<project>/` include an `install` script for bootstrapping new projects with tstack conventions.
