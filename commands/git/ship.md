---
name: tstack:ship
description: [git] Full pipeline → PR — validate, commit, push, create PR. Returns PR URL.
argument-hint: "[PR title or description]"
model: sonnet
---

You are the tstack shipper. Take committed work and open a PR.

## Input

PR title or description hint: $ARGUMENTS

## Step 1 — Detect base branch

```bash
# Check if a PR already exists for this branch
gh pr view --json baseRefName 2>/dev/null

# If no existing PR, detect default branch
git remote show origin | grep 'HEAD branch' | awk '{print $NF}'
```

Use the existing PR's base branch if found, otherwise use the default branch.

## Step 2 — Merge base branch

Before testing, ensure we're testing against the latest base:
```bash
git fetch origin <base>
git merge origin/<base> --no-edit
```

If merge conflicts occur, stop and report. Don't auto-resolve.

## Step 3 — Quality pipeline

```bash
pnpm lint
pnpm build
pnpm test
```

Stop on failure. Do not ship broken code.

## Step 4 — Check branch

```bash
git status
git branch --show-current
```

If on `main` or `master`: create a feature branch first:
```bash
git checkout -b feat/<slug>
```
Where `<slug>` is derived from `$ARGUMENTS` or recent commit messages.

## Step 5 — Commit uncommitted changes (if any)

If there are uncommitted changes, commit them with a conventional commit message derived from the diff.

### Bisectable commit splitting

If the uncommitted diff spans multiple domains (e.g., schema + API + UI), split into separate commits in this order:
1. Infrastructure/config changes
2. Models/schema changes
3. API/controller changes
4. UI/frontend changes
5. Tests
6. VERSION/CHANGELOG updates

Each commit should build and pass on its own. If splitting isn't cleanly possible, one commit is fine.

## Step 6 — Verification gate

If any code was committed in Step 5 (i.e., code changed after the initial test pass in Step 3), re-run the quality pipeline:
```bash
pnpm lint
pnpm build
pnpm test
```

This ensures we never push code that wasn't tested in its final form.

## Step 7 — Push

```bash
git push -u origin <branch>
```

## Step 8 — Create PR

If the `pr-body` skill is available, invoke it to generate the PR description.
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

## Step 9 — Post-ship tasks

**Update STATE.md** (if it exists):
- "Current Position" → PR: #<number> (open), Phase: shipping
- "What's In Progress" → "PR #<number> created. Awaiting review."
- "Resume Instructions" → "PR is open at <URL>. Run `/review-cycle <PR#>` to process review feedback."
- "Last activity" → timestamp + "Shipped PR #<number>"

**Append to AGENTS.md** (if it exists):
```markdown
## [YYYY-MM-DD HH:MM] shipper → ship: PR #<number>
- Status: complete
- PR: #<number> — <URL>
- Branch: <branch>
- Commits: <count>
- Base: <base branch>
```

**TODOS.md:** If `TODOS.md` exists, check if any items were completed by this PR's changes. Mark them `[x]`.

**Documentation sync:** If the `doc-sync` skill is available, invoke it to update any documentation that drifted from the shipped code.

## Step 10 — Return

Show:
```
✓ PR opened: <URL>
Branch: feat/<slug>
Commits: N commits
Base: <base branch>
Docs synced: yes/no
```

Return the PR URL so the user can click it directly.
