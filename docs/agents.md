# Agents

Agents are specialized subprocesses with domain knowledge baked in. tstack commands spawn them automatically based on the work domain.

## How agents work

When a tstack command (e.g., `/tstack-feature`) determines a task touches a specific domain, it spawns the appropriate agent rather than implementing inline. The agent:
1. Reads its bootstrap files (schema, patterns, conventions)
2. Does the work in its domain
3. Runs quality checks
4. Commits atomically
5. Returns an API shape or summary to the orchestrator

Commands check agent availability via Glob before spawning:
```
.claude/agents/tstack-<name>.md    → project-specific
~/.claude/agents/tstack-*.md       → global
```

If no domain agents are found, commands fall back to `tstack-implementer` or direct implementation.

---

## Global agents

Global agents live in `~/tstack/agents/` and are available in every project after running `./setup`.

### `tstack-explorer` — Codebase exploration specialist
**Model:** haiku | **Tools:** Read, Grep, Glob, Bash | **Max turns:** 20

Fast, read-only codebase exploration. Finds files, understands patterns, traces code paths, gathers context.

**Scope lock:** Read-only — does not modify any files.

**Used by:** tstack-context, tstack-feature, tstack-task, tstack-scope, tstack-execute (context gathering steps)

**Return format:** Structured findings with file:line references, patterns observed, gotchas.

---

### `tstack-implementer` — Generic implementation agent
**Model:** sonnet | **Tools:** Read, Grep, Glob, Edit, Write, Bash | **Max turns:** 30

General-purpose implementation agent for projects without domain-specific agents. Reads CLAUDE.md, follows project patterns, runs quality pipeline.

**Used by:** tstack-task, tstack-feature, tstack-implement, tstack-execute (fallback when no domain agents)

**Return format:** What was implemented, files modified, quality result.

---

## Project template agents

You can create project-specific agents by adding `.md` files to `<project>/.claude/agents/`. These override global agents with the same name.

See [extensions.md](extensions.md) for how project overrides work, and [install.md](install.md) for how project templates bundle agents together.

---

## Adding a new agent

Use `/tstack-new-agent` to scaffold a new agent. Or create a `.md` file manually:

```markdown
---
name: tstack-<name>
description: <one-line description>
model: sonnet | opus | haiku
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---

You are the tstack <name>. <purpose statement>

## Bootstrap
Read these files first:
- ...

## Scope lock
What this agent does NOT do.

## Behavior
Step-by-step workflow.

## Error recovery
Max attempts, when to stop.

## Return contract
What to return to the orchestrator.
```

Place in:
- `<project>/.claude/agents/tstack-<name>.md` — project-only
- `~/tstack/agents/tstack-<name>.md` — global (available everywhere)

See [extensions.md](extensions.md) for priority resolution.
