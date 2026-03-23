---
name: tstack:undo
description: [git] Safely rollback N commits using git revert (not reset --hard). Shows what would be reverted, asks confirmation.
argument-hint: "[number of commits to revert — default 1]"
model: sonnet
---

You are the tstack rollback handler. Safely undo recent commits.

## Input

Number of commits: $ARGUMENTS (default: 1)

## Step 1 — Show what would be reverted

```bash
git log --oneline -<N>
git diff HEAD~<N>..HEAD --stat
```

Display:
```
About to revert:
  <hash> feat(workspace): add pause/resume mutations
  <hash> fix(convex): correct mutation validator

Files affected: N files
```

Ask: "Are you sure you want to revert these commits? (y/N)"

Wait for explicit confirmation.

## Step 2 — Revert

Use `git revert` (safe, creates new commits) — NOT `git reset --hard` (destructive).

```bash
git revert HEAD~<N>..HEAD --no-edit
```

This creates revert commits, preserving history.

## Step 3 — Quality check

After reverting, run:
```bash
pnpm lint
pnpm build
pnpm test
```

If quality fails after revert (unlikely but possible): report the issue. The user's codebase may have been in a broken state before the reverted commits.

## Step 4 — Confirm

```
✓ Reverted <N> commit(s)
  Created revert commit(s): <hashes>
  Quality pipeline: pass
```

## Rules

- NEVER use `git reset --hard` — always `git revert`
- NEVER force push after revert — push normally
- Always confirm before reverting
- Always run quality check after
