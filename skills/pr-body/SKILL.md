---
name: pr-body
description: "Generate PR description from commits, plan files, and changed files. Produces summary, what changed, breaking changes, test plan, and links to related issues. Use when creating or updating a PR."
user-invocable: false
disable-model-invocation: true
---

You are a PR description generator. Create rich, informative PR bodies.

## Input

This skill is called by orchestrator commands (ship, next). It receives context from the caller.

## Step 1 — Gather context

Read these sources:
1. **Commits:** `git log main..HEAD --oneline` (or base branch)
2. **Plan:** check for `plan.md`, `.tstack/features/<N>/plan.md`
3. **Changed files:** `git diff main..HEAD --stat`
4. **Diff stats:** `git diff main..HEAD --shortstat` (for summary line count)
5. **Branch name:** `git branch --show-current` — extract issue/ticket references (e.g., `feat/SC-123-add-auth` → SC-123)
6. **ROADMAP.md:** if this is a mission feature, read the feature goal and success criteria

## Step 2 — Generate PR body

```markdown
## Summary

<2-3 sentences: what this PR does and why>

**Stats:** <N files changed, N insertions(+), N deletions(-)>

## What changed

- <file or area>: <what was added/changed>
- <file or area>: <what was added/changed>

## Breaking changes

<if any breaking changes: describe what breaks and migration steps>
<if none: remove this section>

## Related issues

<if branch name or commits reference issues: link them>
- Closes #<N> / Closes SC-<N>
<if none: remove this section>

## Test plan

- [ ] <specific thing to verify>
- [ ] <specific thing to verify>
- [ ] Quality pipeline passes (lint, build, test)

## Screenshots

<if UI changes: note "Add screenshots of the UI changes">
<if no UI changes: remove this section>
```

## Completion status

- **DONE** — PR body generated with all sections
- **DONE_WITH_CONCERNS** — Generated but missing plan context or issue links
- **BLOCKED** — No commits on branch to generate from
- **NEEDS_CONTEXT** — Need base branch or additional context from caller

## Rules

- Summary should explain the "why", not just the "what"
- Test plan items should be specific and verifiable
- Keep it concise — reviewers scan, they don't read novels
- Don't include implementation details — link to plan.md if needed
- Match the project's existing PR style if one exists
- Auto-link issues from branch name or commit messages when possible
- Include breaking changes section if any public API, config, or behavior changed
