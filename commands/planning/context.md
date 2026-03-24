---
name: tstack:context
description: "[planning] Gather context for a task or feature — reads affected files, recent changes, conventions, gotchas. Output stays in conversation."
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

**Agent discovery:**
1. Glob `.claude/agents/*.md` to find project-specific agents (installed via templates)
2. If no project agents found, the plugin's core agents are available as fallbacks:
   - `explorer` (haiku) — read-only codebase exploration
   - `implementer` (sonnet) — generic implementation
3. Read frontmatter of discovered agents to extract: name, domain, description, model
4. Build agent roster grouped by domain

Map the task domains:
- Schema/database changes → agent with `domain: schema` (if available)
- Backend/API changes → agent with `domain: backend` (if available)
- UI/component changes → agent with `domain: frontend` (if available)
- Both domains → both agents with coordination needed
- No domain agents → implement in main context or use `implementer`

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
- List discovered agents by domain, or note "implementer" / "main context" as fallback

### Gotchas
- Any non-obvious constraints or requirements

### Suggested approach
- Brief description of how to tackle this
```

## Scope lock

This command gathers context only. Do NOT implement anything. Do NOT create files. Do NOT modify code.
