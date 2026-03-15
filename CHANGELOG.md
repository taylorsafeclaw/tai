# Changelog

## [0.1.0] — 2026-03-14

First public release.

### Added
- Three-tier workflow: Task, Feature, Mission
- 23 core slash commands
- 2 global agents: `tstack-explorer`, `tstack-implementer`
- 8 global skills: `tstack-frontend-design`, `tstack-simplify`, `tstack-dogfood`, `tstack-research`, `tstack-audit`, `tstack-test-gen`, `tstack-changelog`, `tstack-pr-body`
- 4 hooks: `tstack-quality-gate.js`, `tstack-branch-guard.js`, `tstack-agent-return-validator.js`, `guard-destructive.js` (destructive command guard, default), `guard-pnpm.js` (pnpm enforcer, opt-in)
- Rust CLI (`tstack install`, `tstack uninstall`, `tstack doctor`, `tstack list`, `tstack add`)
- Project template system with `templates/example/` reference implementation
- Quality pipeline: lint → build → test → browser (smart detect)
- Agent Teams coordination for vertical slices
- 3-tier model strategy: opus thinks, sonnet builds, haiku validates
- MIT license
