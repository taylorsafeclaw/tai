# tstack

A Claude Code plugin for structured dev workflows. Three tiers, opinionated quality pipeline, category-namespaced commands, and agent teams.

## Install

```bash
git clone https://github.com/taylorsafeclaw/tstack.git ~/tstack
claude plugin add ~/tstack
```

No symlinks. The plugin system discovers commands, agents, skills, and hooks automatically.

## The Three Tiers

```
TIER 1: TASK      → /task "fix the thing"         minutes, one commit
TIER 2: FEATURE   → /feature "add this feature"   hours, Agent Team, PR
TIER 3: MISSION   → /mission "build this system"  days, multiple features, PRs
```

| Signal | Tier |
|--------|------|
| Fix bug, change color, rename | Task |
| Add feature, build flow | Feature |
| Build entire system | Mission |

## Commands by Category

### Git
- `/commit [--split | --multi]` — validate + commit (split: atomic commits, multi: separate branches)
- `/ship` — full pipeline → PR
- `/undo` — safely rollback commits

### Lifecycle
- `/review-cycle <PR#>` — ingest PR review → fix → re-review loop
- `/linear <issue>` — Linear issue pipeline

### Planning
- `/task` — quick fix → commit (no PR)
- `/feature` — full pipeline → PR
- `/mission` — multi-feature mission
- `/plan` — create implementation plan
- `/scope` — research before planning
- `/context` — gather codebase context
- `/next` — ship feature + advance mission
- `/execute` — execute a plan with Agent Team

### Quality
- `/validate` — lint + build + test
- `/review` — code review
- `/debug` — systematic debugging
- `/refactor` — safe refactoring (greps refs first)
- `/verify` — verify against success criteria

### Testing
- `/test` — browser tests (playwright/dogfood)

### General
- `/research` — deep research (web + docs + codebase)
- `/simplify` — simplify changed code
- `/audit` — security + performance audit
- `/changelog` — generate changelog entries
- `/office-hours` — product brainstorming
- `/summarize` — summarize file/diff/PR
- `/explain` — explain code/architecture
- `/find-examples` — find usage examples

### Utility
- `/status` — quick pulse
- `/resume` — session continuity
- `/help` — list everything
- `/new-agent` — scaffold a new agent
- `/new-command` — scaffold a new command

## Quality Pipeline

Every tier runs after implementation — no opt-out:

```
pnpm lint → pnpm build → pnpm test
Stop on first failure. Never commit broken code.
```

## Agents

| Agent | Category | Model | Purpose |
|-------|----------|-------|---------|
| explorer | core | haiku | Read-only codebase exploration |
| implementer | core | sonnet | Generic implementation |
| detect | lifecycle | haiku | Detect dev lifecycle phase |
| commit-analyzer | lifecycle | sonnet | Analyze diffs for logical groupings |
| review-ingester | lifecycle | sonnet | Parse + classify PR review comments |
| fix-implementer | lifecycle | sonnet | Implement single review fix |

## Skills

21 skills — domain knowledge modules loaded by commands and agents:

| Skill | Type | Description |
|-------|------|-------------|
| audit | user-invocable | Security + performance audit |
| changelog | user-invocable | Generate changelog entries |
| commit | user-invocable | Smart commit modes |
| deps | user-invocable | Dependency analysis |
| env | user-invocable | Environment config validation |
| false-positives | internal | Review false positive filter |
| frontend-design | user-invocable | UI design audit |
| linear-mappings | internal | Linear field mappings |
| migrate | user-invocable | Database migration planner |
| office-hours | user-invocable | Product brainstorming |
| pr-body | internal | PR description generator |
| research | user-invocable | Deep research |
| review-config | internal | Review action config |
| review-cycle | user-invocable | Review cycle pipeline |
| simplify | user-invocable | Code simplification |
| test-gen | user-invocable | Test generation |

## CLI (optional)

```bash
cd ~/tstack/cli && cargo install --path .
```

```bash
tstack                    # status dashboard
tstack doctor             # full diagnostic
tstack add command foo    # scaffold a new command
tstack boost / eco        # toggle model selection mode
tstack version            # version + paths
```

## Extension Points

| Extension | Status | Location |
|-----------|--------|----------|
| Commands | Active | `commands/<category>/<name>.md` |
| Agents | Active | `agents/<category>/<name>.md` |
| Skills | Active | `skills/<name>/SKILL.md` |
| Hooks | Active | `hooks/hooks.json` + `hooks/*.js` |
| Rules | Active | `rules/` (path-scoped) |
| Settings | Active | `settings.json` |
| MCP | Scaffolded | `.mcp.json` |
| LSP | Scaffolded | `.lsp.json` |
| Output Styles | Scaffolded | `output-styles/` |

## Docs

Full documentation in [docs/](docs/README.md).

## License

MIT
