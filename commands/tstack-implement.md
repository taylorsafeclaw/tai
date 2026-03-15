---
name: tstack-implement
description: Route a task or plan to the right agents and execute. Reads plan.md or takes inline description.
argument-hint: "<task description or plan.md path>"
model: sonnet
---

You are the tstack implementation router. Read the task or plan and execute it with the right agents.

## Bootstrap

Read these files first:
- `CLAUDE.md` (project root)
- `.claude/CLAUDE.md` (if exists)
- `package.json` — project scripts

## Input

Task or plan: $ARGUMENTS

## Step 1 — Read the input

If `$ARGUMENTS` ends in `.md`: read the file as a plan.
Otherwise: treat it as an inline task description.

## Step 2 — Identify domains

From the task/plan, determine what domains are involved:
- Convex schema, mutations, queries, actions → **backend**
- React components, pages, UI state, styling → **frontend**
- Config, env vars, scripts → **main context**
- Tests only → **main context**

## Step 3 — Check agent availability

Use Glob to check what's available:
```
.claude/agents/tstack-convex.md
.claude/agents/tstack-ui.md
~/.claude/agents/tstack-implementer.md
```

## Step 4 — Execute

**Single domain + domain agent available:**

Spawn the domain agent using the Agent tool:
- **prompt:** Include the full task context:
  - What to build (from task/plan)
  - Files to touch
  - Patterns to follow
  - API contracts if relevant
  - "Commit atomically. Return: what was implemented, files modified, quality result."

**Multiple domains + domain agents available:**

Sequential agent coordination:
1. Spawn backend agent first with backend tasks + full context
2. Backend agent commits and returns API shape
3. Spawn frontend agent with frontend tasks + API shape from backend
4. Frontend agent commits
5. Run quality pipeline in main context

**No domain agents available:**

Check for tstack-implementer (global agent). If available, spawn it.
If no agents at all: implement directly in main context. Follow existing patterns.

## Step 5 — Quality gate

Run in order. Stop on first failure.
1. `pnpm lint` — if project has lint script
2. `pnpm build` — if project has build script
3. `pnpm test` — if project has test script

Detection: read `package.json` scripts to verify each exists before running.
If a script doesn't exist, skip it.

On failure:
- Show exact error output (never summarize)
- Attempt one targeted fix
- Re-run the failed step only
- If still failing: stop, report, ask user

## Step 6 — Commit (if not already committed by agents)

If quality passes and there are uncommitted changes:
```
<type>(<scope>): <description>
```
Stage files specifically. Never `git add -A`.

## Error recovery

At each step, if failure occurs:
1. Log what failed — exact error, which step, which file
2. Attempt fix — one attempt only
3. Re-run from failure point
4. If still failing — stop, report, ask user
5. Never loop more than twice on the same error

## Rules

- Always check quality before committing
- Atomic commits per logical chunk
- Match existing code patterns — no new abstractions unless the plan specifies them
- If a step fails, stop and report — don't spiral through retries
