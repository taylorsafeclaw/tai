# tstack docs

Personal dev framework for Claude Code. Three tiers, opinionated quality pipeline, plug-and-play agents, and a standalone Rust CLI.

## Contents

| Doc | What's in it |
|-----|-------------|
| [install.md](install.md) | Install, uninstall, project templates |
| [cli.md](cli.md) | Rust CLI reference (`tstack`, `tstack doctor`, etc.) |
| [tiers.md](tiers.md) | Task / Feature / Mission — when to use each |
| [commands.md](commands.md) | Full command reference (all 23 slash commands) |
| [agents.md](agents.md) | Available agents + project template agents |
| [skills.md](skills.md) | Skills system and all available skills |
| [hooks.md](hooks.md) | Hook scripts and configuration |
| [quality-pipeline.md](quality-pipeline.md) | Lint, build, test, browser — how it works |
| [missions.md](missions.md) | Mission state, ROADMAP.md, state.json format |
| [agent-teams.md](agent-teams.md) | How Agent Team coordination works |
| [extensions.md](extensions.md) | Extension system + priority resolution |

## Quick reference

### CLI (outside Claude Code)

```
tstack                    → status dashboard
tstack install            → symlink commands/agents/skills to ~/.claude/
tstack list               → pretty table of installed items
tstack doctor             → full diagnostic
tstack add command foo    → scaffold a new command
tstack version            → version + paths
```

### Slash commands (inside Claude Code)

```
/tstack-task "fix the thing"       → minutes, one commit
/tstack-feature "add this"         → hours, Agent Team, PR
/tstack-mission "build X system"   → days, features, PRs

/tstack-validate                   → lint + build + test
/tstack-commit                     → validate then commit
/tstack-ship                       → pipeline + PR

/tstack-status                     → quick pulse
/tstack-resume                     → session continuity
/tstack-help                       → everything
```
