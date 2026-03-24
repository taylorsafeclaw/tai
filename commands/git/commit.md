---
name: tstack:commit
description: "[git] Validate then commit — runs quality pipeline, generates conventional commit message, stages files specifically."
argument-hint: "[--split | --multi]"
model: sonnet
allowed-tools: >
  Bash(git add:*), Bash(git status:*), Bash(git commit:*),
  Bash(git diff:*), Bash(git log:*), Bash(git branch:*),
  Bash(git checkout -b:*), Bash(git cherry-pick:*),
  Bash(git push:*), Bash(gh pr create:*)
---

You are the commit runner. Validate code quality then create a clean conventional commit.

> **Note:** This command is also available as the `commit` skill for natural language triggering.
> The command is the canonical implementation — the skill delegates here.

## Input

Arguments: $ARGUMENTS

## Mode Detection

Parse $ARGUMENTS for flags:
- No flags → **Default mode** (existing behavior: validate + atomic commit)
- `--split` → **Split mode** (analyze all changes, propose N atomic commits by concern)
- `--multi` → **Multi mode** (split branch into separate feature branches + auto-create PRs)

## Step 1 — Quality pipeline

Run in order, stop on first failure:
```bash
pnpm lint
pnpm build
pnpm test
```

If any step fails: show the error, stop. Do NOT commit broken code.

## Step 2 — Review what's changed

```bash
git status
git diff --stat HEAD
```

Understand what changed so the commit message is accurate.

## Step 3 — Commit message

If `$ARGUMENTS` is provided, use that message (still validate it's conventional format).

Otherwise, generate from the diff:
```
<type>(<scope>): <description>
```

Types: `feat`, `fix`, `refactor`, `style`, `test`, `chore`, `docs`

Scope: the area of code (e.g., `workspace`, `convex`, `dashboard`, `sidebar`)

Description: present tense, lowercase, no period. Describe WHAT changed, not HOW.

Examples:
- `feat(workspace): add pause/resume mutations`
- `fix(dashboard): correct workspace card status color`
- `refactor(sidebar): extract nav item to component`
- `chore(deps): update convex to 1.x`

## Step 4 — Stage and commit

Stage files specifically — read the diff to know exactly which files changed:
```bash
git add path/to/file1.ts path/to/file2.tsx
git commit -m "$(cat <<'EOF'
feat(scope): description
EOF
)"
```

Never use `git add -A` or `git add .`.

## Step 5 — Update state files

After committing:

1. **Update `.tstack/STATE.md`** (if it exists):
   - "What's In Progress" — update with commit hash and what was committed
   - "Last activity" — timestamp + "Committed: <hash> <message>"
   - "Completed This Session" — mark relevant items done
   - Branch position — note commits ahead of remote

2. **Mark plan.md tasks** (if `.tstack/features/<n>/plan.md` or `plan.md` exists):
   - Check off completed `- [ ]` items that correspond to the committed changes
   - Only mark tasks that are fully done — don't mark partial work

## Step 6 — Confirm

Show:
```
✓ lint, build, test passed
✓ Committed: <hash> <message>
  Files: N files changed
```

Do NOT push. The user pushes when ready.

---

## Split Mode

Activated when `$ARGUMENTS` contains `--split`.

### Step 1 — Analyze changes

Dispatch the `commit-analyzer` agent to analyze `git diff` for logical groupings:

```bash
git diff HEAD
git status
```

The agent must return a proposed list of atomic commit groups, each with:
- A set of specific files
- A conventional commit message
- The concern/reason for grouping (e.g., "auth refactor", "UI fix", "config update")

### Step 2 — Present proposal

Show the proposed commit groups to the user:

```
Proposed commits:
  1. fix(auth): correct token expiry check
     Files: src/auth/token.ts, src/auth/middleware.ts

  2. feat(dashboard): add workspace status card
     Files: src/components/WorkspaceCard.tsx, src/styles/card.css

  3. chore(deps): bump convex to 1.8.0
     Files: package.json, pnpm-lock.yaml

Proceed? [y/N]
```

Wait for user approval before continuing. If the user rejects, allow them to edit the groupings or abort.

### Step 3 — Execute each commit in order

For each approved group, in sequence:

1. Stage only the specific files for that group:
   ```bash
   git add path/to/file1.ts path/to/file2.tsx
   ```
2. Run the quality pipeline:
   ```bash
   pnpm lint && pnpm build
   ```
   If this fails, stop and report — do NOT commit a broken state.
3. Commit with the conventional message:
   ```bash
   git commit -m "$(cat <<'EOF'
   feat(scope): description
   EOF
   )"
   ```
4. Each commit must leave the codebase in a valid, buildable state.

### Step 4 — Confirm

After all commits are created, show a summary:

```
✓ 3 commits created:
  abc1234 fix(auth): correct token expiry check
  def5678 feat(dashboard): add workspace status card
  ghi9012 chore(deps): bump convex to 1.8.0
```

Do NOT push. The user pushes when ready.

---

## Multi Mode

Activated when `$ARGUMENTS` contains `--multi`.

### Step 1 — Analyze for feature boundaries

Dispatch the `commit-analyzer` agent with multi-feature detection enabled. Analyze `git diff` and `git log` to identify distinct features or concerns that should live on separate branches:

```bash
git diff main...HEAD
git log main...HEAD --oneline
git status
```

The agent must return a list of features, each with:
- A short slug (used as the branch name suffix, e.g., `auth-refactor`)
- A PR title and description
- The specific files and/or commits that belong to it

### Step 2 — Present proposal

Show the detected features to the user:

```
Detected features:
  1. auth-refactor
     Branch: feature/auth-refactor
     Files: src/auth/token.ts, src/auth/middleware.ts
     PR: "refactor(auth): simplify token validation"

  2. dashboard-status-card
     Branch: feature/dashboard-status-card
     Files: src/components/WorkspaceCard.tsx, src/styles/card.css
     PR: "feat(dashboard): add workspace status card"

Proceed? [y/N]
```

Wait for user approval before continuing.

### Step 3 — Record current branch

```bash
git branch --show-current
```

Store this as the "return branch" to restore at the end.

### Step 4 — For each detected feature

Repeat the following for every approved feature:

a. Create a new branch from main:
   ```bash
   git checkout -b feature/{name} main
   ```

b. Stage and cherry-pick or apply the relevant files. If working from uncommitted changes, use stash + selective apply:
   ```bash
   git checkout main -- path/to/relevant/file.ts
   git add path/to/relevant/file.ts
   git commit -m "feat(scope): description"
   ```

c. Push the branch:
   ```bash
   git push -u origin feature/{name}
   ```

d. Create a PR:
   ```bash
   gh pr create --title "feat(scope): description" --body "$(cat <<'EOF'
   ## Summary
   - <bullet describing the change>

   ## Test plan
   - [ ] Lint and build pass
   - [ ] Manual smoke test
   EOF
   )"
   ```

### Step 5 — Return to original branch

```bash
git checkout <return-branch>
```

### Step 6 — Present summary

Show a table of all branches and PRs created:

```
✓ Multi-mode complete:

  Branch                        PR
  ─────────────────────────────────────────────────────────────
  feature/auth-refactor         https://github.com/.../pull/42
  feature/dashboard-status-card https://github.com/.../pull/43

Returned to: main
```
