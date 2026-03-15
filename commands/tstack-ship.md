---
name: tstack-ship
description: Full pipeline → PR — validate, commit, push, create PR. Returns PR URL.
argument-hint: "[PR title or description]"
model: sonnet
---

You are the tstack shipper. Take committed work and open a PR.

## Input

PR title or description hint: $ARGUMENTS

## Step 1 — Quality pipeline

```bash
pnpm lint
pnpm build
pnpm test
```

Stop on failure. Do not ship broken code.

## Step 2 — Check branch

```bash
git status
git branch --show-current
```

If on `main` or `master`: create a feature branch first:
```bash
git checkout -b feat/<slug>
```
Where `<slug>` is derived from `$ARGUMENTS` or recent commit messages.

## Step 3 — Commit uncommitted changes (if any)

If there are uncommitted changes, commit them with a conventional commit message derived from the diff.

## Step 4 — Push

```bash
git push -u origin <branch>
```

## Step 5 — Create PR

If the `tstack-pr-body` skill is available, invoke it to generate the PR description.
Otherwise, generate the PR body manually:

```bash
gh pr create --title "<title>" --body "$(cat <<'EOF'
## Summary
- <what this does>
- <why it's needed>

## Test plan
- [ ] <what to verify manually>
- [ ] <another check>
EOF
)"
```

Title: if `$ARGUMENTS` provided, use it. Otherwise derive from branch name or commits.

## Step 6 — Return

Show:
```
✓ PR opened: <URL>
Branch: feat/<slug>
Commits: N commits
```

Return the PR URL so the user can click it directly.
