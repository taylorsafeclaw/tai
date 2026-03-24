# Command Reference

All commands are available after installing the plugin (`claude plugin add /path/to/tstack`). Invoke with `/<name>` in Claude Code — e.g. `/task "fix the login bug"`.

---

## Git

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/commit` | Validate then commit — runs quality pipeline, generates conventional commit message, stages files specifically. | `[--split \| --multi]` | sonnet |
| `/ship` | Full pipeline → PR — validate, commit, push, create PR. Returns PR URL. | `[PR title or description]` | sonnet |
| `/undo` | Safely rollback N commits using `git revert` (not `reset --hard`). Shows what would be reverted, asks confirmation. | `[number of commits — default 1]` | sonnet |

---

## Lifecycle

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/review-cycle` | Fetch Claude Code GH Action review comments from a PR, classify with confidence scoring, create Linear tickets, fix issues, push, and re-review until clean. | `<PR#> [--ingest \| --ticket \| --fix] [--max-iter N] [--all]` | opus |
| `/linear` | Run Linear issues through tstack — fetches issue, auto-routes to the right tier, executes, and updates Linear when done. | `<issue ID or query> [--tier task\|feature] [--dry-run]` | sonnet |
| `/dag-execute` | Build and execute a structured dependency DAG from a reviewed plan. Creates Tasks with blockedBy relationships and agent assignments. | `[plan source]` | sonnet |

---

## Planning

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/task` | Tier 1 — quick atomic change → commit. No planning, no PR, no coordination. For fixes, renames, small UI tweaks. | `<what to do>` | sonnet |
| `/feature` | Tier 2 — full feature pipeline with planning and Agent Team coordination → PR. For multi-step work crossing domains. | `<feature description>` | opus |
| `/mission` | Tier 3 — start a multi-feature mission. Reads requirements, produces ROADMAP.md + state.json, then runs feature-by-feature. | `<mission description or path to requirements doc>` | opus |
| `/plan` | Plan an implementation — lightweight for small tasks, detailed plan.md for multi-file features. Asks user to confirm before proceeding. | `<task or feature description>` | opus |
| `/scope` | Research a mission feature before planning — reads ROADMAP.md goal, explores codebase, identifies what exists vs needs building. | `[feature number or name]` | opus |
| `/context` | Gather context for a task or feature — reads affected files, recent changes, conventions, gotchas. Output stays in conversation. | `<task or feature description>` | opus |
| `/next` | Advance a mission to the next feature — verifies current, opens PR, updates state.json, shows next goal. | — | sonnet |
| `/execute` | Execute a feature plan with Agent Team coordination. Reads plan.md, spawns agents, commits atomically, updates plan progress. | `[plan file path]` | sonnet |

---

## Quality

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/validate` | Run the quality pipeline — lint, build, test. Stops on first failure. Reports pass/fail with errors. Does not fix anything. | — | sonnet |
| `/review` | Code review recent changes — security, logic errors, convention violations. Single pass, high-confidence issues only. | `[files \| staged \| branch]` | sonnet |
| `/debug` | Systematic debugging — reads error, traces execution path, checks recent changes, proposes a fix. | `<error message or stack trace>` | opus |
| `/refactor` | Safe refactoring — greps all references first, makes the change, runs quality pipeline. Reverts on failure. | `<what to refactor>` | sonnet |
| `/verify` | Verify a completed feature against its success criteria. Single-pass check — reads ROADMAP.md criteria, checks codebase, runs quality pipeline. | `[feature number]` | sonnet |
| `/plan-review` | Multi-perspective plan review — CEO (scope/ambition), Eng (architecture/edge cases), Design (visual/UX). Rates dimensions 0–10, suggests improvements. | `[--ceo\|--eng\|--design] [plan file path]` | opus |

---

## Testing

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/test` | Browser testing — runs Playwright tests and/or dogfood session. Smart-detects what's available. | `[playwright\|dogfood\|all]` | sonnet |

---

## General

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/research` | Deep research a topic using web search, library docs, and codebase exploration. Outputs structured findings to research.md. | `<topic or question>` | opus |
| `/simplify` | Review changed code for reuse, quality, and efficiency, then fix issues found. | `[file path \| staged \| last-commit]` | sonnet |
| `/audit` | Security and performance audit — OWASP top 10, N+1 queries, race conditions, dependency vulnerabilities. | `[file/dir path \| full]` | opus |
| `/changelog` | Generate changelog entries from git commits and diffs. Follows Keep a Changelog format. | `[version]` | sonnet |
| `/office-hours` | Product brainstorming — Startup mode (PMF questions) or Builder mode (design thinking). | `[startup \| builder] <idea>` | opus |
| `/summarize` | Summarize a file, diff, PR, or conversation into a concise overview. | `<file path \| PR# \| diff \| conversation>` | sonnet |
| `/explain` | Explain code, architecture, or a concept in the context of this codebase. | `<file:line \| function name \| concept>` | sonnet |
| `/find-examples` | Find usage examples of a pattern, function, or API in the codebase. | `<function name \| pattern \| API>` | sonnet |

---

## Utility

| Command | Description | Argument | Model |
|---------|-------------|----------|-------|
| `/status` | Quick pulse — git state, mission progress if active, one-screen output. | — | sonnet |
| `/resume` | Session continuity — shows git state, reads plan/mission files, summarizes where you are and what's next. | — | sonnet |
| `/help` | List all available tstack commands, agents, and skills, grouped by tier. | — | sonnet |
| `/new-agent` | Scaffold a new tstack agent with proper frontmatter. Places in the right directory (global or project). | `<agent name and purpose>` | sonnet |
| `/new-command` | Scaffold a new tstack command with proper frontmatter. Places in the right directory (global or project). | `<command name and purpose>` | sonnet |
