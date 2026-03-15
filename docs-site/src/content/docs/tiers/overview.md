---
title: "Tiers: Task / Feature / Mission"
description: The three work tiers and how to choose between them
---

tstack organizes work into three tiers based on scope and coordination needs.

```
┌──────────────────────────────────────────────────────────┐
│  TIER 1: TASK                                           │
│  Minutes. One agent. One commit. No PR.                 │
│  /tstack-task "fix the submit button color"                │
├──────────────────────────────────────────────────────────┤
│  TIER 2: FEATURE                                        │
│  Hours. Agent Team. Atomic commits. PR.                 │
│  /tstack-feature "add workspace pause/resume"              │
├──────────────────────────────────────────────────────────┤
│  TIER 3: MISSION                                        │
│  Days/weeks. Multiple features. Multiple PRs.           │
│  /tstack-mission "build workspace management"              │
└──────────────────────────────────────────────────────────┘
```

## Choosing the right tier

| Signal | Tier |
|--------|------|
| Fix a bug, change a color, rename something | Task |
| Add a feature, build a flow, connect X to Y | Feature |
| Build an entire system, rebuild from scratch | Mission |
| Touches 1–3 files, single domain | Task |
| Touches 3–10 files, may cross domains | Feature |
| Touches 10+ files, needs multiple features | Mission |
| Minutes of work | Task |
| Hours of work | Feature |
| Days or weeks of work | Mission |

When in doubt, start smaller — a task can always be promoted to a feature if it grows.

---

## Tier 1: Task

**Command:** `/tstack-task`

**Pipeline:**
```
context (fast Explore) → implement → lint + build + test → commit
```

- No plan file — too small
- No PR — commits to current branch
- No Agent Team — one agent, single shot
- No browser tests — too small to warrant it
- Stops on quality failure — never commits broken code

**Model:** sonnet

**Example:**
```
/tstack-task "fix the workspace card status badge color — it should be green for running"
```

---

## Tier 2: Feature

**Command:** `/tstack-feature`

**Pipeline:**
```
context → plan → confirm → branch → Agent Team → quality → push → PR
```

- Plan presented before any implementation starts (user confirms)
- Feature branch created automatically (`feat/<slug>`)
- Agent Team handles vertical slices (backend first, then frontend)
- Each agent commits its chunk atomically
- Full quality pipeline at the end
- PR opened via `gh pr create`

**Model:** opus (context + planning), sonnet (implementation)

**Manual decomposed flow** (step-by-step control):
```
/tstack-context "pause/resume"   → gather context
/tstack-plan "pause/resume"      → write plan, confirm
/tstack-implement plan.md        → Agent Team executes
/tstack-validate                 → quality pipeline
/tstack-commit                   → commit
/tstack-ship                     → open PR
```

**Example:**
```
/tstack-feature "add workspace pause/resume — pause stops the machine, resume restarts it"
```

---

## Tier 3: Mission

**Command:** `/tstack-mission`

**Pipeline:**
```
scope codebase → produce ROADMAP.md → create state.json → per-feature loop:
  /tstack-scope → /tstack-plan → /tstack-execute → /tstack-verify → /tstack-next → repeat
```

- ROADMAP.md defines numbered features with success criteria
- `.tstack/state.json` tracks progress across sessions
- Each feature is a self-contained tier-2 pipeline with its own branch and PR
- `/tstack-next` closes the current feature (verify → PR → advance)
- State persists across Claude Code sessions — missions can span days or weeks

**Model:** opus (mission planning + scoping), sonnet (execution), haiku (verification)

**Example:**
```
/tstack-mission "build the full workspace management system — pause, resume, archive, restore, billing"
```

See [Missions](/tiers/missions/) for the full mission workflow and state format.
