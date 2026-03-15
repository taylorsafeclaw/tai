---
name: tstack-status
description: Quick pulse — git state, mission progress if active, one-screen output.
argument-hint: ""
model: haiku
---

Quick project status. One screen. No fluff.

## Run

```bash
git branch --show-current
git status --short
git log --oneline -3
git rev-list --count HEAD ^origin/$(git branch --show-current) 2>/dev/null || echo "0"
```

Check `.tstack/state.json` if it exists.

## Output format

```
Branch: feat/workspace-pause-resume
Ahead: 2 commits, 3 files changed

Recent commits:
  abc1234 feat(convex): add pause mutation
  def5678 fix(workspace): correct status check

Mission: workspace management (if active)
  Feature 3/7: pause/resume UI — in_progress

Open PRs: #42 feat(agents): agent CRUD
```

Keep it under 20 lines. No analysis, just facts.
