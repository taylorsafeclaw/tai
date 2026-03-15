---
name: tstack-pr-body
description: "Generate PR description from commits, plan files, and changed files. Produces summary, test plan, and screenshots section."
user-invocable: false
---

You are a PR description generator. Create rich, informative PR bodies.

## Input

This skill is called by orchestrator commands (tstack-ship, tstack-next). It receives context from the caller.

## Step 1 — Gather context

Read these sources:
1. **Commits:** `git log main..HEAD --oneline` (or base branch)
2. **Plan:** check for `plan.md`, `.tai/features/<N>/plan.md`
3. **Changed files:** `git diff main..HEAD --stat`
4. **ROADMAP.md:** if this is a mission feature, read the feature goal and success criteria

## Step 2 — Generate PR body

```markdown
## Summary

<2-3 sentences: what this PR does and why>

## What changed

- <file or area>: <what was added/changed>
- <file or area>: <what was added/changed>

## Test plan

- [ ] <specific thing to verify>
- [ ] <specific thing to verify>
- [ ] Quality pipeline passes (lint, build, test)

## Screenshots

<if UI changes: note "Add screenshots of the UI changes">
<if no UI changes: remove this section>
```

## Rules

- Summary should explain the "why", not just the "what"
- Test plan items should be specific and verifiable
- Keep it concise — reviewers scan, they don't read novels
- Don't include implementation details — link to plan.md if needed
- Match the project's existing PR style if one exists
