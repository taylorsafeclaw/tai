---
title: Agents
---

Agents are specialized subprocesses that tstack commands spawn to do focused work. Each agent has a fixed model tier, a scope lock, and a defined return contract so the calling command knows exactly what it gets back. {% .lead %}

---

## How agents work

When a tstack command determines it needs exploration, implementation, or lifecycle work, it spawns the appropriate agent rather than doing that work inline. The agent:

1. Reads its bootstrap files (CLAUDE.md, package.json, relevant source files)
2. Does the work within its scope lock
3. Returns a structured result to the orchestrator

Commands check agent availability via Glob before spawning:

```
.claude/agents/<name>.md          → project-specific (highest priority)
~/.claude/agents/<name>.md        → global (from this repo)
```

If no suitable agent is found, commands fall back to inline implementation.

---

## Core agents

Core agents live in `agents/core/` and handle the two universal concerns: understanding the codebase and changing it.

### `explorer` — Codebase exploration specialist

**Model:** haiku | **Tools:** Read, Grep, Glob, Bash | **Max turns:** 20

Fast, read-only codebase exploration. Finds files, understands patterns, traces code paths, checks git history, and returns structured findings.

**Scope lock:** Read-only — does not modify any files.

**Key behavior:**

- Runs Glob and Grep in parallel for independent searches
- Follows import chains to trace code paths
- Returns file:line references so the caller can navigate directly
- Stops after 3 failed search attempts and reports what it found vs. what is missing

**Return format:**

```
## Files found
## Patterns observed
## Recent changes
## Gotchas
## Suggested approach
```

**Used by:** context, feature, task, scope, execute (context-gathering steps)

---

### `implementer` — Generic implementation agent

**Model:** sonnet | **Tools:** Read, Grep, Glob, Edit, Write, Bash | **Max turns:** 30

General-purpose implementation agent. Reads CLAUDE.md, follows project patterns, runs the quality pipeline, and commits atomically.

**Key behavior:**

- Reads CLAUDE.md and package.json before touching any code
- Finds existing patterns to follow before writing new code
- Runs `pnpm lint → pnpm build → pnpm test` in order; stops on first failure
- Max 2 fix attempts on quality failures, then stops and reports
- Stages files by name — never `git add -A`

**Return format:** What was implemented, files modified, quality result, notes/gotchas.

**Used by:** task, feature, implement, execute (fallback when no domain agents present)

---

## Lifecycle agents

Lifecycle agents live in `agents/lifecycle/` and support the post-commit pipeline: detecting where you are, analyzing commits, ingesting PR review comments, and applying fixes.

### `detect` — Lifecycle phase detector

**Model:** haiku | **Tools:** Bash, Read, Grep | **Max turns:** 10

Analyzes git state, open PRs, review comments, and Linear tickets to determine the current development phase. Used by lifecycle commands to infer context and suggest the next action.

**Key behavior:**

- Runs git status, branch position check, `gh pr view`, and review comment count in order
- Maps the results to one of seven defined phases (see table below)
- Returns a single-screen structured summary with a concrete command recommendation

**Phase definitions:**

| Phase | Condition |
|-------|-----------|
| CODING | Uncommitted changes, no open PR |
| READY_TO_COMMIT | Uncommitted changes, logically complete |
| READY_TO_SHIP | All committed, no open PR, ahead of main |
| AWAITING_REVIEW | Open PR, no review decision yet |
| REVIEW_CYCLE | Open PR, changes requested or unresolved comments |
| READY_TO_MERGE | Open PR, approved, checks passing |
| UP_TO_DATE | On main, no changes, no open PR |

**Return format:**

```
Phase: {PHASE}
Branch: {branch_name}
PR: #{number} ({state}, {reviewDecision}) — or "none"
Uncommitted: {count} files
Ahead of main: {count} commits
Unresolved comments: {count} — or "n/a"
Recommendation: Run /{recommended_command} {args}
```

**Used by:** `/status`, `/next`, `/resume`

---

### `commit-analyzer` — Diff grouping and commit boundary analyzer

**Model:** sonnet | **Tools:** Bash, Read, Grep, Glob | **Max turns:** 20

Analyzes the current diff for logical groupings — file clusters, module boundaries, import chains, feature vs. infrastructure separation. Proposes atomic commit boundaries.

**Key behavior:**

- Clusters changes by: module boundary, import chain, feature vs. infrastructure, test + implementation pairs, schema + migration pairs
- Detects multi-feature branches (unrelated clusters touching different domains)
- Each proposed commit must be independently valid (no broken intermediate states)
- Migration files always get their own commit

**Return format:**

```
## Proposed Commits (N total)

### Commit 1: {conventional commit message}
Files: ...
Rationale: ...

---
Multi-feature detected: yes/no
```

**Used by:** `/commit --split`, `/commit --multi`

---

### `review-ingester` — PR comment parser and classifier

**Model:** sonnet | **Tools:** Bash, Read, Grep | **Max turns:** 15
**Skills preloaded:** `false-positives`, `linear-mappings`

Fetches PR review comments via the GitHub API, classifies each by type (BUG / STYLE / SUGGESTION / QUESTION), assigns confidence scores (0–100), and filters false positives.

**Key behavior:**

- Fetches both review-level and inline comments via `gh api`
- Looks for structured `<!-- review-meta:start -->` markers first; falls back to text parsing
- Uses the `false-positives` skill to check pre-existing issues via `git blame` and filter linter-catchable problems
- Uses the `linear-mappings` skill for ticket field mappings and classification rules
- Confidence scoring: 90–100 = confirmed bug, 75–89 = very likely real, 50–74 = possible, 25–49 = likely false positive, 0–24 = almost certainly not real

**Return format:**

```
## PR #{number} Review Summary
Total comments: N | After filtering: N actionable (N skipped)

### BUG (N): ...
### STYLE (N): ...
### SUGGESTION (N): ...
### QUESTION (N): ...
### SKIPPED (N): ...
```

**Used by:** `/review-cycle --ingest`

---

### `fix-implementer` — Surgical fix applicator

**Model:** sonnet | **Tools:** Bash, Read, Grep, Edit, Write | **Max turns:** 15

Implements a single fix for one review issue. Reads context via git blame, applies the minimal targeted change, and commits atomically with a Linear ticket reference.

**Key behavior:**

- ONE fix per invocation — never batches multiple issues
- Reads ±20 lines of context around the referenced line before touching anything
- Prefers `Edit` over rewriting files
- Runs `pnpm lint` and `pnpm build` to verify the fix before committing
- If the fix is unclear or risky, reports back instead of guessing
- Commit message format: `fix: {description} (SAF-{ticket#})`
- Never modifies test assertions to make tests pass

**Used by:** `/review-cycle --fix`, `/review-cycle` (full pipeline)

---

## Skill preloading

Agents can declare skills they need in their frontmatter:

```yaml
skills:
  - false-positives
  - linear-mappings
```

Claude Code loads the referenced skills before the agent runs, making their content available as context. This keeps agent prompts lean while allowing domain knowledge to be composed in.

Skills with `disable-model-invocation: true` are reference catalogs (not callable workflows) — they inject static lookup data rather than running a procedure.

---

## Model strategy

| Model | Role | Agents |
|-------|------|--------|
| haiku | Fast reads, phase detection, status checks | explorer, detect |
| sonnet | Building, analysis, fixing, committing | implementer, commit-analyzer, review-ingester, fix-implementer |
| opus | Reserved for orchestrator commands (planning, mission, review-cycle) | — |

Agents do not inherit the calling command's model. Their model is set explicitly in frontmatter and reflects the complexity of their task.

---

## How commands dispatch agents

Commands use the `Task` tool to spawn agents, passing a description and the context the agent needs:

```
Task("Explore the codebase for patterns related to X", agent: "explorer")
Task("Implement the plan at plan.md", agent: "implementer")
```

Commands that coordinate multiple agents (like `/execute` or `/review-cycle`) run agents sequentially, passing the output of one as input to the next:

1. `/execute` → `explorer` (gather context) → `implementer` (make changes)
2. `/review-cycle` → `review-ingester` (classify comments) → `fix-implementer` (one fix per cycle) → loop until clean

---

## Project-specific agents

Add `.md` files to `<project>/.claude/agents/` to override global agents or add domain-specific ones. A project agent with the same name as a global agent takes priority.

See [Extensions](/docs/extensions) for priority resolution and [Install](/docs/install) for how project templates bundle agents.

---

## Adding a new agent

Use `/new-agent` to scaffold or create manually:

```markdown
---
name: <name>
description: <one-line description>
model: sonnet | opus | haiku
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
skills:          # optional
  - skill-name
---

You are the tstack <name>. <purpose statement>

## Bootstrap
## Scope lock
## Behavior
## Error recovery
## Return contract
```

Place in:

- `<project>/.claude/agents/<name>.md` — project-only
- `~/.claude/agents/<name>.md` — global (available everywhere after `plugin install`)
