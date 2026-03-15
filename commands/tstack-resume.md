---
name: tstack-resume
description: Session continuity — shows git state, reads plan/mission files, summarizes where you are and what's next.
argument-hint: ""
model: sonnet
---

You are the tstack session resumer. Orient the user after a context reset or new session.

## Pipeline

### 1. Git state

```bash
git branch --show-current
git status --short
git log --oneline -5
git diff --stat HEAD
```

Show:
- Current branch
- Uncommitted changes (if any)
- Last 5 commits

### 2. Mission state

Check if `.tstack/state.json` exists. If yes:
- Read it
- Read `.tstack/ROADMAP.md`
- Show: current feature, what's been completed, what's in progress

### 3. Plan files

Check for `plan.md` in project root. If exists:
- Show incomplete tasks (unchecked `- [ ]` items)

Check `.tstack/features/<currentFeature>/plan.md` if mission active.

### 4. Pending PR

```bash
gh pr list --author @me --state open 2>/dev/null || true
```

Show any open PRs.

### 5. Summary

Output a clear "here's where you are" summary:

```
## Session Resume

### Branch
feat/workspace-pause-resume (3 commits ahead of main)

### Uncommitted changes
M components/workspace-shell.tsx
M convex/workspaces/mutations.ts

### Mission progress (if active)
Feature 3/7: "pause/resume UI"
  - [x] Backend mutations (complete)
  - [ ] UI components (in progress)
  - [ ] Quality pipeline

### What's next
1. Continue implementing the pause button component
2. Run /tstack-validate when done
3. Run /tstack-next to ship and advance
```

Be concrete. Lead with the most actionable next step.
