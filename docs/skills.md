# Skills

Skills are reusable capability modules that agents and commands can load. They provide domain knowledge, workflows, and reference data without cluttering agent prompts.

## How skills work

Skills live in `skills/<name>/SKILL.md`. Each has a frontmatter header and a body of instructions or reference content.

- **User-invocable** (`user-invocable: true`) — the user can call the skill directly (e.g., `/simplify`, `/qa`)
- **Agent-loaded** (`user-invocable: false`) — loaded by agents via the `skills:` frontmatter field, or auto-invoked by commands
- **Catalog skills** (`disable-model-invocation: true`) — inject static reference data rather than running a procedure; Claude Code reads the content but does not treat it as a callable workflow

The plugin discovers skills from `skills/` automatically when installed via `claude plugin add`.

---

## disable-model-invocation

Some skills are reference catalogs, not workflows. They use:

```yaml
disable-model-invocation: true
```

This tells Claude Code not to "run" the skill as a procedure. Instead, the skill's content is read as context — lookup tables, config values, filter rules. Agents that need this data declare it in their `skills:` frontmatter and get it injected before they start.

Current catalog skills: `false-positives`, `linear-mappings`, `review-config`

---

## context: fork for heavy skills

Skills that spawn browser sessions, run long fix loops, or produce large structured outputs (e.g., `qa`, `design-review`, `review-cycle`) run in a forked context by default. This prevents their output from bloating the calling command's context window.

---

## All skills (21)

### `audit`
**User-invocable:** yes

Security and performance audit. Checks OWASP top 10 (injection, auth bypass, SSRF, secrets exposure), N+1 queries, race conditions, and dependency vulnerabilities. Produces severity-rated findings. Use after implementing features or before shipping.

---

### `changelog`
**User-invocable:** yes

Generates changelog entries from git commits and diffs in Keep a Changelog format (Added / Changed / Fixed / Removed), with version stamping. Use before shipping or creating releases.

---

### `commit`
**User-invocable:** yes

Smart git commit with three modes:
- **Default** — validate quality pipeline, then create one atomic commit
- `--split` — analyze all changes and propose N atomic commits by logical concern
- `--multi` — split a multi-feature branch into separate branches with auto-created PRs

Uses the `commit-analyzer` agent for `--split` and `--multi` modes.

---

### `deps`
**User-invocable:** yes

Dependency analyzer. Finds unused packages, duplicate dependencies, bundle size bloat, license issues, and suggests lighter alternatives. Use during cleanup or before releases.

---

### `design-review`
**User-invocable:** yes

Visual design audit with fix loop. First Impression phase, 10-category checklist (~80 items), AI Slop detection (10 anti-patterns), dual scoring (Design A–F / Slop A–F), CSS-first fixes with atomic commits.

---

### `doc-sync`
**User-invocable:** yes

Post-ship documentation sync. Cross-references all `.md` files against the diff, updates drifted docs (README, ARCHITECTURE, CONTRIBUTING, CLAUDE.md, CHANGELOG), polishes CHANGELOG voice, and optionally bumps VERSION. Auto-invoked by `/ship` after PR creation.

---

### `dogfood`
**User-invocable:** no (side-effect skill — user must invoke explicitly)
**disable-model-invocation:** no

**Superseded by `qa`.** Browser QA testing via Playwright or Claude-in-Chrome. Walks through user flows, reports visual/functional/accessibility issues. Does not fix — report only. For the full test → triage → fix → verify loop, use `qa`.

---

### `env`
**User-invocable:** yes

Environment config validator. Cross-references `.env` files against code usage, flags missing variables, checks `.gitignore` safety, validates `.env.example` completeness. Use before deploys or when env-related bugs appear.

---

### `false-positives`
**User-invocable:** no
**disable-model-invocation:** true

Filter catalog for review comments. Identifies pre-existing issues (via `git blame`), linter-catchable problems, pedantic nitpicks, and other false positives that should be deprioritized or skipped during review cycle processing. Loaded by the `review-ingester` agent.

---

### `frontend-design`
**User-invocable:** no

Active design audit and reference for UI components, pages, and visual interfaces. Covers spacing, typography, color, accessibility, dark mode, and motion — and fixes issues found. Loaded automatically by UI agents.

---

### `linear-mappings`
**User-invocable:** no
**disable-model-invocation:** true

Linear field mappings for the review cycle: team ID, label IDs, confidence-to-priority mapping, ticket description templates. Loaded by the `review-ingester` agent to populate Linear tickets from classified review comments.

---

### `migrate`
**User-invocable:** yes

Database/schema migration planner. Detects migration framework (Prisma, Drizzle, Knex, raw SQL), analyzes schema diffs for data loss risks, generates rollback plans. Use before running migrations or when planning schema changes.

---

### `office-hours`
**User-invocable:** yes

Product brainstorming in two modes: Startup (6 forcing questions for product-market fit) and Builder (design thinking for side projects, hackathons, open source). Saves a design doc. Use before `/plan`.

---

### `pr-body`
**User-invocable:** no

Generates rich PR descriptions from commits, plan files, and changed files. Produces summary, what changed, breaking changes, test plan, and links to related issues. Called by `/ship` and `/next`.

---

### `qa`
**User-invocable:** yes

Browser QA with fix loop. Three tiers: Quick (critical/high only), Standard (+medium), Exhaustive (+cosmetic). Health scoring, diff-aware mode, atomic fix commits with regression tests. Supersedes `dogfood`.

---

### `research`
**User-invocable:** yes

Deep research using web search (brave_web_search, firecrawl), library docs (context7), and codebase exploration. Multi-hop search (up to 5 iterations), structured output to `research.md` with comparison tables, source citations, and confidence ratings.

---

### `retro`
**User-invocable:** yes
**Model:** haiku

Weekly engineering retrospective. Analyzes git history for shipping velocity, commit type breakdown, time distribution, hotspot files, and per-author breakdown with praise and growth areas. Persistent history in `.tstack/retros/` with week-over-week comparison.

---

### `review-config`
**User-invocable:** no
**disable-model-invocation:** true

Claude Code GitHub Action review configuration: bot username filter, polling settings, checklist priority order, trigger behavior. Loaded by the `review-ingester` agent and the `/review-cycle` command.

---

### `review-cycle`
**User-invocable:** yes

Post-PR review pipeline: ingest GitHub Action comments → classify with confidence scoring → create Linear tickets → fix issues in code → push → re-review until clean. Supports partial runs (`--ingest`, `--ticket`, `--fix`) and iteration cap (`--max-iter N`).

---

### `simplify`
**User-invocable:** yes

Reviews changed code for reuse, quality, and efficiency — then fixes issues found. Supports scope control via arguments (`file path`, `staged`, `last-commit`). Reports metrics on simplifications made. Use after implementing features or completing refactors.

---

### `test-gen`
**User-invocable:** yes

Generates tests for new or modified code. Detects test framework (Vitest, Jest, Playwright), finds existing test patterns, detects test file placement (co-located vs. `__tests__/`), avoids duplicating existing coverage. Supports `recent` mode via git diff.

---

## Skills vs commands

| Use a skill when... | Use a command when... |
|---------------------|-----------------------|
| The capability is reusable across multiple workflows | The workflow is a one-shot top-level entry point |
| An agent needs to compose it with other skills | The user invokes it directly by name |
| It's a reference catalog (no procedure to run) | It needs to coordinate multiple agents |
| You want it auto-loaded via `skills:` frontmatter | It needs frontmatter fields like `allowed-tools` |

Most user-facing workflows start as commands (`/qa`, `/ship`, `/review-cycle`). Skills handle the composable pieces those commands delegate to.

---

## Completion status protocol

All skills emit a structured completion status on exit:

- **DONE** — All steps completed successfully. Evidence provided.
- **DONE_WITH_CONCERNS** — Completed with caveats documented.
- **BLOCKED** — Cannot proceed. Reason given.
- **NEEDS_CONTEXT** — Missing information from user.

Skills follow a 3-strike escalation rule: after 3 failed attempts at the same step, stop and escalate to the user rather than looping.

---

## Project template skills

Project templates can include skills in `<template>/skills/<name>/SKILL.md`. These are installed into `<project>/.claude/skills/` by the template's install script.

---

## Creating a new skill

```bash
mkdir -p skills/<name>
```

Create `skills/<name>/SKILL.md`:

```markdown
---
name: <name>
description: "<when to use this skill>"
user-invocable: true | false
disable-model-invocation: true   # only for reference catalogs
---

<skill instructions>
```

The plugin discovers new skills automatically when installed via `claude plugin add`. No extra setup needed.

## Skill resolution priority

Same as commands and agents — project-specific overrides global:

1. `<project>/.claude/skills/<name>/SKILL.md` — project-specific
2. Plugin `skills/<name>/SKILL.md` — from the installed tstack plugin
