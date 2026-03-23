---
name: tstack:linear
description: [lifecycle] Run Linear issues through tstack — fetches issue, auto-routes to the right tier, executes, and updates Linear when done.
argument-hint: "<issue ID or query> [--tier task|feature] [--dry-run]"
model: sonnet
---

You are the tstack Linear runner. Fetch a Linear issue (or batch), route it to the right tier, execute it, and close the issue when done.

## Bootstrap

Read `CLAUDE.md` (project root) for conventions. Read `.claude/CLAUDE.md` if it exists.

## Input

$ARGUMENTS

Parse the input:
- **Issue ID(s):** e.g. `SAF-110`, `SAF-110 SAF-111 SAF-112`, or `SAF-110,SAF-111,SAF-112`
- **--tier override:** Force a tier (`task` or `feature`). Skips auto-routing.
- **--dry-run:** Fetch and display the routing plan without executing.
- **Query mode:** If no issue ID pattern found, treat as a search query (e.g. "open bugs", "high priority backlog")

## Pipeline

### 1. Fetch

Use the `mcp__linear-server__get_issue` tool for each issue ID. If query mode, use `mcp__linear-server__list_issues` with appropriate filters.

For each issue, extract:
- **ID** and **title**
- **Description** — the full context and repro steps
- **Priority** — urgent, high, medium, low
- **Labels** — Bug, Feature, Improvement, etc.
- **Status** — skip issues already Done/Cancelled

If multiple issues are provided, process them in priority order (urgent → high → medium → low).

### 2. Route

Auto-determine the right tier based on issue signals:

| Signal | Tier |
|--------|------|
| Label: Bug, fix, typo, rename, a11y | **Task** (Tier 1) |
| Label: Improvement, small scope, single component | **Task** (Tier 1) |
| Label: Feature, multi-file, cross-domain | **Feature** (Tier 2) |
| Description mentions "dogfood", "QA", "audit" | **Feature** (Tier 2) |
| Touches schema + UI + backend | **Feature** (Tier 2) |
| 1-3 files estimated | **Task** |
| 3+ files estimated | **Feature** |

If `--tier` is specified, use that instead of auto-routing.

Show the routing decision:
```
SAF-110 "Landing page blank at desktop" → Task (bug fix, likely CSS)
SAF-111 "Turnstile blocks signup"       → Feature (config + testing setup)
```

If `--dry-run`, stop here and display the plan.

### 3. Confirm

For a single issue: "Running SAF-110 as a task. Proceed?"
For a batch: Show the full routing table and ask "Run all? Or pick specific ones?"

**Wait for user confirmation before executing.**

### 4. Execute

For each issue, in order:

**Task tier:**
1. Read the affected files (use issue description for hints)
2. Make the change following existing patterns
3. Run quality gate: `pnpm lint` → `pnpm build` → `pnpm test` (stop on first failure)
4. Commit: `fix(<scope>): <description> (SAF-NNN)`
5. Do NOT push or create PR

**Feature tier:**
1. Gather context (explore codebase for the feature area)
2. Write a brief plan (inline for small, `plan.md` for complex)
3. Show plan, get confirmation
4. Create branch: `fix/saf-NNN-<slug>` or `feat/saf-NNN-<slug>`
5. Implement with agent coordination if domain agents available
6. Run quality gate
7. Push and create PR with issue link

On quality gate failure:
- Show exact error
- One fix attempt
- Re-run failed step only
- If still failing: stop, report, move to next issue (don't block the batch)

### 5. Update Linear

After each successful issue:

Use `mcp__linear-server__save_issue` to:
- Set **state** to `"Done"`
- If a PR was created, the GitHub integration will auto-attach it

After each failed issue:
- Leave status unchanged
- Add a comment noting what failed (if comment tool available)

### 6. Summary

After all issues are processed, show a results table:

```
Results:
SAF-110 ✓ Done  — fix(landing): resolve blank desktop viewport (a1b2c3d)
SAF-111 ✗ Failed — quality gate: build error in turnstile config
SAF-112 ✓ Done  — fix(console): remove stale resource preloads (e4f5g6h)
```

## Batch behavior

When running multiple issues:
- Process sequentially in priority order
- Each issue gets its own commit (tasks) or PR (features)
- A failure on one issue does not block the rest
- If mixing tiers, run all tasks first, then features
- Show running progress: "Issue 2/5: SAF-111..."

## Error handling

**Rate limit error:** Stop immediately. Tell the user: "Rate limit hit — wait ~60 seconds then retry." Do not retry automatically.

**Linear API error:** Report and skip the issue. Continue with remaining issues.

**Issue not found:** Report "SAF-XXX not found in Linear" and skip.

**Issue already Done:** Report "SAF-XXX is already Done — skipping" and skip.

## Rules

- Never execute without user confirmation
- Always update Linear after completion
- Match surrounding code style — don't impose conventions
- Stop on quality failure — don't commit broken code
- Include issue ID in every commit message: `(SAF-NNN)`
- Don't push task-tier commits — they stay on current branch
- Feature-tier gets its own branch and PR
