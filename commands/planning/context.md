---
name: tstack:context
description: [planning] Gather context for a task or feature — reads affected files, recent changes, conventions, gotchas. Output stays in conversation.
argument-hint: "<task or feature description>"
model: opus
---

You are the tstack context gatherer. Understand the landscape before work begins.

## Bootstrap

Read these files first:
- `CLAUDE.md` (project root) — project conventions
- `.claude/CLAUDE.md` (if exists) — additional instructions
- `package.json` — project type and dependencies

## Input

Task or feature: $ARGUMENTS

## Context gathering

Use the Agent tool with these parameters:
- **subagent_type:** "Explore"
- **description:** "explore context for <topic>"
- **prompt:** Include these specific investigation questions:
  1. Find all files likely touched by this task — search for function names, component names, route patterns related to "$ARGUMENTS"
  2. Check `git log --oneline -10` for recent changes in this area
  3. Read CLAUDE.md (project root and `.claude/CLAUDE.md`) for relevant conventions
  4. Find an existing example if this task follows a pattern (e.g., adding a dialog, a mutation)
  5. Surface gotchas: auth requirements, encryption, validators, schema constraints
  6. Return findings as structured file:line references

## Agent routing

After gathering context, identify which agents are relevant:

Use Glob to check what's available:
```
.claude/agents/*.md
~/.claude/agents/*.md
```

Map the task domains:
- Backend/Convex changes → `convex` (if available)
- UI/component changes → `ui` (if available)
- Both → both agents with coordination needed
- Neither domain → implement in main context or use `implementer`

## Output format

```
## Context: <task summary>

### Affected files
- path/to/file.ts:line — reason it's relevant

### Recent changes in this area
- sha: message

### Relevant patterns
- Pattern name: example at file:line

### Available agents
- convex / ui / implementer / main context

### Gotchas
- Any non-obvious constraints or requirements

### Suggested approach
- Brief description of how to tackle this
```

## Scope lock

This command gathers context only. Do NOT implement anything. Do NOT create files. Do NOT modify code.
