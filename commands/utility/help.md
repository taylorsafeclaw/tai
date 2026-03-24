---
name: tstack:help
description: "[utility] List all available tstack commands, agents, and skills, grouped by category."
argument-hint: ""
model: sonnet
---

List all available tstack commands, agents, and skills.

## Output format

```
tstack v0.3.0 — Claude Code plugin for structured dev workflows

━━━ GIT ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/commit [--split | --multi]   Validate + commit (split: atomic commits, multi: separate branches)
/ship [title]                 Pipeline → PR
/undo [N]                     Revert N commits

━━━ LIFECYCLE ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/review-cycle <PR#>           Ingest review → fix → re-review loop
/linear <issue>               Linear issue pipeline

━━━ PLANNING ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/task <what>                  Quick fix → commit (no PR)
/feature <what>               Full pipeline → PR
/mission <desc>               Start multi-feature mission
/plan <what>                  Create implementation plan
/scope [feature]              Research a feature
/context <what>               Gather context
/next                         Ship feature + advance
/execute [plan]               Execute feature plan

━━━ QUALITY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/validate                     Lint + build + test
/review [scope]               Code review
/debug <error>                Debug + fix
/refactor <what>              Safe refactor
/verify [feature]             Verify success criteria
/plan-review                  Review a plan before executing

━━━ TESTING ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/test [mode]                  Browser tests (playwright/dogfood)

━━━ GENERAL ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/research <topic>             Deep research (web + docs + codebase)
/simplify [scope]             Simplify changed code
/audit [scope]                Security + performance audit
/changelog [version]          Generate changelog
/office-hours [mode] <idea>   Product brainstorming
/summarize <target>           Summarize file/diff/PR
/explain <target>             Explain code/architecture
/find-examples <pattern>      Find usage examples in codebase

━━━ UTILITY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/status                       Quick pulse
/resume                       Session continuity
/help                         This list
/new-agent                    Scaffold new agent
/new-command                  Scaffold new command

━━━ AGENTS ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Glob for agent files and read their frontmatter:
1. Check `.claude/agents/*.md` for project-specific agents
2. List the plugin's core agents from `agents/core/` and `agents/lifecycle/`
3. For each agent: show name (model) — description

━━━ SKILLS ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Glob `skills/*/SKILL.md` and read frontmatter of each.
For each skill with `user-invocable: true`: show name — description
```
