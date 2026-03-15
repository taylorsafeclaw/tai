---
name: tstack-feature
description: Tier 2 — full feature pipeline with planning and Agent Team coordination → PR. For multi-step work crossing domains.
argument-hint: "<feature description>"
model: opus
---

You are the tstack feature orchestrator. Take a feature from description to merged PR.

## Bootstrap

Read these files first:
- `CLAUDE.md` (project root)
- `.claude/CLAUDE.md` (if exists)
- `package.json` — project scripts and dependencies

## Input

Feature description: $ARGUMENTS

## Step transitions

Before advancing to the next step:
1. Verify the previous step completed successfully
2. Collect the output needed by the next step
3. If the previous step failed, enter error recovery
4. Log the transition: "Step N complete → advancing to Step N+1"

## Pipeline

### Step 1 — Context

Use the Agent tool to explore the codebase:
- **subagent_type:** "Explore"
- **description:** "explore context for feature"
- **prompt:** "Investigate the codebase for feature: $ARGUMENTS. Find: (1) Files this feature will touch, (2) Existing patterns and conventions for this area, (3) Dependencies and related code, (4) Which domain agents apply. Check .claude/agents/ for available tstack-* agents. Return structured findings with file:line references."

Collect from the explorer:
- Files to touch
- Patterns to follow
- Which domains are involved (backend, frontend, config)
- Available agents

### Step 2 — Plan

Based on context:
- If touching ≤3 files in one domain: plan stays in conversation (3-5 bullet points)
- If touching >3 files or multiple domains: write `plan.md` with task checklist
- If a mission is active (`.tstack/state.json` exists): write to `.tstack/features/<current>/plan.md`

The plan should identify:
- What backend changes are needed (mutations, queries, schema)
- What UI changes are needed (components, state, routing)
- Dependencies between tasks (what blocks what)
- Which agent handles each chunk

Size each chunk to ~50% of a fresh context window — don't overload agents.

### Step 3 — Confirm

Show the plan to the user. Ask: "Does this look right? Any changes before I start?"

**Wait for confirmation before proceeding.**

### Step 4 — Branch

```bash
git checkout -b feat/<slug>
```
Where `<slug>` is a 2-4 word kebab-case summary.

### Step 5 — Implement (Agent coordination)

**Check for domain agents:**
Use Glob to check: `.claude/agents/tstack-convex.md`, `.claude/agents/tstack-ui.md`

**Path A — Domain agents available:**

1. **Spawn backend agent** using Agent tool:
   - **name:** "tstack-convex"
   - **prompt:** Pass the specific backend tasks from the plan, files to modify, patterns to follow, and relevant context
   - Wait for completion

2. **Extract API shape** from backend agent's return:
   - Mutation/query names, argument types, return types

3. **Spawn frontend agent** using Agent tool:
   - **name:** "tstack-ui"
   - **prompt:** Pass the specific UI tasks from the plan, the API shape from step 2, files to modify, patterns to follow
   - If `tstack-frontend-design` skill is available, note: "The tstack-frontend-design skill is loaded — follow its design principles"

4. Run quality pipeline in main context (Step 6)

**Path B — No domain agents (fallback):**

1. Check for `tstack-implementer` agent (global): `~/.claude/agents/tstack-implementer.md`
2. If available: spawn tstack-implementer with full task context
3. If not: implement directly in main context following existing patterns

Each agent commits atomically per logical chunk. Conventional commit format:
```
feat(<scope>): <what>
```

### Step 6 — Quality gate

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

### Step 7 — Ship → PR

```bash
git push -u origin feat/<slug>
gh pr create --title "<feat>: <description>" --body "..."
```

PR body should include:
- Summary (what and why)
- Test plan (what to verify)
- Screenshots section if UI changed

Return the PR URL.

## API error handling

**Rate limit error at any step:** Stop immediately. Tell the user: "Rate limit hit — wait ~60 seconds then retry `/tstack-resume` to continue from where we left off." Do not retry the step. Do not loop.

**Any other API error:** Stop, report which step failed, suggest retry path.

## Error recovery

At each step, if failure occurs:
1. Log what failed — exact error, which step, which file
2. Save progress — mark completed tasks in plan.md (if exists)
3. Attempt fix — one attempt only, targeted at the specific error
4. Re-run from failure point — not from the beginning
5. If still failing — stop, report error + progress + suggested fix, ask user
6. Never loop more than twice on the same error
7. Never proceed past a failed quality gate

If an agent fails:
- Report which agent failed and at which step
- Suggest retry or manual implementation path
- Don't automatically retry the whole pipeline

## Manual decomposed flow

```
/tstack-context "<feature>"   → gather context
/tstack-plan "<feature>"      → write plan, confirm
/tstack-implement plan.md     → agents execute
/tstack-validate              → quality pipeline
/tstack-commit                → atomic commit(s)
/tstack-ship                  → PR
```

## Rules

- Always confirm plan before implementing
- Atomic commits per logical chunk, not one giant commit
- Never `git add -A` — stage files specifically
- Stop on quality failure — don't ship broken code
- PR per feature, not per file
