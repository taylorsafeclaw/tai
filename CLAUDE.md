# CLAUDE.md — tstack Framework

tstack is a Claude Code plugin — a structured, opinionated dev workflow delivered via the plugin system, not symlinks.

## Repo structure

```
tstack/
├── .claude-plugin/
│   └── plugin.json             ← plugin manifest
├── commands/                   ← slash commands, organized by category
│   ├── git/                    ← branch, commit, pr, ship, review
│   ├── lifecycle/              ← task, feature, mission, execute, validate
│   ├── planning/               ← plan, context, scope, roadmap
│   ├── quality/                ← lint, audit, test-gen
│   ├── testing/                ← qa, dogfood
│   ├── general/                ← help, status, new-command, new-agent
│   └── utility/                ← simplify, research, changelog
├── agents/                     ← subagents, organized by category
│   ├── core/                   ← implementer, reviewer, planner, researcher
│   └── lifecycle/              ← executor, validator, shipper
├── skills/                     ← skills (flat), each a <name>/SKILL.md
├── hooks/
│   ├── hooks.json              ← hook registry
│   └── *.js                    ← individual hook scripts
├── rules/                      ← path-scoped rule files
├── settings.json               ← default Claude settings
├── .mcp.json                   ← MCP server config
├── .lsp.json                   ← LSP config
├── output-styles/              ← named output style presets
├── cli/                        ← Rust CLI (`tstack doctor`, `tstack add`, etc.)
│   ├── Cargo.toml
│   └── src/
├── templates/                  ← project-specific extension templates
│   └── example/                ← minimal template stub
├── extensions/                 ← personal add-ons (gitignored)
├── docs/                       ← full documentation
├── VERSION                     ← semver
└── CLAUDE.md                   ← this file
```

## Install / uninstall

```bash
claude plugin add /path/to/tstack    # install
claude plugin remove tstack          # uninstall
```

No symlinks. The plugin system discovers all commands, agents, and skills automatically.

### Building the CLI

```bash
cd cli && cargo install --path .   # puts `tstack` on PATH
```

## Conventions

- No `tstack-` file prefix required — namespacing comes from the plugin system and directory structure
- Commands: `commands/<category>/<name>.md` with frontmatter (`name`, `description`, `argument-hint`, `model`)
- Agents: `agents/<category>/<name>.md` with frontmatter (`name`, `description`, `model`, `tools`, `maxTurns`)
- Skills: `skills/<name>/SKILL.md` with frontmatter (`name`, `description`, `user-invocable`)
- Project templates: `templates/<project>/` with an `install` script

## Adding components

**New command** — drop a `.md` file into the right `commands/<category>/` directory. Auto-discovered.

```yaml
---
name: <name>
description: <one-line description>
argument-hint: "<hint for the user>"
model: sonnet | opus | haiku
---
```

**New agent** — drop a `.md` file into `agents/<category>/`. Auto-discovered.

```yaml
---
name: <name>
description: <one-line description>
model: sonnet | opus | haiku
tools: Read, Grep, Glob, Edit, Write, Bash
domain: backend | frontend | schema | infrastructure | integration | testing | review | quality | orchestrator
maxTurns: 30
---
```

The `domain` field enables dynamic agent discovery. Commands like `/feature` and `/execute` glob `.claude/agents/*.md`, read frontmatter, and dispatch agents by domain in this order: schema -> backend -> infrastructure (parallel) -> integration -> frontend -> testing -> review -> quality.

**New skill** — create `skills/<name>/SKILL.md`. Auto-discovered.

```yaml
---
name: <name>
description: <one-line description>
user-invocable: true | false
---
```

## The three tiers

| Tier | Command | Scope | Model |
|------|---------|-------|-------|
| Task | `/task` | Minutes, 1–3 files, single commit, no PR | sonnet |
| Feature | `/feature` | Hours, 3–10 files, Agent Team, PR | opus → sonnet |
| Mission | `/mission` | Days/weeks, multiple features, multiple PRs | opus → sonnet → haiku |

## Model strategy

- **opus** — thinking: context (`/context`), planning (`/plan`), missions, scoping, debugging. Use boost mode for hard problems.
- **sonnet** — building: implementation (`/task`, `/execute`), review, refactoring, committing
- **haiku** — running: validation (`/validate`), status, help

## Quality pipeline

Every tier runs after implementation — no opt-out:
```
pnpm lint → pnpm build → pnpm test
Stop on first failure. Never commit broken code.
```

## Extension system

All extension points support project-level overrides at `.claude/<type>/`. Highest → lowest priority: project → personal (`extensions/`) → core (this repo).

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
| Context fork | `context:fork` in commands | Parallel context branching |

## State system (`.tstack/`)

tstack uses a file-based state directory that agents read/write, surviving context resets and sessions.

### State directory structure

```
.tstack/
├── state.json              ← mission tracker (existing)
├── .quality-passed          ← quality gate flag (existing)
├── STATE.md                 ← living project memory (100 lines max)
├── DECISIONS.md             ← locked user decisions
├── AGENTS.md                ← agent activity log
├── ROADMAP.md               ← feature roadmap (existing)
├── features/
│   ├── <n>/
│   │   ├── plan.md          ← task checklist (existing)
│   │   ├── RESEARCH.md      ← pre-implementation research
│   │   └── SUMMARY.md       ← what was built + claims
├── debug/
│   └── <slug>.md            ← persistent debug state
└── todos/
    └── <id>.md              ← captured ideas/tasks
```

### STATE.md — Living Memory (The Resume Brain)

Written/read by every agent at session start. Max 100 lines. Updated after every significant action. A new Claude session reading ONLY this file should know exactly what's happening and what to do next.

```markdown
# Project State
Updated: [YYYY-MM-DD HH:MM]

## Current Position
- Mission: <name> (if active)
- Feature: <n> of <total> — <name>
- Phase: researching | planning | implementing | reviewing | shipping | debugging
- Branch: <branch name>
- PR: #<number> (<status>) or "none"
- Last activity: [YYYY-MM-DD HH:MM] — <exactly what happened>

## What's In Progress
<1-3 sentences describing the EXACT state of work — not vague, concrete>

## Resume Instructions
<The exact next step to take. Be specific enough that a fresh session can act immediately.>

## Completed This Session
- [x] Schema additions (commit abc1234)
- [x] Backend mutations (commit def5678)
- [ ] UI task list page (in progress — type error)
- [ ] Tests

## Agent Roster (last used)
- convex — [date] pass, built task mutations
- ui — [date] FAILED, type mismatch on TaskCard props

## Active Blockers
- <anything preventing progress>

## Key Context (don't lose this)
- <critical decisions, API shapes, gotchas discovered during this session>
```

**Update rules:**
- "What's In Progress" and "Resume Instructions" are overwritten each update (always current)
- "Completed This Session" is appended (tracks progress)
- "Key Context" is curated — add important things, remove stale things
- "Agent Roster" shows only agents used in current feature
- Max 100 lines — if approaching limit, compress older entries

### DECISIONS.md — Locked Decisions

User decisions that all agents must respect. Three levels:

```markdown
# Decisions

## Locked (must follow exactly)
- [date] Use opus for all implementation agents
- [date] Always invoke /frontend-design before UI work

## Discretion (agent can adapt if justified)
- [date] Prefer sheets over modals for detail views

## Deferred (don't work on yet)
- [date] Dark mode support
```

### AGENTS.md — Agent Activity Log

Append-only log of agent dispatches and results. Critical for resume — tells a new session what already ran and what was returned.

```markdown
# Agent Activity

## [YYYY-MM-DD HH:MM] convex → feature: add task table
- Status: complete
- Files: convex/tasks/mutations.ts, convex/tasks/queries.ts
- API shape: api.tasks.mutations.create({ title, assignee })
- Commit: abc1234
- Quality: pass
- Handoff: API shape ready for ui agent

## [YYYY-MM-DD HH:MM] ui → feature: task list UI
- Status: FAILED
- Error: Type error in TaskCard.tsx:42
- Files touched (uncommitted): app/(app)/tasks/page.tsx
- Resume: fix query return type, then retry
```

**Trim rule:** Keep last 20 entries. When exceeding, archive older entries to `AGENTS-archive.md`.

### features/<n>/RESEARCH.md

Written by explorer/researcher before planning. Contains: files to touch, existing patterns, dependencies, risks.

### features/<n>/SUMMARY.md

Written after implementation. Contains: what was built (artifacts + file paths), claims (verifiable assertions), commits, notes for reviewer.

### debug/<slug>.md

Persistent debug state. Sections: Symptom (fixed on creation), Current Focus (overwritten each attempt), Evidence (append-only), Resolution.

### Agent state protocol

Every agent must follow this protocol:

**On Start (every agent, every invocation):**
1. Read `.tstack/STATE.md` — understand project position, what's in progress, resume instructions
2. Read `.tstack/DECISIONS.md` — respect all locked decisions, use discretion items as guidance
3. Read feature files if working on a feature: RESEARCH.md, plan.md, SUMMARY.md
4. Check for partial work — if STATE.md says a prior agent failed or was interrupted, pick up from where it stopped

**On Complete (every agent, every invocation):**
5. Update STATE.md: "What's In Progress", "Resume Instructions", "Completed This Session", "Key Context", "Agent Roster"
6. Append to AGENTS.md — structured entry: timestamp, agent, task, status, files, handoff data
7. Mark plan.md tasks — check off completed `- [ ]` items
8. Write SUMMARY.md — if you completed the last task in a feature, write claims

**On Failure:**
9. Still update STATE.md — describe what failed and why in "What's In Progress"
10. Resume Instructions — write exactly what needs to happen to unblock
11. Don't leave orphan state — note created-but-incomplete files so next agent doesn't duplicate work

## Documentation

Full docs in `docs/`:
- `docs/cli.md` — Rust CLI reference (`tstack doctor`, `tstack add`, etc.)
- `docs/tiers.md` — tier breakdown and decision guide
- `docs/commands.md` — all slash commands with args, model, behavior
- `docs/agents.md` — agents reference and coordination model
- `docs/skills.md` — skills system and all available skills
- `docs/hooks.md` — hook scripts, registry format, configuration
- `docs/quality-pipeline.md` — pipeline details and failure behavior
- `docs/missions.md` — state format, ROADMAP.md, feature loop
- `docs/agent-teams.md` — Agent Team coordination model
- `docs/extensions.md` — all extension points and priority resolution
- `docs/install.md` — install, uninstall, project templates
- `docs/plugin.md` — plugin.json manifest reference
