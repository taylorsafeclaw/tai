---
name: review-cycle
description: "Fetch Claude Code GitHub Action review comments from a PR, classify issues, create Linear tickets, fix issues in code, push, and re-review until clean. Use when asked to 'ingest review', 'process review comments', 'fix review issues', 'review cycle', 'fetch PR feedback', 'create tickets from review', or 'run the fix loop'."
user-invocable: true
---

Post-PR review pipeline: ingest GH Action comments → classify → Linear tickets → fix → push → re-review.

## Arguments

| Input | Behavior |
|-------|----------|
| `/review-cycle <PR#>` | Full pipeline: ingest → tickets → fix → re-review loop |
| `/review-cycle <PR#> --ingest` | Fetch and classify comments only |
| `/review-cycle <PR#> --ticket` | Ingest + create Linear tickets (stop before fixing) |
| `/review-cycle <PR#> --fix` | Execute existing Linear tickets only (skip ingest) |
| `/review-cycle <PR#> --max-iter N` | Set max re-review iterations (default: 3) |

Parse the PR number from arguments. If missing, check current branch for an open PR via `gh pr view --json number`.

## Phase 1: Ingest

1. Detect repo owner/name:
   ```bash
   git remote get-url origin
   ```
   Parse `owner/repo` from the URL (handles both HTTPS and SSH formats).

2. Fetch PR review comments:
   ```bash
   gh api repos/{owner}/{repo}/pulls/{PR}/reviews --jq '.[] | {id, body, state, user: .user.login}'
   gh api repos/{owner}/{repo}/pulls/{PR}/comments --jq '.[] | {id, body, path, line, created_at, user: .user.login}'
   ```

3. Filter to bot comments. Read `references/config.md` for the bot username. Default: any username ending in `[bot]`.

4. Parse review body: extract individual issues — look for numbered lists, bullet points, file references (`path:line`), code blocks with suggestions.

5. Classify each issue. Read `references/linear-mappings.md` for classification rules and Linear field mappings.

6. Present summary to user:
   ```
   PR #42 Review Summary (N issues):

   BUG (N):
   1. [file:line] description

   STYLE (N):
   2. [file:line] description

   SUGGESTION (N):
   3. [file:line] description

   QUESTION (N):
   4. [file:line] description
   ```

If `--ingest` flag: stop here.

## Phase 2: Linear Tickets

For each classified issue, create a Linear ticket using `mcp__linear-server__save_issue`. Read `references/linear-mappings.md` for the exact field mappings (team ID, label IDs, priority values).

- Title format: `[PR-{number}] {brief description}`
- Description: full review comment text + link to PR + file:line reference
- Auto-assign to the current user

Present created tickets:
```
Created N Linear tickets:
- SAF-123: [PR-42] Missing null check in provisioning (Bug, High)
- SAF-124: [PR-42] Consider batch insert for action logs (Suggestion, Normal)
```

If `--ticket` flag: stop here.

## Phase 3: Fix Loop

1. Filter to actionable issues — bugs first (priority order), then suggestions. Skip questions entirely.
2. For each issue:
   a. Read the referenced file and surrounding context
   b. Understand the issue from the review comment
   c. Implement the fix
   d. Stage specific files, commit: `fix: {description} (SAF-{ticket#})`
   e. Update the Linear ticket status to "Done" via `mcp__linear-server__save_issue`
3. After all fixes: `git push`
4. Wait for GH Action review — poll `gh pr checks {PR#}` every 30s, max 5 min
5. Re-run Phase 1 (ingest new comments)
6. Check exit conditions:
   - **No new issues** → report "Clean review, all issues resolved" and stop
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

## Completion Status

- **DONE** — All review issues resolved, clean re-review
- **DONE_WITH_CONCERNS** — Some issues fixed, others remain (questions, style)
- **BLOCKED** — Cannot fetch PR comments or Linear API unavailable
- **NEEDS_CONTEXT** — PR number missing or ambiguous review comments
