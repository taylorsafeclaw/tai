---
name: tstack:status
description: [utility] Quick pulse — git state, STATE.md position, agent activity, mission progress. One-screen output.
argument-hint: ""
model: haiku
---

Quick project status. One screen. No fluff.

## Run

Read these sources (skip any that don't exist):

1. `.tstack/STATE.md` — current position, phase, what's in progress
2. `.tstack/state.json` — mission progress (feature N of M)
3. `.tstack/AGENTS.md` — last 3 agent activity entries

4. Git state:
```bash
git branch --show-current
git status --short
git log --oneline -3
git rev-list --count HEAD ^origin/$(git branch --show-current) 2>/dev/null || echo "0"
gh pr list --author @me --state open --limit 3 2>/dev/null || true
```

## Output format

```
Branch: feat/workspace-pause-resume
Ahead: 2 commits, 3 files changed
Phase: implementing

Recent commits:
  abc1234 feat(convex): add pause mutation
  def5678 fix(workspace): correct status check

Mission: workspace management (if active)
  Feature 3/7: pause/resume UI — in_progress

In progress: Backend mutations done. UI agent failed on TaskCard props.
Next step: Fix query return type in convex/tasks/queries.ts

Recent agents:
  convex — pass, built task mutations
  ui — FAILED, type mismatch

Open PRs: #42 feat(agents): agent CRUD
```

Keep it under 25 lines. No analysis, just facts.
