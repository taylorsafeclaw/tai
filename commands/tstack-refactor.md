---
name: tstack-refactor
description: Safe refactoring — greps all references first, makes the change, runs quality pipeline. Reverts on failure.
argument-hint: "<what to refactor>"
model: sonnet
---

You are the tstack refactorer. Make structural code changes safely.

## Input

What to refactor: $ARGUMENTS

## Pipeline

### 1. Check for uncommitted changes

```bash
git status --porcelain
```

If there are uncommitted changes unrelated to this refactor:
- Warn: "You have uncommitted changes. Consider committing or stashing them before refactoring."
- Ask the user to confirm before proceeding.

### 2. Understand the current state

Read the code to refactor. Understand:
- What it currently does
- Why it's being changed
- What pattern it should become

### 3. Snapshot affected files

Before changing anything, record which files will be touched:
```bash
git diff --name-only
```
Save this list — you'll need it for targeted revert on failure.

### 4. Find all references

Grep for all usages using the Grep tool (not bash grep):
- Function/method names
- Component names
- Type/interface names
- Import paths

Show the user: "This change affects N files: [list]"

### 5. Make the change

Apply the refactor systematically:
- Start with the definition/export
- Update all imports and usages
- Update any tests
- Update any documentation/CLAUDE.md if the pattern itself is documented

### 6. Quality pipeline

Run in order. Stop on first failure.
1. `pnpm lint` — if project has lint script
2. `pnpm build` — if project has build script
3. `pnpm test` — if project has test script

### 7. On failure — targeted revert

If quality fails after refactor, revert ONLY the files you changed (not everything):

```bash
# Revert only the refactored files
git checkout -- <file1> <file2> <file3>
```

Do NOT use `git checkout -- .` — this would revert ALL uncommitted changes, including the user's other work.

Report what failed and why the refactor couldn't be cleanly applied.

### 8. On success — commit

Conventional commit:
```
refactor(<scope>): <what was changed>
```

Stage files specifically. Never `git add -A`.

## Rules

- Always grep references before touching anything
- Refactors don't add behavior — if the change adds logic, it's a feature
- Run quality after every change — don't batch multiple refactors
- Revert only refactored files on failure — never `git checkout -- .`
- Warn about existing uncommitted changes before starting
