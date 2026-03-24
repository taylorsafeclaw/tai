---
name: commit
description: "Smart git commit with three modes: atomic (commit logical chunks during work), split (analyze all changes and create atomic commits at end of feature), and multi (split a branch with multiple features into separate branches with auto-created PRs). Use when asked to 'commit', 'split commits', 'atomic commit', 'organize changes', 'split this branch', or 'create separate PRs for each feature'."
user-invocable: true
---

Smart commit tool with three modes. Parse the user's intent to determine mode.

## Mode Detection

| User intent | Mode |
|-------------|------|
| "commit" / "commit my changes" / no flags | **Atomic** — commit current changes as logical chunks |
| "split commits" / "organize changes" / `--split` | **Split** — analyze all uncommitted changes, group by concern, create N atomic commits |
| "split this branch" / "separate PRs" / `--multi` | **Multi** — split branch into separate feature branches + auto-create PRs |

## Mode A: Atomic (Default)

### 1. Quality pipeline
Run in order, stop on first failure:
```bash
pnpm lint
pnpm build
pnpm test
```
If any step fails: show the error, stop. Do NOT commit broken code.

### 2. Review changes
```bash
git status
git diff --stat HEAD
```

### 3. Commit message
Generate from the diff if not provided:
```
<type>(<scope>): <description>
```
Types: `feat`, `fix`, `refactor`, `style`, `test`, `chore`, `docs`
Scope: area of code. Description: present tense, lowercase, no period.

### 4. Stage and commit
Stage files specifically — never `git add -A` or `git add .`:
```bash
git add path/to/file1.ts path/to/file2.tsx
git commit -m "$(cat <<'EOF'
feat(scope): description
EOF
)"
```

### 5. Update state files
After committing:
- Update `.tstack/STATE.md` (if exists): "What's In Progress", "Last activity", "Completed This Session"
- Mark `plan.md` tasks (if exists): check off completed `- [ ]` items

### 6. Confirm
Show: lint/build/test status, commit hash + message, file count. Do NOT push.

## Mode B: Split (`--split`)

1. Run `git diff HEAD` and `git status` to see all changes
2. Cluster by concern:
   - **Module boundary** — files in the same directory that change together
   - **Import chain** — if file A imports file B and both changed, one concern
   - **Feature vs infrastructure** — separate feature code from config/build changes
   - **Test + implementation** — tests paired with the code they test = one commit
3. Present proposed split to user (wait for approval):
   ```
   Proposed commits:
     1. fix(auth): correct token expiry check
        Files: src/auth/token.ts, src/auth/middleware.ts
     2. feat(dashboard): add workspace status card
        Files: src/components/WorkspaceCard.tsx
   Proceed? [y/N]
   ```
4. Execute each commit in order. Between each: stage specific files, run quality pipeline, commit. Each commit must leave the codebase buildable.
5. Show summary of all commits created.

## Mode C: Multi-Feature (`--multi`)

Split a branch with multiple features into separate branches with PRs.

**Prerequisite:** All changes must be committed.

1. Analyze `git log main..HEAD --stat` and `git diff main..HEAD` for full branch picture
2. Identify feature boundaries using file clusters, module boundaries, and dependency analysis
3. Present proposed features with file lists for user confirmation
4. On approval: for each feature, create branch from main, cherry-pick relevant commits, push, create PR via `gh pr create`
5. Return to original branch. Report all branches and PR URLs.

## Constraints

- Always present groupings for user confirmation before acting
- Conventional commit messages
- Never amend unless asked, never force push, never skip hooks
- Stage files by name, never `git add .`
