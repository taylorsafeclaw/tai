---
name: commit
description: "Smart git commit with three modes: atomic (commit logical chunks during work), split (analyze all changes and create atomic commits at end of feature), and multi (split a branch with multiple features into separate branches with auto-created PRs). Use when asked to 'commit', 'split commits', 'atomic commit', 'organize changes', 'split this branch', or 'create separate PRs for each feature'."
user-invocable: true
---

Smart commit tool with three modes. Parse arguments to determine mode.

## Mode Detection

| Input | Mode |
|-------|------|
| `/commit` (no args) | **Atomic** — commit current changes as logical chunks |
| `/commit --split` | **Split** — analyze all uncommitted changes, group by concern, create N atomic commits |
| `/commit --multi` | **Multi** — analyze branch with multiple features, split into separate branches + auto-create PRs |

## Mode A: Atomic (Default)

1. Run `git status` and `git diff HEAD` to see all changes
2. Analyze changes for logical groupings (by directory, module, concern)
3. **Single concern:** stage specific files by name, commit with conventional-commit message
4. **Multiple concerns:** stage first group, commit, repeat for each group
5. Present each proposed commit message before creating it

**Rules:**
- Never use `git add .` or `git add -A` — always stage specific files by name
- Conventional commit format: `type(scope): description`
- Match the project's existing commit style from `git log --oneline -10`
- Never amend unless explicitly asked

## Mode B: Split (`--split`)

Analyze all uncommitted changes and split into logical atomic commits.

1. `git diff HEAD` to see all uncommitted changes
2. Analyze file clusters using:
   - **Directory grouping** — files in the same directory/module
   - **Import chains** — files that import each other
   - **Test pairs** — test files matched with their source files
   - **Change type** — schema vs API vs UI vs config
3. Present proposed split to user:
   ```
   I see N logical groups:
   1. [type]: description (file1, file2, ...)
   2. [type]: description (file3, file4, ...)
   Proceed? [y/n]
   ```
4. On user approval, commit in dependency order:
   - Data/schema layer first
   - Backend/API second
   - Frontend/UI third
   - Config/docs last

## Mode C: Multi-Feature (`--multi`)

Split a branch containing multiple features into separate branches with PRs.

**Prerequisite:** All changes must be committed. If working tree is dirty, prompt user to commit first.

Read `references/multi-feature-strategy.md` for the detailed algorithm.

**Summary:**
1. Analyze `git log main..HEAD --stat` and `git diff main..HEAD` for full branch picture
2. Identify feature boundaries using file clusters, module boundaries, and dependency analysis
3. Present proposed feature groupings with file lists for user confirmation
4. On approval: create branch per feature from main, cherry-pick relevant commits, auto-create PR via `gh pr create`
5. Report all created branches and PR URLs

## Constraints

- Always present groupings for user confirmation before acting
- Commit messages follow conventional commit style
- Never amend unless asked
- Never force push
- Never skip hooks
- Stage files by name, never `git add .`

## Completion Status

- **DONE** — All commits created successfully
- **DONE_WITH_CONCERNS** — Commits created but some changes couldn't be cleanly separated
- **BLOCKED** — No changes to commit or dirty working tree for --multi mode
- **NEEDS_CONTEXT** — Need user input on ambiguous groupings
