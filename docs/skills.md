# Skills

Skills are reusable capability modules that agents and commands can load. They provide domain knowledge, workflows, and reference material without cluttering agent prompts.

## How skills work

Skills live in `skills/tstack-<name>/SKILL.md` directories. Each has a `SKILL.md` file with frontmatter and instructions.

- **User-invocable skills** (`user-invocable: true`) can be called directly by the user
- **Agent-loaded skills** (`user-invocable: false`) are loaded by agents via `skills:` frontmatter or invoked programmatically

Skills are symlinked to `~/.claude/skills/` by `./setup` (global) or copied to `<project>/.claude/skills/` by template install scripts (project-specific).

---

## Global skills (8)

### `tstack-frontend-design`
**User-invocable:** no

Design principles for building UI: typography, color, spatial composition, responsive patterns, anti-patterns. Loaded by UI agents automatically.

### `tstack-simplify`
**User-invocable:** yes

Reviews changed code for reuse, quality, and efficiency. Checks information architecture, code patterns, and reuse opportunities. Verifies with quality pipeline.

### `tstack-dogfood`
**User-invocable:** yes (side effects — user must invoke)

Browser QA testing via Playwright or Claude-in-Chrome. Walks through user flows, reports visual/functional issues. Credentials from `.claude/dogfood.json` — never hardcoded.

### `tstack-research`
**User-invocable:** yes

Deep research using web search, documentation, and codebase exploration. Multi-hop search (up to 5 iterations), structured output to `research.md` with source citations and confidence ratings.

### `tstack-audit`
**User-invocable:** yes

Security and performance audit. Checks OWASP top 10 (injection, auth, secrets), performance issues (N+1, missing indexes, unbounded queries, memory leaks). Severity-rated output.

### `tstack-test-gen`
**User-invocable:** yes

Generates tests following project patterns. Detects framework (Vitest, Jest, Playwright), finds existing test patterns, generates happy path + edge cases + error cases.

### `tstack-changelog`
**User-invocable:** yes

Generates changelog entries from git commits in Keep a Changelog format. Categorizes by Added/Changed/Fixed/Removed.

### `tstack-pr-body`
**User-invocable:** no

Generates rich PR descriptions from commits, plan files, and changed files. Called by orchestrator commands (tstack-ship, tstack-next).

---

## Project template skills

Project templates can include skills in `<template>/skills/tstack-<name>/SKILL.md`. These are installed into `<project>/.claude/skills/` by the template's install script.

---

## Creating a new skill

```bash
mkdir -p skills/tstack-<name>
```

Create `skills/tstack-<name>/SKILL.md`:

```markdown
---
name: tstack-<name>
description: "<when to use this skill>"
user-invocable: true | false
---

<skill instructions>
```

Run `./setup` to symlink it to `~/.claude/skills/`.

## Skill resolution priority

Same as commands and agents — project-specific overrides global:

1. `<project>/.claude/skills/tstack-<name>/SKILL.md` — project-specific
2. `~/.claude/skills/tstack-<name>/SKILL.md` — global (from this repo)
