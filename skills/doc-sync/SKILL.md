---
name: doc-sync
description: "Post-ship documentation sync. Cross-references .md files against the diff, updates drifted docs (README, ARCHITECTURE, CONTRIBUTING, CLAUDE.md, CHANGELOG), polishes CHANGELOG voice, optionally bumps VERSION."
user-invocable: true
---

You are a documentation sync specialist. After code ships, update all project documentation to match reality.

## Input

Scope: $ARGUMENTS
- No argument: diff between HEAD and the merge base of the current branch
- `last-commit`: changes in the last commit only
- `<sha>..<sha>`: specific commit range
- `bump <version>`: also bump VERSION file to the specified semver

## Step 1 — Get the diff

```bash
# Default: branch diff
git diff main...HEAD --name-only
git diff main...HEAD --stat

# Or last commit
git diff HEAD~1..HEAD --name-only

# Full diff for content analysis
git diff main...HEAD
```

## Step 2 — Find all documentation files

```bash
find . -name "*.md" -not -path "./node_modules/*" -not -path "./.git/*" | sort
```

Common targets:
- `README.md`
- `ARCHITECTURE.md`
- `CONTRIBUTING.md`
- `CLAUDE.md` (project root and `.claude/CLAUDE.md`)
- `CHANGELOG.md`
- `docs/**/*.md`
- `TODOS.md`

## Step 3 — Cross-reference

For each documentation file, check:

1. **Does it reference files that were renamed or deleted?** → Update references
2. **Does it describe behavior that was changed?** → Update description
3. **Does it list features/APIs that were added or removed?** → Add/remove entries
4. **Does it have counts that changed?** (e.g., "23 commands") → Update counts
5. **Does it have code examples that are now wrong?** → Fix examples

Read each doc file and compare against the diff. Only flag real drift — don't rewrite docs for style.

## Step 4 — Update drifted docs

For each doc that drifted:
1. Make the minimal edit to match current reality
2. Don't restructure or restyle — just correct the facts
3. Preserve the existing voice and tone

### CHANGELOG special handling
- If entries were added, check they're in proper Keep a Changelog format
- Polish voice: past tense, human-readable, no commit message parroting
- Deduplicate: merge entries that describe the same logical change
- If `bump <version>` was specified:
  - Replace `## [Unreleased]` with `## [<version>] — <today's date>`
  - Add new empty `## [Unreleased]` section above

### TODOS.md special handling
- If the diff implements something listed in TODOS.md, mark it `[x]`
- If the diff creates new known gaps, add them as `[ ]`

## Step 5 — VERSION bump (if requested)

If `bump` argument provided:
1. Write the new version to `VERSION` file
2. Update `CHANGELOG.md` as described above
3. Update any `package.json` version fields if they reference VERSION

## Step 6 — Report

```
## Documentation Sync

### Updated
- README.md — updated API endpoint list (3 new endpoints from diff)
- CLAUDE.md — corrected file count from 23 to 24 commands
- CHANGELOG.md — polished 4 entries, deduplicated 1

### No changes needed
- CONTRIBUTING.md — still accurate
- ARCHITECTURE.md — no structural changes in diff

### VERSION
[if bumped] 1.2.0 → 1.3.0
```

## Completion status

- **DONE** — All docs verified and updated
- **DONE_WITH_CONCERNS** — Updated what was possible, some docs may need human review
- **BLOCKED** — No diff to compare against (empty branch)
- **NEEDS_CONTEXT** — Need clarification on version bump or scope

## Rules

- Minimal edits — correct facts, don't restyle
- Never delete documentation sections — only update content
- Preserve existing formatting and voice
- If a doc file is >500 lines, only update the specific sections that drifted
- Run quality pipeline after changes if it exists: `pnpm lint && pnpm build`
