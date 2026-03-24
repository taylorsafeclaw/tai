---
name: tstack:review-cycle
description: "[lifecycle] Fetch Claude Code GH Action review comments from a PR, classify with confidence scoring, create Linear tickets, fix issues, push, and re-review until clean."
argument-hint: "<PR#> [--ingest | --ticket | --fix] [--max-iter N] [--all]"
model: opus
allowed-tools: >
  Bash(gh api:*), Bash(gh pr:*), Bash(git remote:*),
  Bash(git diff:*), Bash(git log:*), Bash(git status:*),
  Bash(git add:*), Bash(git commit:*), Bash(git push:*),
  Bash(git blame:*), Bash(git rev-parse:*),
  Bash(pnpm lint), Bash(pnpm build), Bash(pnpm test)
---

Post-PR review pipeline: ingest GH Action comments → classify → Linear tickets → fix → push → re-review.

> **Note:** This command is also available as the `review-cycle` skill for natural language triggering.
> The command is the canonical implementation — the skill delegates here.

## Arguments

| Input | Behavior |
|-------|----------|
| `/review-cycle <PR#>` | Full pipeline: ingest → tickets → fix → re-review loop |
| `/review-cycle <PR#> --ingest` | Fetch and classify comments only |
| `/review-cycle <PR#> --ticket` | Ingest + create Linear tickets (stop before fixing) |
| `/review-cycle <PR#> --fix` | Execute existing Linear tickets only (skip ingest) |
| `/review-cycle <PR#> --max-iter N` | Set max re-review iterations (default: 3) |
| `/review-cycle <PR#> --all` | Include low-confidence issues (below 75 threshold) |

Parse the PR number from arguments. If missing, check current branch for an open PR via `gh pr view --json number`.

## Phase 1: Ingest

Dispatch the `review-ingester` agent with the PR number. It will:

1. Detect repo owner/name from `git remote get-url origin`
2. Fetch PR review comments via GitHub API
3. Filter to bot comments (username ending in `[bot]`)
4. Parse structured `<!-- review-meta -->` markers (fall back to text parsing)
5. Classify each issue (bug/style/suggestion/question) using the `linear-mappings` skill
6. Assign confidence scores (0-100)
7. Filter false positives using the `false-positives` skill

Present the ingestion summary to the user.

If `--ingest` flag: **stop here**.

## Phase 2: Linear Tickets

For each classified issue with confidence >= 75 (or all if `--all`):

1. Read `linear-mappings` skill for field mappings (team ID, label IDs, priority values)
2. Create a Linear ticket via `mcp__linear-server__save_issue`:
   - Title: `[PR-{number}] {brief description}`
   - Description: full review comment + PR link + file:line + commit SHA link
   - Team: from skill mappings
   - Labels: from classification mapping
   - Priority: from classification mapping
   - Auto-assign to current user

Present created tickets:
```
Created N Linear tickets:
- SAF-123: [PR-42] Missing null check in provisioning (Bug, High)
- SAF-124: [PR-42] Consider batch insert for action logs (Suggestion, Normal)
```

If `--ticket` flag: **stop here**.

## Phase 3: Fix Loop

1. Sort actionable issues: bugs first (highest priority), then suggestions. Skip questions entirely.

2. For each issue, dispatch the `fix-implementer` agent:
   - Pass the file, line, description, and Linear ticket ID
   - Agent reads context, applies minimal fix, commits atomically
   - Commit message: `fix: {description} (SAF-{ticket#})`
   - Agent updates Linear ticket status to "Done"

3. After all fixes: `git push`

4. Wait for GH Action re-review:
   - Read `review-config` skill for polling settings
   - Poll `gh pr checks {PR#}` every 30s, max 5 min

5. Re-run Phase 1 (ingest new comments)

6. Check exit conditions:
   - **No new issues** → report "Clean review, all issues resolved" ✓
   - **Max iterations reached** (default 3) → report remaining issues and stop
   - **Same issue persists** after fix attempt → report "Unable to resolve" and stop
   - **No code changes made** in this iteration → stop (nothing fixable)
   - **Only questions/style remaining** → report and stop (not worth looping)

## Constraints

- Never force push
- Stage files by name, never `git add .`
- Commit messages reference Linear ticket IDs
- Present all proposed fixes for user review before committing (unless in a re-review loop iteration after initial approval)
- If a fix is unclear or risky, ask the user before proceeding

## State Updates

After each iteration of the fix loop:
1. **Update `.tstack/STATE.md`** (if it exists):
   - "What's In Progress" → "Review cycle iteration <N>/<max>. Issues resolved: <count>. Remaining: <count>."
   - "Resume Instructions" → current state of the review cycle
   - "Last activity" → timestamp + "Review cycle iteration <N>"

On completion:
2. **Write SUMMARY.md** (if in a feature directory):
   - Add final review status to the summary
   - Mark all claims as verified or note remaining concerns
3. **Update STATE.md**:
   - Phase → "shipped" (if DONE) or "reviewing" (if DONE_WITH_CONCERNS)
   - "What's In Progress" → final status description
4. **Append to AGENTS.md**:
   ```markdown
   ## [YYYY-MM-DD HH:MM] review-cycle → PR #<number>
   - Status: <DONE|DONE_WITH_CONCERNS|BLOCKED>
   - Iterations: <count>
   - Issues resolved: <count>
   - Issues remaining: <count>
   - Linear tickets: <list>
   ```

## Completion Status

- **DONE** — All review issues resolved, clean re-review
- **DONE_WITH_CONCERNS** — Some issues fixed, others remain (questions, style)
- **BLOCKED** — Cannot fetch PR comments or Linear API unavailable
- **NEEDS_CONTEXT** — PR number missing or ambiguous review comments
