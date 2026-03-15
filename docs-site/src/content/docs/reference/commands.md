---
title: Commands
description: All 23 tstack slash commands with full reference
---

All commands are available after running `~/tstack/setup`. Invoke them with `/tstack-<name>` in Claude Code.

---

## Tier 1: Task

### `/tstack-task <what to do>`
**Model:** sonnet

Quick atomic change from description to commit. No planning, no PR.

**Pipeline:** bootstrap → fast context (skip Explore for known files) → route to agent or implement directly → quality gate → commit

**Agent routing:**
- Checks for domain agents (tstack-convex, tstack-ui) via Glob
- For 1-3 file changes, prefers direct implementation over spawning agents
- Falls back to tstack-implementer or main context if no domain agents

**Rules:**
- No plan file
- No PR — commits to current branch
- No agent coordination
- No browser tests
- Stops on quality failure

**Example:**
```
/tstack-task "rename WorkspaceCard to AgentCard in all components"
```

---

## Tier 2: Feature

### `/tstack-feature <feature description>`
**Model:** opus

Full feature pipeline from description to merged PR.

**Pipeline:** bootstrap → context (Explore agent) → plan → **user confirms** → branch → agent coordination (dual-path) → quality gate → push → PR

**Agent coordination (dual-path):**
- **Path A:** Domain agents available → spawn tstack-convex first, extract API shape, spawn tstack-ui with API shape
- **Path B:** No domain agents → use tstack-implementer or implement directly
- Each agent commits atomically per logical chunk

**Error recovery:** Logs failures, saves progress to plan.md, attempts one fix, re-runs from failure point. Never loops more than twice.

**Example:**
```
/tstack-feature "add workspace pause/resume — pause stops the Fly machine, resume restarts it"
```

---

### `/tstack-context <task or feature description>`
**Model:** opus

Gather context before implementing. Uses Agent tool with `subagent_type: "Explore"` and specific investigation questions.

Finds: affected files, recent changes, patterns, available agents, gotchas.

**Scope lock:** Gathers context only — does not implement anything.

---

### `/tstack-plan <task or feature description>`
**Model:** opus

Create an implementation plan and wait for user confirmation.

**Small tasks (≤3 files, 1 domain):** plan stays in conversation as bullet points.

**Large tasks (>3 files or multiple domains):** writes plan.md (or `.tstack/features/<N>/plan.md` if mission active).

**Context window sizing:** Each agent's workload sized to ~50% of a fresh context window. Each chunk is self-contained.

Always asks "Does this plan look right?" and waits for confirmation.

---

### `/tstack-implement <task description or plan.md path>`
**Model:** sonnet

Route a task or plan to the right agents and execute.

- Checks agent availability via Glob
- Uses dual-path agent coordination (domain agents → fallback to tstack-implementer → main context)
- Atomic commits per logical chunk
- Runs quality gate after implementation

---

## Tier 3: Mission

### `/tstack-mission <description or requirements doc path>`
**Model:** opus

Start a multi-feature mission.

**Produces:**
- `.tstack/ROADMAP.md` — numbered features with goals and success criteria
- `.tstack/state.json` — progress tracker
- `.tstack/features/<N>/` — directory per feature for plans

**Context window sizing:** Each feature sized to be completable in a fresh context window (~50% utilization). Self-contained plans.

Will **not** overwrite an existing `.tstack/state.json` without asking.

---

### `/tstack-scope [feature number or name]`
**Model:** opus

Research a specific mission feature before planning.

Uses Agent tool with `subagent_type: "Explore"` (very thorough) to find what exists, what needs building, patterns to follow, dependencies, constraints.

Can invoke `tstack-research` skill for web research on external APIs/libraries.

**Scope lock:** Researches only — does not implement anything.

---

### `/tstack-execute [plan file path]`
**Model:** sonnet

Execute a feature plan with agent coordination.

- Reads plan.md (or searches `.tstack/features/<N>/plan.md` → `plan.md`)
- Uses dual-path agent coordination
- Updates plan.md with `[x]` marks as tasks complete
- Reports progress after each agent completes
- Error recovery: saves progress, suggests resume with `/tstack-execute`

---

### `/tstack-verify [feature number]`
**Model:** sonnet

Verify a completed feature against its ROADMAP.md success criteria.

- Reads success criteria from ROADMAP.md
- Checks each criterion (code existence, behavior wiring)
- Runs quality gate
- Reports pass/fail per criterion with file:line references

Single pass — does **not** fix anything.

---

### `/tstack-next`
**Model:** sonnet

Close the current feature and advance to the next.

1. Runs verification inline (not as separate command)
2. If fail: shows what's missing, stops
3. If pass: opens PR (with error recovery for push/auth failures), updates `state.json`, shows next feature goal

---

## Quality

### `/tstack-validate`
**Model:** haiku

Run the quality pipeline and report results. Does **not** fix anything.

```
pnpm lint   → stop on failure
pnpm build  → stop on failure
pnpm test   → stop on failure
[playwright] → if configured
```

Detects available scripts from package.json. Skips missing scripts.

---

### `/tstack-test [playwright|dogfood|all]`
**Model:** sonnet

Run browser tests. Smart-detects what's available.

| Mode | What runs |
|------|-----------|
| `playwright` | `npx playwright test` |
| `dogfood` | `tstack-dogfood` skill with credentials from `.claude/dogfood.json` |
| `all` | playwright then dogfood |
| (none) | auto-detects — runs playwright if configured |

Can invoke `tstack-test-gen` skill to generate missing tests before running.

**No hardcoded credentials** — reads from `.claude/dogfood.json`.

---

### `/tstack-review [files|staged|branch]`
**Model:** sonnet

Code review recent changes. High-confidence issues only.

- Checks for project-specific reviewer agent first (`.claude/agents/tstack-reviewer.md`)
- If available, spawns it instead of running generic review
- Can invoke `tstack-audit` skill for deeper security analysis

Checks: security (OWASP top 10), logic errors, convention violations.

---

## Git

### `/tstack-commit [commit message]`
**Model:** sonnet

Validate then commit. Runs quality gate, stages files specifically, conventional commit format.

### `/tstack-ship [PR title or description]`
**Model:** sonnet

Full pipeline → PR. Can invoke `tstack-pr-body` skill for rich PR descriptions.

### `/tstack-undo [N]`
**Model:** sonnet

Safely rollback N commits using `git revert` — never `git reset --hard`.

---

## Debug & Refactor

### `/tstack-debug <error message or stack trace>`
**Model:** opus

Systematic debugging. Uses parallel tool calls to read source + check git history simultaneously. Checks for known quality pipeline fix patterns first.

### `/tstack-refactor <what to refactor>`
**Model:** sonnet

Safe refactoring with reference discovery.

- Warns about existing uncommitted changes before starting
- Greps all references before touching anything
- On failure: reverts only refactored files (not `git checkout -- .`)
- On success: commits with `refactor(<scope>): <what>`

---

## Utility

### `/tstack-resume`
**Model:** sonnet — Session continuity.

### `/tstack-status`
**Model:** haiku — One-screen project pulse.

### `/tstack-help`
**Model:** haiku — Lists all commands, agents, AND skills.

### `/tstack-new-agent`
**Model:** sonnet — Scaffold new agent with full frontmatter (tools, maxTurns, etc.).

### `/tstack-new-command`
**Model:** sonnet — Scaffold new command.
