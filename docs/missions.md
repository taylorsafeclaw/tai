# Missions

A mission is a large initiative broken into a sequence of shippable features. Each feature gets its own branch and PR. State persists across Claude Code sessions via files in `.tstack/`.

## Starting a mission

```bash
/tstack-mission "build the full workspace management system"
```

Or pass a requirements doc path:
```bash
/tstack-mission docs/requirements.md
```

tstack will:
1. Read the project codebase + requirements
2. Produce `.tstack/ROADMAP.md` with numbered features
3. Create `.tstack/state.json` to track progress
4. Show the roadmap, ask for confirmation
5. Prompt you to start feature 1

---

## File structure

```
.tstack/
├── ROADMAP.md              ← feature list with goals + success criteria
├── state.json              ← progress tracker
└── features/
    ├── 1-agent-crud/
    │   └── plan.md         ← feature plan (updated as tasks complete)
    ├── 2-config-editor/
    │   └── plan.md
    └── ...
```

---

## ROADMAP.md format

```markdown
# Mission: workspace management

## Goal
Full workspace lifecycle — users can pause, resume, archive, and restore workspaces.

## Features

### Feature 1: pause/resume
**Goal:** Users can pause a running workspace (stops the machine, keeps the volume) and resume it.
**Success criteria:**
- [ ] `workspaces.pause` and `workspaces.resume` Convex mutations exist
- [ ] Pause/resume buttons appear on workspace card when in running/paused state
- [ ] Status transitions through state machine correctly
- [ ] Action log entries written on pause and resume
**Domains:** backend + frontend
**Estimated size:** medium

### Feature 2: archive/restore
**Goal:** Users can archive a paused workspace (snapshots volume, deletes Fly resources) and restore it.
**Success criteria:**
- [ ] ...
```

**Rules:**
- Each feature is independently shippable (own branch, own PR)
- Features are ordered by dependency (unblocked ones first)
- Success criteria are verifiable from the codebase — not vague
- No requirement IDs, coverage matrices, or over-engineering

---

## state.json format

```json
{
  "mission": "workspace management",
  "roadmap": ".tstack/ROADMAP.md",
  "currentFeature": 3,
  "startedAt": "2026-03-12T18:00:00Z",
  "features": {
    "1": {
      "status": "complete",
      "completedAt": "2026-03-13",
      "pr": "#42"
    },
    "2": {
      "status": "complete",
      "completedAt": "2026-03-14",
      "pr": "#45"
    },
    "3": {
      "status": "in_progress"
    },
    "4": {
      "status": "ready"
    }
  }
}
```

Status values: `ready`, `in_progress`, `complete`

---

## Working through a mission

### Standard loop

```
/tstack-scope     → research the current feature
/tstack-plan      → create plan.md, confirm with user
/tstack-execute   → Agent Team implements, atomic commits
/tstack-verify    → check success criteria + quality
/tstack-next      → PR + advance state to next feature
```

### Shortcut

```
/tstack-next      → runs verify internally, opens PR, advances, shows next feature
```

Use the shortcut when the feature is done and you just want to ship and move on.

---

## Per-feature commands

### `/tstack-scope`

Research what a feature needs before planning. Reads the ROADMAP.md goal, explores the codebase, finds:
- What already exists
- What needs building
- Patterns to follow
- Dependencies and constraints

Use before `/tstack-plan` for medium/large features.

### `/tstack-plan`

Creates `plan.md` for the current feature. See [commands.md](commands.md#tstack-plan) for the plan.md format.

The plan goes in `.tstack/features/<N>/plan.md`.

### `/tstack-execute`

Reads `plan.md`, creates an Agent Team, runs implementation, marks tasks complete. See [agent-teams.md](agent-teams.md) for coordination details.

### `/tstack-verify`

Single-pass verification against ROADMAP.md success criteria. Checks code existence and runs the quality pipeline. Reports pass/fail per criterion — does **not** fix anything.

### `/tstack-next`

The closer. Internally runs `/tstack-verify`. If pass: opens PR, updates `state.json`, shows next feature. If fail: shows what's missing, stops.

---

## Git strategy

```
main
  └── feat/pause-resume         → PR #42 (feature 1, merged)
  └── feat/archive-restore      → PR #45 (feature 2, merged)
  └── feat/billing-events       → PR #48 (feature 3, in progress)
```

Each feature gets its own branch (created automatically by `/tstack-execute` or the feature pipeline). PRs merge to main.

For large missions with many sequential features, you can optionally use a mission branch:
```
main
  └── mission/workspace-mgmt    ← optional long-lived branch
       └── feat/pause-resume    → PR into mission branch
       └── feat/archive-restore → PR into mission branch
```

---

## Resuming after a break

```
/tstack-resume
```

Shows git state, mission progress, incomplete tasks, open PRs, and the next action. Works across sessions because `.tstack/` files are the source of truth.

---

## Completing a mission

When `/tstack-next` is run on the last feature, it detects there's no next feature and prints:

```
✓ Feature 7 complete → PR #61

Mission complete! All features shipped.
Run /tstack-status to see the full summary.
```

The `.tstack/` files can be archived or left as project history.
