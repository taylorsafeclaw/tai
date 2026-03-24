---
name: dag-execute
description: |
  Build and execute a structured dependency DAG using Claude Code Tasks from a reviewed plan.
  Takes plan items (from a spec or ad-hoc list), analyzes dependencies, creates
  real Tasks with blockedBy relationships and agent owner assignments, then generates a
  self-contained orchestration prompt. Use when: "generate execution prompt", "create DAG",
  "dag execute", "make the agent team prompt", "turn this plan into waves", "execute this
  with agents", "orchestrate these tasks", "build the execution prompt", "create task DAG",
  Use when the user says "kick off implementation", "dispatch agents", or "run the plan with agents".
allowed-tools:
  - Read
  - Grep
  - Glob
  - Bash
  - AskUserQuestion
  - TaskCreate
  - TaskUpdate
  - TaskList
  - TaskGet
---

# /dag-execute — Plan-to-DAG Builder & Orchestration Prompt Generator

You take a reviewed plan and do two things:

1. **Create real Claude Code Tasks** with `blockedBy` dependencies and `owner` assignments
   so the runtime can track progress, enforce ordering, and let agents self-coordinate.
2. **Generate a self-contained orchestration prompt** the user pastes after `/clear` to
   kick off agent team execution against those tasks.

The Tasks are the source of truth. The prompt tells the orchestrator how to read them.

## Why Both Tasks AND a Prompt?

Claude Code Tasks give you dependency tracking (`blockedBy`), ownership (`owner`), and
status progression (`pending` → `in_progress` → `completed`). But Tasks alone don't tell
agents *how* to do the work or *which specialist* to dispatch. The prompt provides the
context, file paths, code patterns, and coordination rules that make the Tasks actionable.

Without explicit `blockedBy` on Tasks, Claude defaults to flat sequential execution.
Without explicit `owner` assignments, lead agents implement instead of delegating.
Without a prompt that says "do NOT implement directly," orchestrators take shortcuts.
This skill prevents all three failure modes.

## Step 1: Gather the Plan

Find the plan items to execute. Check these sources in order:

1. **User provided items directly** — if the user said "execute these 5 things", use those
2. **Plan files** — check `plan.md`, `.tstack/features/*/plan.md`
3. **Ask the user** if nothing found

For each item, extract:
- **ID** — short slug (e.g., `O1`, `schema-prep`, `add-tests`)
- **Title** — one-line imperative description (e.g., "Add atomic concurrency to createThread")
- **Details** — what to change, which files, what the code should look like
- **Dependencies** — which other task IDs must complete first
- **Domain** — schema | backend | frontend | integration | testing | quality | infrastructure
- **Files touched** — specific file paths this task modifies

## Step 2: Build the Dependency Graph

Analyze dependencies using these rules:

| Pattern | Dependency Rule |
|---------|----------------|
| Schema change + code that uses new fields | Schema first |
| Backend API + frontend consuming it | Backend before frontend |
| Fix A uses a field/type added by fix B | B blocks A |
| Implementation + tests for it | Implementation before tests |
| Any code change + validation | All changes before validate |
| Any code change + review | All changes before review |
| Two fixes touching different files | No dependency — parallel |
| Two fixes touching the SAME file | Sequential (parallel writes can silently fail) |

### File Conflict Detection (Critical)

For every pair of tasks in the same wave, verify their file lists don't overlap. If two
tasks both touch the same file, they CANNOT run in parallel — add a dependency to
serialize them, or merge them into one task for one agent.

### Topological Sort into Waves

```
Wave 1: [tasks with zero dependencies]
Wave 2: [tasks whose blockedBy are all in Wave 1]
Wave 3: [tasks whose blockedBy are all in Waves 1-2]
...
Wave N-1: [test-writer — blocked by all implementation tasks]
Wave N:   [validate + reviewer — blocked by tests]
```

## Step 3: Map Tasks to Agents

Each agent owns specific file domains. Parallel agents MUST own different files.

Discovery: glob `.claude/agents/*.md` and read `domain:` frontmatter to find available
agents. If no project agents exist, use the agent names from the plan directly.

**Orchestrator selection:**
- Mostly bug fixes → `bugfix-lead`
- Mostly new features → `feature-lead`
- Mixed or unclear → `feature-lead` (more general)

## Step 4: Create the Tasks

Use `TaskCreate` for each task, then `TaskUpdate` to wire `addBlockedBy` and `owner`.

### Task Creation Pattern

For each task in topological order:

```
TaskCreate:
  subject: "[ID]: [Title]"
  description: |
    **Agent:** [agent-name]
    **Wave:** [N]
    **Files:** [list of file paths]
    **Commit:** [type]: [message]

    [Full implementation details — what to change, code patterns,
    file paths with line references, acceptance criteria.
    Detailed enough that the assigned agent can work without questions.]

    **Depends on:** [list of task IDs and why, or "None"]
  activeForm: "[Gerund form, e.g., 'Adding atomic concurrency']"
```

Then for each task with dependencies:
```
TaskUpdate:
  taskId: [this task's ID]
  addBlockedBy: [array of dependency task IDs]
  owner: [agent-name]
```

For tasks with no dependencies:
```
TaskUpdate:
  taskId: [this task's ID]
  owner: [agent-name]
```

### Required Metadata in Description

Every task description MUST include these fields so agents can self-coordinate:
- **Agent:** — which specialist agent should own this
- **Wave:** — which parallel wave this belongs to
- **Files:** — exact file paths this task will modify (for conflict detection)
- **Commit:** — the git commit message to use
- **Depends on:** — human-readable dependency explanation

## Step 5: Generate the Orchestration Prompt

After creating all Tasks, generate a self-contained prompt the user pastes after `/clear`.
This prompt references the Tasks by ID and tells the orchestrator how to coordinate.

### Prompt Template

````markdown
## Mission: [One-line goal]

[2-3 sentence context: what was reviewed/approved, branch name, PR number if applicable]

### How to Execute

A Task DAG has been created with [N] tasks across [M] waves. Each task has:
- `blockedBy` dependencies enforcing execution order
- `owner` set to the specialist agent that should do the work
- Full implementation details in the task description

**Your job as orchestrator:**
1. Call `TaskList` to see all tasks and their dependency status
2. Find tasks that are `pending` with empty `blockedBy` (or all blockers completed) — these are ready
3. For each ready task, read its full description via `TaskGet`
4. Dispatch the agent named in the task's **Agent:** field as a subagent with the task description as prompt
5. Dispatch independent tasks in **parallel** (same wave = safe to parallelize)
6. When a subagent completes, mark the task `completed` via `TaskUpdate`
7. Call `TaskList` again — newly unblocked tasks will appear
8. Repeat until all tasks are completed
9. If any task fails, mark it `in_progress` still, create a new task describing the blocker, and continue with other unblocked tasks

### Context Files (read before dispatching)
- **Design spec:** `[path]` — design decisions and review notes
- **Test plan:** `[path]` — test requirements for Wave N-1
[additional context files]

### Wave Summary

| Wave | Tasks | Agents | Parallel? |
|------|-------|--------|-----------|
| 1 | [IDs] | [agents] | Yes |
| 2 | [IDs] | [agents] | Yes (after Wave 1) |
| ... | ... | ... | ... |
| N | validate, reviewer | validate, reviewer | Yes (after all impl) |

### Agent Roster

- **[orchestrator]** (you): Coordinate only. Read tasks, dispatch agents, track completion.
  **CRITICAL: Do NOT implement tasks yourself. You are a coordinator, not a coder.**
  If you catch yourself writing code or editing files, STOP — dispatch the right specialist.
- **[agent-1]**: [role + file ownership]
- **[agent-2]**: [role + file ownership]
[all agents in the DAG]

### Rules
- Each task = 1 atomic git commit. Do not batch.
- Agents must not modify files outside their task's listed **Files:** scope.
- If a task is blocked or fails after 2 attempts, skip it, flag it, and continue.
- Parallel agents in the same wave MUST NOT touch the same files.
- The orchestrator reads `TaskList` after each completion to find newly unblocked work.
- When all tasks show `completed`, run the validate task as final gate.
````

## Step 6: Present and Confirm

Show the user:
1. **The wave diagram** — ASCII visualization of the DAG
2. **Task count** — how many tasks were created, how many waves
3. **The orchestration prompt** — ready to copy

Ask:
> "I've created [N] tasks across [M] waves with blockedBy dependencies.
> Here's the orchestration prompt — copy it and paste after /clear to start.
> Want to adjust anything before you kick it off?"

## Wave Diagram Format

```
WAVE 1 (parallel)          WAVE 2 (parallel)         WAVE 3           WAVE 4
──────────────────         ──────────────────         ──────           ──────
[O4/X1] schema       ───→  [O1] concurrency    ───→  [tests]    ───→  [validate]
[cleanup] plan.md          [A2] completionType        test-writer      validate
                           [C2] bot token                              reviewer
                                  │
                                  ├───→  [A1] budget (W3)
                                  └───→  [C1] defer (W3)
                                         [C4] truncate (W3)
```

## Sizing Guidance

| Plan size | Expected waves | Tasks | Approx agents |
|-----------|---------------|-------|---------------|
| 1-3 items | 2 (impl + validate) | 3-5 | 2-3 |
| 4-8 items | 3-4 | 6-12 | 4-6 |
| 9-15 items | 4-6 | 11-20 | 6-8 |
| 16+ items | Split into multiple runs | — | — |

## Edge Cases

- **All tasks sequential** — each blockedBy the previous. Still use agent dispatch for
  specialization. One task per wave.
- **No schema changes** — skip schema agent, start with implementation agents
- **Frontend-only** — use `ui` agent only, still include `test-writer` + `validate` waves
- **User gives raw list with no dependencies** — infer from file paths and domain patterns.
  Schema before backend. Backend before frontend. Implementation before tests. Flag
  inferred dependencies in the confirmation step so the user can verify.
- **Tasks already exist** — call `TaskList` first. If there are existing tasks, ask the
  user whether to clear them or build on top.
