---
name: tstack-changelog
description: "Generate changelog entries from git commits. Follows Keep a Changelog format. Use before shipping or creating releases."
user-invocable: true
---

You are a changelog generator. Create human-readable changelog entries from git history.

## Input

Range: $ARGUMENTS (defaults to commits since last tag)

## Step 1 — Determine range

```bash
# Find last tag
git describe --tags --abbrev=0 2>/dev/null || echo "no-tags"

# Get commits since last tag (or all if no tags)
git log --oneline <last-tag>..HEAD
```

If `$ARGUMENTS` specifies a range (e.g., "v1.0.0..HEAD"), use that.

## Step 2 — Categorize commits

Read each commit message and categorize:

- **Added** — `feat:` commits, new capabilities
- **Changed** — `refactor:`, `style:` commits, behavior changes
- **Fixed** — `fix:` commits, bug fixes
- **Removed** — commits that remove features or code
- **Security** — security-related fixes

Skip: `chore:`, `docs:`, `test:` commits (unless they represent user-visible changes)

## Step 3 — Write entries

Generate human-readable entries. Transform commit messages:
- Remove type prefix (`feat(scope):` → just the description)
- Write in past tense ("Added X", "Fixed Y")
- Group by category
- Include PR numbers if available

Format (Keep a Changelog):
```markdown
## [Unreleased]

### Added
- Added workspace pause/resume from the dashboard (#42)
- Added channel configuration via detail sheet (#45)

### Fixed
- Fixed workspace status not updating after Fly.io deploy (#43)

### Changed
- Refactored encryption to use project-level keys (#44)
```

## Step 4 — Update CHANGELOG.md

If `CHANGELOG.md` exists: insert new entries under `## [Unreleased]`
If it doesn't exist: create it with the standard header:
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

<entries>
```

## Rules

- Human-readable — don't just copy commit messages verbatim
- Group by category, not by date or commit order
- Skip internal/chore commits unless they affect users
- Don't invent changes — only document what's in the git log
