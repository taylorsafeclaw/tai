---
name: review-cycle
description: "Fetch Claude Code GitHub Action review comments from a PR, classify issues, create Linear tickets, fix issues in code, push, and re-review until clean. Use when asked to 'ingest review', 'process review comments', 'fix review issues', 'review cycle', 'fetch PR feedback', 'create tickets from review', or 'run the fix loop'."
user-invocable: true
---

Post-PR review pipeline: ingest GH Action comments → classify → Linear tickets → fix → push → re-review.

## Mode Detection

| User intent | Mode |
|-------------|------|
| "review cycle" / "process review" / no flags | **Full pipeline** — ingest → tickets → fix → re-review loop |
| "ingest review" / "fetch review comments" / `--ingest` | **Ingest only** — fetch and classify comments |
| "create tickets from review" / `--ticket` | **Ingest + tickets** — classify and create Linear tickets |
| "fix review issues" / `--fix` | **Fix only** — execute existing Linear tickets |

Parse PR number from arguments. If missing, detect from current branch: `gh pr view --json number`.

## Phase 1: Ingest

1. Detect repo: `git remote get-url origin` → parse `owner/repo`
2. Fetch comments:
   ```bash
   gh api repos/{owner}/{repo}/pulls/{PR}/reviews --jq '.[] | {id, body, state, user: .user.login}'
   gh api repos/{owner}/{repo}/pulls/{PR}/comments --jq '.[] | {id, body, path, line, created_at, user: .user.login}'
   ```
3. Filter to bot comments (username ending in `[bot]`)
4. Parse structured `<!-- review-meta -->` markers, fall back to text parsing
5. Classify each issue: **BUG** / **STYLE** / **SUGGESTION** / **QUESTION**
6. Assign confidence scores (0-100):
   - 90-100: Confirmed bug with evidence
   - 75-89: Very likely real, verified against code
   - 50-74: Possible but may be intentional
   - Below 50: Likely false positive
7. Filter false positives: check `git blame` (pre-existing?), `git diff main..HEAD` (unmodified?)

Present summary. If `--ingest`: stop here.

## Phase 2: Linear Tickets

For each issue with confidence >= 75:
1. Create Linear ticket via `mcp__linear-server__save_issue`:
   - Title: `[PR-{number}] {brief description}`
   - Description: full review comment + PR link + file:line + commit SHA
   - Auto-assign to current user
2. Present created tickets.

If `--ticket`: stop here.

## Phase 3: Fix Loop

1. Sort: bugs first (highest priority), then suggestions. Skip questions.
2. For each issue:
   - Read file at referenced line + surrounding context
   - Apply minimal targeted fix
   - Stage specific files, commit: `fix: {description} (SAF-{ticket#})`
   - Update Linear ticket status to "Done"
3. Push all fixes: `git push`
4. Poll for GH Action re-review: `gh pr checks {PR#}` every 30s, max 5 min
5. Re-run Phase 1 on new comments
6. Exit when: no new issues, max iterations (3), same issue persists, or only questions/style remain

## Constraints

- Never force push
- Stage files by name, never `git add .`
- Commit messages reference Linear ticket IDs
- Present fixes for user review before committing (first iteration)
- Max re-review iterations: 3 (override with `--max-iter N`)
- Default confidence threshold: 75 (include lower-confidence issues with `--all`)

## State Updates

After each iteration, update `.tstack/STATE.md` and append to `.tstack/AGENTS.md` with status, iteration count, issues resolved/remaining.
