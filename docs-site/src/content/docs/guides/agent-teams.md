---
title: Agent Teams
description: Multi-agent coordination for features that cross backend and frontend boundaries
---

Agent Teams coordinate multiple domain agents on a single feature. Backend finishes first and hands its API shape to the frontend agent.

## Dual-path coordination

Commands that need multi-agent coordination use a **dual-path pattern** — checking for available agents before deciding the execution strategy.

### Path A — Domain agents available

```
Orchestrator (main context)
  │
  ├── Glob check: .claude/agents/tstack-convex.md ✓
  ├── Glob check: .claude/agents/tstack-ui.md ✓
  │
  ├── Agent tool: spawn tstack-convex ─────────────────────────┐
  │     prompt: backend tasks, files to modify, patterns     │
  │     tstack-convex: reads schema, validators, auth, crypto   │
  │     tstack-convex: writes mutations + queries               │
  │     tstack-convex: runs pnpm build + pnpm test              │
  │     tstack-convex: commits atomically                       │
  │     Returns: API shape ──────────────────────────────── ┤
  │       { pauseWorkspace: { args: {id}, returns: null } }  │
  │                                                          │
  ├── (orchestrator extracts API shape)                      │
  │                                                          │
  ├── Agent tool: spawn tstack-ui ───────────────────────────── ┘
  │     prompt: UI tasks + API shape from backend
  │     tstack-ui: reads globals.css, workspace-shell, patterns
  │     tstack-ui: builds components using exact API shape
  │     tstack-ui: runs pnpm build
  │     tstack-ui: commits atomically
  │
  └── Quality gate in main context
        pnpm lint + build + test (+ playwright if configured)
```

### Path B — No domain agents (fallback)

```
Orchestrator
  │
  ├── Glob check: .claude/agents/tstack-convex.md ✗
  ├── Glob check: ~/.claude/agents/tstack-implementer.md ✓
  │
  └── Agent tool: spawn tstack-implementer
        prompt: full task context, all files, patterns
        Implements everything, runs quality, commits
```

### Path C — No agents at all

Implement directly in main context. Same patterns, same quality gate.

## Agent spawning — concrete invocations

Commands use the Agent tool with specific parameters:

```
Agent tool:
  name: "tstack-convex"
  prompt: "Implement these backend tasks:
    1. Add pause mutation to convex/workspaces/mutations.ts
    2. Add resume mutation
    3. Update status query
    Files to modify: convex/workspaces/mutations.ts, convex/workspaces/queries.ts
    Pattern to follow: existing updateWorkspace mutation at mutations.ts:45
    Commit atomically. Return: what was implemented, files modified, API shape."
```

**Key:** Prompts include specific tasks, specific files, specific patterns to follow, and what to return.

## Task board

Tasks are defined in `plan.md` with domain assignments:

```markdown
## Tasks

### Backend (tstack-convex)
- [ ] Add pause/resume mutations to convex/workspaces/mutations.ts
- [ ] Add status query updates to convex/workspaces/queries.ts
- [ ] Add indexes for new queries in schema.ts

### Frontend (tstack-ui)
- [ ] Pause button component (blocked by backend)
- [ ] Status indicator updates (blocked by backend)
- [ ] Wire to Convex mutations

### Quality
- [ ] Full pipeline: lint + build + test (blocked by frontend)
```

## API shape handoff

The orchestrator explicitly extracts and passes the API surface:

```
Backend returned:
  api.workspaces.pause({ workspaceId: Id<"workspaces"> }) → null
  api.workspaces.resume({ workspaceId: Id<"workspaces"> }) → null
  api.workspaces.get({ id }) → { status: "running" | "paused" | ... }
```

The frontend agent receives this before it starts — no guessing at mutation names or argument shapes.

## Atomic commits

Each agent commits its own chunk before returning:

```
feat(convex): add pause/resume mutations and status query
feat(workspace): add pause/resume UI controls
```

## When NOT to use multi-agent coordination

- Tasks (tier 1) — always single agent or direct, too small to coordinate
- Pure-backend features — single `tstack-convex` call
- Pure-frontend features — single `tstack-ui` call
- Config/script changes — handled in main context

Teams are for features that genuinely cross the backend/frontend boundary.

## Error recovery in agent coordination

If an agent fails mid-execution:
1. Save progress to plan.md (mark completed tasks with `[x]`)
2. Report which agent failed, at which task, with the exact error
3. Suggest: "Resume with `/tstack-execute` — it will pick up from the last incomplete task"
4. Never retry the whole pipeline automatically
