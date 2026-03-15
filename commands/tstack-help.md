---
name: tstack-help
description: List all available tstack commands, agents, and skills, grouped by tier.
argument-hint: ""
model: haiku
---

List all available tstack commands, agents, and skills.

## Commands to run

```bash
ls ~/.claude/commands/tstack-*.md 2>/dev/null
ls ~/.claude/agents/tstack-*.md 2>/dev/null
ls ~/.claude/skills/tstack-*/SKILL.md 2>/dev/null
ls .claude/commands/tstack-*.md 2>/dev/null
ls .claude/agents/tstack-*.md 2>/dev/null
ls .claude/skills/tstack-*/SKILL.md 2>/dev/null
```

## Output format

```
tstack v<version> — personal dev framework for Claude Code

━━━ TIER 1: TASK ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-task <what>        Quick fix → commit (no PR)

━━━ TIER 2: FEATURE ━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-feature <what>     Full pipeline → PR
/tstack-context <what>     Gather context
/tstack-plan <what>        Create implementation plan
/tstack-implement <plan>   Route to agents + execute

━━━ TIER 3: MISSION ━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-mission <desc>     Start multi-feature mission
/tstack-scope [feature]    Research a feature
/tstack-execute [plan]     Execute feature plan
/tstack-verify [feature]   Verify success criteria
/tstack-next               Ship feature + advance

━━━ QUALITY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-validate           lint + build + test
/tstack-test [mode]        Browser tests (playwright/dogfood)
/tstack-review [scope]     Code review
/tstack-audit [scope]      Security + performance audit (skill)

━━━ GIT ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-commit [msg]       Validate + commit
/tstack-ship [title]       Pipeline + PR
/tstack-undo [N]           Revert N commits

━━━ DEBUG & REFACTOR ━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-debug <error>      Debug + fix
/tstack-refactor <what>    Safe refactor

━━━ UTILITY ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/tstack-resume             Session continuity
/tstack-status             Quick pulse
/tstack-new-agent          Scaffold new agent
/tstack-new-command        Scaffold new command
/tstack-help               This list

━━━ SKILLS (global) ━━━━━━━━━━━━━━━━━━━━━━━━━━
[list from ~/.claude/skills/tstack-*/SKILL.md — show name + description from frontmatter]

━━━ SKILLS (this project) ━━━━━━━━━━━━━━━━━━━━
[list from .claude/skills/tstack-*/SKILL.md — show name + description]

━━━ AGENTS (this project) ━━━━━━━━━━━━━━━━━━━━
[list from .claude/agents/tstack-*.md — show name + description]

━━━ AGENTS (global) ━━━━━━━━━━━━━━━━━━━━━━━━━━
[list from ~/.claude/agents/tstack-*.md — show name + description]
```

Read the version from `~/tstack/VERSION` if it exists.
