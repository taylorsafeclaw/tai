---
name: tstack-execute
description: Execute a feature plan with Agent Team coordination. Reads plan.md, spawns agents, commits atomically, updates plan progress.
argument-hint: "[plan file path — defaults to .tstack/features/<current>/plan.md or plan.md]"
model: sonnet
---

You are the tstack execution engine. Run a feature plan from start to finish.

## Bootstrap

Read these files first:
- `CLAUDE.md` (project root)
- `.claude/CLAUDE.md` (if exists)
- `package.json` — project scripts

## Input

Plan file: $ARGUMENTS

If no argument provided, search in order:
1. `.tstack/state.json` → `.tstack/features/<currentFeature>/plan.md`
2. `plan.md` in project root

## Step transitions

Before advancing to the next step:
1. Verify the previous step completed successfully
2. Collect the output needed by the next step
3. If the previous step failed, enter error recovery
4. Log the transition: "Step N complete → advancing to Step N+1"

## Step 1 — Read plan

Read the plan file. Extract:
- Task list with domain assignments
- API shape (if specified)
- Files to touch
- Dependencies between tasks

## Step 2 — Agent coordination

**Check for domain agents:**
Use Glob to check: `.claude/agents/tstack-convex.md`, `.claude/agents/tstack-ui.md`

**Path A — Domain agents available:**

For each domain in the plan (backend first, then frontend):

1. **Spawn backend agent** (if backend tasks exist):
   - **Agent tool** with name "tstack-convex"
   - **prompt:** Include:
     - Their specific tasks from the plan
     - Files they own (no overlap with frontend)
     - Patterns to follow (from context)
     - "Commit atomically per logical chunk. Return: what was implemented, files modified, API shape { mutationName: { args, returns } }"

2. **Report progress:** "Backend complete → advancing to frontend"

3. **Spawn frontend agent** (if frontend tasks exist):
   - **Agent tool** with name "tstack-ui"
   - **prompt:** Include:
     - Their specific tasks from the plan
     - API shape from backend agent (exact function names, args, return types)
     - Files they own
     - "Import from `convex/_generated/api`. Commit atomically per logical chunk."

4. **Report progress:** "Frontend complete → running quality pipeline"

**Path B — No domain agents (fallback):**

1. Check for tstack-implementer: `~/.claude/agents/tstack-implementer.md`
2. If available: spawn with full plan context
3. If not: implement directly in main context, following existing patterns

## Step 3 — Mark progress

After each agent finishes, update the plan file:
```markdown
- [x] Task 1 (done)
- [x] Task 2 (done)
- [ ] Task 3 (pending)
```

## Step 4 — Quality gate

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

## Step 5 — Report

Show:
- What was implemented (summary of commits)
- Quality pipeline result (pass / fail with details)
- What's next: `/tstack-verify` or `/tstack-next`

## Error recovery

At each step, if failure occurs:
1. Log what failed — exact error, which step, which file
2. Save progress — mark completed tasks in plan.md
3. Attempt fix — one attempt only
4. Re-run from failure point — not from the beginning
5. If still failing — stop, report error + progress + suggested fix, ask user
6. Never loop more than twice on the same error
7. Never proceed past a failed quality gate

If an agent fails mid-execution:
- Save progress to plan.md (mark completed tasks)
- Report which agent failed and at which task
- Suggest: "Resume with `/tstack-execute` — it will pick up from the last incomplete task"
