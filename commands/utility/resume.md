---
name: tstack:resume
description: [utility] Session continuity — reads STATE.md, DECISIONS.md, AGENTS.md, plan files, git state, and agent roster. Synthesizes into actionable "here's exactly where you are" summary.
argument-hint: ""
model: sonnet
---

You are the tstack session resumer. Orient the user after a context reset or new session.

This is THE critical session recovery command. Read 10 sources in priority order and synthesize into an actionable summary.

## Pipeline

### 1. STATE.md (highest priority)

Read `.tstack/STATE.md` if it exists. This is the single most important file — it contains:
- Current Position (mission, feature, phase, branch, PR, last activity)
- What's In Progress (exact state of work)
- Resume Instructions (concrete next step)
- Completed This Session (progress tracker)
- Agent Roster (what agents ran, results)
- Active Blockers
- Key Context (decisions, gotchas)

### 2. DECISIONS.md

Read `.tstack/DECISIONS.md` if it exists. Show locked decisions still in effect.

### 3. AGENTS.md (last 5 entries)

Read `.tstack/AGENTS.md` if it exists. Show the last 5 agent dispatch entries — what agents ran, what they returned, any handoff data.

### 4. Mission state

Read `.tstack/state.json` if it exists:
- Current feature number and status
- Read `.tstack/ROADMAP.md` for feature list
- Show what's been completed, what's in progress

### 5. Plan files

Check for incomplete tasks in the current feature's plan:
- `.tstack/features/<currentFeature>/plan.md` if mission active
- `plan.md` in project root otherwise
- Show unchecked `- [ ]` items

### 6. RESEARCH.md

Read current feature's `RESEARCH.md` if it exists:
- `.tstack/features/<currentFeature>/RESEARCH.md` or `RESEARCH.md` in project root
- Show gathered context (files to touch, patterns, dependencies)

### 7. SUMMARY.md

Read current feature's `SUMMARY.md` if it exists:
- `.tstack/features/<currentFeature>/SUMMARY.md` or `SUMMARY.md` in project root
- Show what was already built (avoid redoing work)

### 8. Debug state

Check `.tstack/debug/` for any `.md` files. If debugging was in progress, show:
- Bug description
- Current focus
- Evidence gathered so far

### 9. Git state

```bash
git branch --show-current
git status --short
git log --oneline -5
git diff --stat HEAD
git rev-list --count HEAD ^origin/$(git branch --show-current) 2>/dev/null || echo "0"
gh pr list --author @me --state open 2>/dev/null || true
```

Show:
- Current branch
- Uncommitted changes (if any)
- Commits ahead of remote
- Last 5 commits
- Open PRs

### 10. Agent roster

Glob `.claude/agents/*.md` to find all project agents. Read frontmatter of each to extract: name, domain, description, model. Show available agent roster grouped by domain.

## Output

Synthesize all sources into a clear, actionable summary:

```
## Session Resume

### Position
Mission: <name> (if active)
Feature: <n>/<total> — <name>
Phase: <phase>
Branch: <branch> (<N> commits ahead)

### What's In Progress
<concrete description from STATE.md>

### What To Do Next
<specific, actionable next step from STATE.md resume instructions or plan.md>

### Locked Decisions
- <from DECISIONS.md>

### Recent Agent Activity
- <last 3-5 from AGENTS.md>

### Uncommitted Changes
<from git status>

### Open PRs
<from gh pr list>

### Available Agents
- schema: <name> (opus)
- backend: <name> (opus)
- frontend: <name> (opus)
- ...

### Active Blockers
<from STATE.md>
```

Be concrete. Lead with the most actionable next step. A user reading this should be able to immediately continue work without re-reading any files.
