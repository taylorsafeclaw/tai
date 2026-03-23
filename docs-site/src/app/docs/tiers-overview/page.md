---
title: The Three Tiers
---

tstack organizes work into three tiers based on scope and coordination needs. {% .lead %}

---

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

**Command:** `/task`

**Pipeline:**

```
context (fast Explore) → implement → lint + build + test → commit
```

- No plan file — too small
- No PR — commits to current branch
- No Agent Team — one agent, single shot
- Stops on quality failure — never commits broken code

**Model:** sonnet

```shell
/task "fix the workspace card status badge color"
```

---

## Tier 2: Feature

**Command:** `/feature`

**Pipeline:**

```
context → plan → confirm → branch → Agent Team → quality → push → PR
```

- Plan presented before implementation (user confirms)
- Feature branch created automatically
- Agent Team handles vertical slices
- Each agent commits atomically
- Full quality pipeline at the end
- PR opened via `gh pr create`

**Model:** opus (context + planning), sonnet (implementation)

**Manual decomposed flow:**

```shell
/context "pause/resume"
/plan "pause/resume"
/execute plan.md
/validate
/commit
/ship
```

---

## Tier 3: Mission

**Command:** `/mission`

**Pipeline:**

```
scope → ROADMAP.md → state.json → per-feature loop:
  /scope → /plan → /execute → /verify → /next → repeat
```

- ROADMAP.md defines numbered features with success criteria
- State persists across sessions via `.tstack/state.json`
- Each feature is a self-contained tier-2 pipeline
- `/next` closes the current feature and advances

**Model:** opus (planning), sonnet (execution), haiku (verification)

```shell
/mission "build the full workspace management system"
```

See [Missions](/docs/missions) for the full workflow and state format.
