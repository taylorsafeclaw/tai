<!-- /autoplan restore point: /Users/taylorallen/.gstack/projects/taylorsafeclaw-tstack/main-autoplan-restore-20260323-021323.md -->
# Plan: Fix SafeClaw Template Install Script

## Problem

The SafeClaw template at `templates/safeclaw/` has 14 agents (unprefixed: `convex.md`, `ui.md`, etc.) but the install script globs for `tstack-*.md` — matching zero files. No agents get copied when running the install.

**Root cause:** The install script was copied from `templates/example/install` which uses a `tstack-*.md` glob pattern, but the SafeClaw agents use unprefixed filenames per CLAUDE.md conventions ("No `tstack-` file prefix required — namespacing comes from the plugin system and directory structure").

**Not the problem:** Agent team coordination (`/feature`, `/execute`) discovers agents by globbing `.claude/agents/*.md` and matching the `domain:` frontmatter field — filename prefix is irrelevant to discovery.

**Scope:** This is a template installer compatibility fix for unprefixed template files. It does NOT change the repo-wide naming convention — the CLI scaffolder (`tstack add`) still uses `tstack-` prefix for tstack's own plugin components, which is a separate context (plugin components vs project-level agents).

## Approach

1. Fix the install script glob pattern from `tstack-*.md` to `*.md`
2. Add zero-copy guard — the current script silently succeeds when zero files match, which is how this bug escaped unnoticed
3. Fix both safeclaw and example install scripts for consistency
4. Update agent `name:` fields to `tstack:` namespace (e.g., `name: tstack:convex`) for future CLI dispatch readiness

## Tasks

- [ ] Fix `templates/safeclaw/install:25` — change `tstack-*.md` to `*.md` for agents glob
- [ ] Fix `templates/safeclaw/install:33` — change `tstack-*.md` to `*.md` for commands glob (no-op today — no commands dir exists — but prevents future bug)
- [ ] Fix `templates/example/install:25` — same fix for consistency
- [ ] Fix `templates/example/install:33` — same fix for consistency
- [ ] Add zero-copy guard to both install scripts — warn when agent/command dirs exist but zero files are copied
- [ ] Update agent `name:` fields in all 14 safeclaw template agents to `tstack:` namespace (e.g., `name: tstack:convex`, `name: tstack:ui`)
- [ ] Verify: run install from safeclaw, confirm all 14 agents copied
- [ ] Verify: `/feature` agent discovery finds agents by `domain:` field

## What stays untouched

- Agent filenames — stay unprefixed per CLAUDE.md conventions
- Agent `name:` fields — updated to `tstack:` namespace for CLI dispatch readiness
- Agent `domain:` fields — already correct for team coordination
- CLI scaffolder (`tstack add`) — still uses `tstack-` prefix for plugin components (different scope)
- `gsd-*` agents in safeclaw — separate framework, not copied by template
- All skills in safeclaw — work independently

## Trade-offs

- **Why `*.md` not `tstack-*.md`?** Project-level agents don't need the prefix. Agent discovery is domain-based. Prefix adds complexity for no functional benefit at the project level.
- **Why fix example template too?** Consistency — new templates based on example would inherit the same bug.
- **Why zero-copy guard?** The current `[ -f "$src" ] || continue` suppresses empty globs silently. This is the root cause of the bug going unnoticed.

## NOT in scope

- Renaming agent files to `tstack-*` — contradicts CLAUDE.md for project agents, unnecessary for discovery
- ~~Updating agent `name:` fields to `tstack:` namespace~~ — INCLUDED per user approval
- Removing old agents from safeclaw — separate operational task
- Updating CLI scaffolder prefix convention — different scope (plugin components vs project agents)

## What already exists

| Sub-problem | Existing code | Status |
|---|---|---|
| Agent template files | `templates/safeclaw/agents/` — all 14 agents | Complete |
| Install script structure | `templates/safeclaw/install` | Working except glob pattern |
| Agent discovery by domain | `/feature`, `/execute` commands | Working |
| Example template | `templates/example/` with `tstack-example.md` | Has same glob bug |

<!-- AUTONOMOUS DECISION LOG -->
## Decision Audit Trail

| # | Phase | Decision | Principle | Rationale | Rejected |
|---|-------|----------|-----------|-----------|----------|
| 1 | CEO/0A | Premise rejected — rewrite plan around actual bug | P5 (explicit) | Agent discovery is domain-based, not name-based. tstack- prefix is unnecessary for project agents. | Original 9-agent rename plan |
| 2 | CEO/0C-bis | Choose Approach A (fix install script) | P5 (explicit), P3 (pragmatic) | One-line fix vs 18+ file rename. Follows CLAUDE.md conventions. | Approach B (rename agents), Approach C (namespace prep) |
| 3 | CEO/0D | Mode: SELECTIVE EXPANSION | P6 (action) | Enhancement to existing system. Hold scope + cherry-pick. | EXPANSION, HOLD, REDUCTION |
| 4 | CEO/0D-exp1 | DEFER namespace prep (tstack: prefix in name fields) | P3 (pragmatic) | Not needed until CLI dispatch (Phase 2-3). TASTE DECISION. | Add now |
| 5 | CEO/0D-exp2 | VERIFY 5 extra agents in template | P1 (completeness) | Template has 14 agents, plan covered 9. Check all belong. | Ignore extras |
| 6 | CEO/0D-exp3 | YES — verify install end-to-end | P1 (completeness) | Must confirm fix works, not just looks right. | Skip verification |
| 7 | CEO/Sec3 | No security issue with broader glob | P5 (explicit) | Template dirs are author-controlled, not user input. | Restrict glob |
| 8 | ENG/Codex | Add zero-copy guard to install scripts | P1 (completeness) | Prevents this class of bug — silent success on zero matches is the root cause of it going unnoticed | Skip guard |
| 9 | ENG/Codex | Keep commands glob fix (no-op but preventive) | P5 (explicit) | No commands dir today, but fix prevents future bug if commands are added | Remove commands fix |
| 10 | ENG/Codex | Frame as installer fix, not convention change | P5 (explicit) | CLI scaffolder still uses tstack- prefix for plugin components — different scope | Claim repo-wide convention change |
| 11 | GATE | User approved namespace prep (tstack: prefix in name fields) | User decision | Cheap future-proofing for CLI dispatch. 14 frontmatter edits. | Defer |

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAN (via autoplan) | Premise rejected, plan rewritten. Scope reduced 18+ files → 2 files. |
| Codex Review | `/codex review` | Independent 2nd opinion | 1 | ISSUES_FOUND | 3 valid findings: zero-copy guard, framing, commands glob. All incorporated. |
| Eng Review | `/plan-eng-review` | Architecture & tests | 1 | CLEAN (via autoplan) | 0 issues. Test plan written. 0 critical gaps. |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | No UI scope detected. |

**VERDICT:** APPROVED — all reviews clean, codex findings incorporated, 1 taste decision resolved by user (namespace prep included).
