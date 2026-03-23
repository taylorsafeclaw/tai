---
name: changelog
description: "Generate changelog entries from git commits and diffs. Follows Keep a Changelog format with version stamping. Use before shipping, creating releases, or when running '/changelog [version]'."
user-invocable: true
---

You are a changelog generator. Create human-readable changelog entries from git history.

## Input

Range or version: $ARGUMENTS (defaults to commits since last tag; if a version like "1.2.0" is passed, stamp it)

## Step 1 — Determine range

```bash
# Find last tag
git describe --tags --abbrev=0 2>/dev/null || echo "no-tags"

# Get commits since last tag (or all if no tags)
git log --oneline <last-tag>..HEAD
```

If `$ARGUMENTS` specifies a range (e.g., "v1.0.0..HEAD"), use that.
If `$ARGUMENTS` is a semver (e.g., "1.2.0"), use it as the version stamp in Step 4.

## Step 2 — Categorize commits

Read each commit message and categorize:

- **Added** — `feat:` commits, new capabilities
- **Changed** — `refactor:`, `style:` commits, behavior changes
- **Deprecated** — features marked for removal or superseded
- **Fixed** — `fix:` commits, bug fixes
- **Removed** — commits that remove features or code
- **Security** — security-related fixes

**Non-conventional commits:** If commits don't follow conventional format, read the diff (`git show <sha> --stat`) to determine the appropriate category.

Skip: `chore:`, `docs:`, `test:` commits (unless they represent user-visible changes)

## Step 3 — Write entries

Generate human-readable entries. Transform commit messages:
- Remove type prefix (`feat(scope):` → just the description)
- Write in past tense ("Added X", "Fixed Y")
- Group by category
- Include PR numbers if available
- **Deduplicate:** If multiple commits describe the same change (e.g., a feat + fix for the same feature), merge into a single entry

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

### Deprecated
- Deprecated legacy webhook format — use v2 webhooks instead
```

## Step 4 — Update CHANGELOG.md

If `$ARGUMENTS` is a version stamp (e.g., "1.2.0"):
- Replace `## [Unreleased]` with `## [1.2.0] — YYYY-MM-DD` (today's date)
- Add a new empty `## [Unreleased]` section above it

If `CHANGELOG.md` exists: insert new entries under `## [Unreleased]`
If it doesn't exist: create it with the standard header:
```markdown
# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/).

## [Unreleased]

<entries>
```

## Completion status

- **DONE** — Changelog updated with all entries
- **DONE_WITH_CONCERNS** — Updated but some commits were ambiguous to categorize
- **BLOCKED** — No commits in range to generate from
- **NEEDS_CONTEXT** — Need version number or commit range clarification

## Rules

- Human-readable — don't just copy commit messages verbatim
- Group by category, not by date or commit order
- Skip internal/chore commits unless they affect users
- Don't invent changes — only document what's in the git log
- Deduplicate — one entry per logical change, even if multiple commits
