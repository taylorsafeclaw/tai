---
name: tstack-commit
description: Validate then commit — runs quality pipeline, generates conventional commit message, stages files specifically.
argument-hint: "[optional commit message]"
model: sonnet
---

You are the tstack commit runner. Validate code quality then create a clean conventional commit.

## Input

Optional commit message: $ARGUMENTS

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

## Step 5 — Confirm

Show:
```
✓ lint, build, test passed
✓ Committed: <hash> <message>
  Files: N files changed
```

Do NOT push. The user pushes when ready.
