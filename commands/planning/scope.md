---
name: tstack:scope
description: [planning] Research a mission feature before planning — reads ROADMAP.md goal, explores codebase, identifies what exists vs needs building.
argument-hint: "[feature number or name — defaults to current feature in state.json]"
model: opus
---

You are the tstack scope researcher. Understand what's needed for a specific mission feature before planning begins.

## Input

Feature to scope: $ARGUMENTS (or current feature from `.tstack/state.json` if no argument)

## Step 1 — Read mission state

Read `.tstack/state.json` and `.tstack/ROADMAP.md`.

If `$ARGUMENTS` is provided, find that feature in the roadmap. Otherwise use `currentFeature`.

Get:
- Feature goal
- Success criteria
- Domains involved

## Step 2 — Explore

Use the Agent tool for thorough codebase exploration:
- **subagent_type:** "Explore"
- **description:** "scope feature <name>"
- **prompt:** "Thoroughly investigate the codebase for feature: <feature goal>. Answer these questions:
  1. What already exists? Search for related code, components, API functions that partially implement this.
  2. What's the entry point? For backend: where would new mutations/queries live? For frontend: which page/component is the parent?
  3. Dependencies — does this feature depend on any API, schema, or component that doesn't exist yet?
  4. Patterns to follow — find the closest existing example (e.g., if adding a dialog, find an existing dialog implementation). Include file:line references.
  5. Constraints — auth requirements, schema constraints, encrypted fields, existing validators to reuse.
  Check these directories: convex/, components/, app/, lib/. Be very thorough — use thoroughness 'very thorough'."

## Step 3 — Web research (if needed)

If the feature involves an external API, library, or pattern you're not confident about:

Invoke the `research` skill (if available) or use web search directly to research the topic. Document findings in the scope output.

## Step 4 — Report

Output a scope summary in conversation:

```
## Feature: <name>

### What exists
- List of relevant existing code with file:line references

### What needs building
- List of things to create/change

### Pattern to follow
- Pointer to the closest existing example (file:line)

### Dependencies
- Anything this is blocked by (or note "none")

### Constraints
- Auth, encryption, validation requirements

### Rough task breakdown
1. Backend: ...
2. Frontend: ...
3. Tests: ...

### Estimated size
- Small (1-3 files) / Medium (3-7 files) / Large (7+ files)
```

This output is consumed by `/plan` — keep it concrete and complete.

## Scope lock

This command researches only. Do NOT implement anything. Do NOT create files. Do NOT modify code.
