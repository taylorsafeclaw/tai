# tstack

A lightweight, installable dev framework for Claude Code. Three tiers, opinionated quality pipeline, plug-and-play agents and skills.

## Install

```bash
git clone https://github.com/tstack-framework/tstack.git ~/tstack && ~/tstack/setup
```

Symlinks all commands, agents, and skills to `~/.claude/`. No binaries, no dependencies — pure markdown.

## The three tiers

```
TIER 1: TASK      → /tstack-task "fix the thing"         minutes, one commit
TIER 2: FEATURE   → /tstack-feature "add this feature"   hours, Agent Team, PR
TIER 3: MISSION   → /tstack-mission "build this system"  days, multiple features, PRs
```

### Choosing the right tier

| Signal | Tier |
|--------|------|
| Fix bug, change color, rename | Task |
| Add feature, build flow | Feature |
| Build entire system | Mission |
| 1–3 files | Task |
| 3–10 files | Feature |
| 10+ files | Mission |

## Workflow — end to end

### Tier 1: Task (minutes → one commit)

```bash
# In Claude Code:
/tstack-task "fix the null pointer in user.ts"

# What happens:
# 1. Reads affected files
# 2. Makes the change
# 3. Runs: pnpm lint → pnpm build → pnpm test
# 4. Commits with a conventional message
# Done. One atomic commit.
```

### Tier 2: Feature (hours → PR)

```bash
# Start with context gathering:
/tstack-context "add stripe checkout"

# Plan the implementation:
/tstack-plan "add stripe checkout"

# Review plan.md, then execute:
/tstack-execute

# Or do it all in one command:
/tstack-feature "add stripe checkout"

# What happens:
# 1. Gathers context (reads files, recent changes, conventions)
# 2. Writes plan.md with approach, file paths, trade-offs
# 3. Spawns Agent Team — explorer + implementer coordinate
# 4. Runs quality pipeline after each agent completes
# 5. Opens PR with generated description
```

### Tier 3: Mission (days → multiple PRs)

```bash
# Start a mission from requirements:
/tstack-mission "build a billing system with stripe, invoices, and usage limits"

# What happens:
# 1. Reads requirements, explores codebase
# 2. Produces ROADMAP.md (numbered features with success criteria)
# 3. Produces .tstack/state.json (progress tracker)
# 4. Runs Tier 2 on Feature 1

# After each feature merges, advance:
/tstack-next

# Research a feature before planning:
/tstack-scope

# Verify a feature against its success criteria:
/tstack-verify

# Check mission progress:
/tstack-status
```

## Commands

### Tier 1 — Task
- `/tstack-task` — quick fix/change → commit (no PR, no planning)

### Tier 2 — Feature
- `/tstack-feature` — full pipeline → PR (context + plan + Agent Team + PR)
- `/tstack-context` — gather context for a task or feature
- `/tstack-plan` — plan an implementation, write plan.md
- `/tstack-implement` — route to agents + execute
- `/tstack-execute` — execute a feature plan with Agent Team coordination

### Tier 3 — Mission
- `/tstack-mission` — start a multi-feature mission, produce ROADMAP.md + state.json
- `/tstack-scope` — research a mission feature before planning
- `/tstack-verify` — verify a feature against its success criteria
- `/tstack-next` — advance the mission to the next feature

### Quality
- `/tstack-validate` — run lint + build + test, stop on first failure
- `/tstack-test` — browser testing (playwright / dogfood session)
- `/tstack-review` — code review: security, logic errors, convention violations

### Git
- `/tstack-commit` — validate then commit (runs pipeline, generates conventional commit)
- `/tstack-ship` — full pipeline → push → PR
- `/tstack-undo` — safely rollback N commits (git revert, not reset)

### Debug & Refactor
- `/tstack-debug` — systematic debugging: reads error, traces path, proposes fix
- `/tstack-refactor` — safe refactoring: greps all refs first, makes change, runs pipeline

### Utility
- `/tstack-resume` — session continuity: shows git state, reads plan/mission files
- `/tstack-status` — quick pulse: git state, mission progress, one screen
- `/tstack-help` — list all commands, agents, and skills
- `/tstack-new-agent` — scaffold a new agent with proper frontmatter
- `/tstack-new-command` — scaffold a new command with proper frontmatter

## Quality pipeline

Every tier runs this after implementation — no opt-out:

```
1. pnpm lint    (always)
2. pnpm build   (always)
3. pnpm test    (always)
4. browser      (smart detect — playwright if configured, dogfood if --dogfood flag)

Stop on first failure. Never commit broken code.
```

## Agents

Two global agents available in every project:

| Agent | Description | Model |
|-------|-------------|-------|
| `tstack-explorer` | Codebase exploration — find files, trace paths, gather context | haiku |
| `tstack-implementer` | Generic implementation — reads CLAUDE.md, follows patterns, runs pipeline | sonnet |

## Skills

Eight built-in skills:

| Skill | Description |
|-------|-------------|
| `tstack-audit` | Security + performance audit (OWASP, N+1, race conditions) |
| `tstack-changelog` | Generate changelog entries from git commits |
| `tstack-dogfood` | Systematic browser QA session |
| `tstack-frontend-design` | UI design principles for building interfaces |
| `tstack-pr-body` | Generate PR descriptions from commits + plan |
| `tstack-research` | Deep research a topic, output to research.md |
| `tstack-simplify` | Review + simplify recently changed code |
| `tstack-test-gen` | Generate tests for new or modified code |

## CLI (optional)

The `tstack` CLI adds power tools outside of Claude Code sessions. It's optional — `./setup` works without it.

```bash
# Build from source:
cd ~/tstack/cli && cargo install --path .

# Or download a prebuilt binary from GitHub Releases
```

### CLI commands

```bash
tstack                    # status dashboard (installed items, health)
tstack install            # symlink commands + agents + skills to ~/.claude/
tstack uninstall          # remove all tstack symlinks from ~/.claude/
tstack list               # list all installed commands, agents, skills, hooks
tstack list commands      # list only commands
tstack list agents        # list only agents
tstack list skills        # list only skills
tstack doctor             # full diagnostic — checks symlinks, versions, config
tstack version            # print version, root path, claude home
tstack add command <name> # scaffold a new tstack-<name>.md command file
tstack add agent <name>   # scaffold a new tstack-<name>.md agent file
tstack add skill <name>   # scaffold a new tstack-<name>/ skill directory
tstack template list      # list available project templates
tstack template install <name>  # install a project template into .claude/
```

### Workflow shortcuts (via claude CLI)

```bash
tstack run task "fix the bug"       # equivalent to /tstack-task in Claude Code
tstack run feature "add auth"       # equivalent to /tstack-feature
tstack run mission "build X"        # equivalent to /tstack-mission
tstack run commit                   # equivalent to /tstack-commit
tstack run ship                     # equivalent to /tstack-ship
tstack run validate                 # equivalent to /tstack-validate
tstack run debug "error message"    # equivalent to /tstack-debug
```

## Project templates

Install project-specific agents and commands into your project's `.claude/` directory:

```bash
~/tstack/templates/example/install
```

Project-level files override global tstack commands when they share a name. See [docs/install.md](docs/install.md) for creating your own templates.

## Extension system

Drop a `tstack-*.md` file in any of these to extend tstack:
- `~/tstack/commands/` — core (in git)
- `~/tstack/extensions/` — personal add-ons (gitignored)
- `<project>/.claude/commands/tstack-*.md` — project-only

Project files take priority over global files. Use `/tstack-new-command` or `/tstack-new-agent` to scaffold new files.

## Uninstall

```bash
~/tstack/uninstall
```

Removes all global symlinks. Does not touch project-level files.

## Docs

- [docs/tiers.md](docs/tiers.md) — tier breakdown and decision guide
- [docs/commands.md](docs/commands.md) — all commands with full reference
- [docs/agents.md](docs/agents.md) — available agents
- [docs/skills.md](docs/skills.md) — available skills
- [docs/quality-pipeline.md](docs/quality-pipeline.md) — pipeline details
- [docs/missions.md](docs/missions.md) — mission state, ROADMAP.md, state.json
- [docs/agent-teams.md](docs/agent-teams.md) — Agent Team coordination model
- [docs/extensions.md](docs/extensions.md) — extension system + priority
- [docs/install.md](docs/install.md) — install, uninstall, templates
- [docs/cli.md](docs/cli.md) — CLI reference

## License

MIT — see [LICENSE](LICENSE).
