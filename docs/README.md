# tstack docs

Claude Code plugin — opinionated dev framework with three tiers, quality pipeline, category-namespaced commands, and agent teams.

## Contents

| Doc | What's in it |
|-----|-------------|
| [install.md](install.md) | Plugin install, uninstall, boost mode, project templates |
| [cli.md](cli.md) | Rust CLI reference (`tstack`, `tstack doctor`, etc.) |
| [tiers.md](tiers.md) | Task / Feature / Mission — when to use each |
| [commands.md](commands.md) | Full command reference (30+ commands, organized by category) |
| [agents.md](agents.md) | Core + lifecycle agents, skill preloading, model strategy |
| [skills.md](skills.md) | Skills system and all available skills |
| [hooks.md](hooks.md) | Hook scripts, hooks.json registry, configuration |
| [quality-pipeline.md](quality-pipeline.md) | Lint, build, test — how it works |
| [missions.md](missions.md) | Mission state, ROADMAP.md, state.json format |
| [agent-teams.md](agent-teams.md) | How Agent Team coordination works |
| [extensions.md](extensions.md) | All extension points and priority resolution |

## Quick reference

### CLI (outside Claude Code)

```
tstack                    → status dashboard
tstack doctor             → full diagnostic
tstack add command foo    → scaffold a new command
tstack boost / eco        → toggle model selection mode
tstack version            → version + paths
```

### Slash commands (inside Claude Code)

```
# Planning
/task "fix the thing"       → minutes, one commit
/feature "add this"         → hours, Agent Team, PR
/mission "build X system"   → days, features, PRs

# Git
/commit                     → validate then commit
/commit --split             → split changes into atomic commits
/ship                       → pipeline + PR

# Quality
/validate                   → lint + build + test
/review                     → code review
/debug                      → systematic debugging

# Lifecycle
/review-cycle 42            → ingest PR comments → fix → re-review loop
/linear                     → Linear issue pipeline

# General
/research "topic"           → deep research
/summarize                  → summarize file/diff/PR
/explain "concept"          → explain code/architecture

# Utility
/status                     → quick pulse
/resume                     → session continuity
/help                       → everything
```

### Command categories

| Category | Commands |
|----------|----------|
| git | commit, ship, undo |
| lifecycle | review-cycle, linear |
| planning | task, feature, mission, plan, scope, context, next, execute |
| quality | validate, review, debug, refactor, verify, plan-review |
| testing | test |
| general | research, simplify, audit, changelog, office-hours, summarize, explain, find-examples |
| utility | status, resume, help, new-agent, new-command |
