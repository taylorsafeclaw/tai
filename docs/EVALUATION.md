# tstack Repository — LLM Judge Evaluation

**Date:** 2026-03-23  
**Scope:** Structure, documentation, conventions, best practices, maintainability

---

## Evaluation Criteria & Scores

### 1. Directory Structure & Organization (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Clear separation of concerns | 5/5 | Commands, agents, skills, hooks, docs, CLI cleanly separated |
| Logical categorization | 5/5 | Commands by domain (git, lifecycle, planning, quality, etc.) |
| Discoverability | 4/5 | Plugin manifest points to dirs; agents not in manifest (convention-based) |

**Strengths:** Flat skills, category-nested commands/agents, state isolated in `.tstack/`.  
**Minor gap:** `plugin.json` omits agents path — works by convention but could be explicit.

---

### 2. Documentation Consistency (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Single source of truth | 5/5 | CLAUDE.md as canonical; install.md, docs/ align |
| Cross-references | 5/5 | docs/README.md, docs/install.md, extensions properly linked |
| No stale references | 4/5 | Fixed: setup, symlinks, haiku, superpowers removed |

**Fixes applied:** Removed legacy `./setup`, `tstack-*` prefix, superpowers, autoplan; corrected haiku→sonnet in docs.

---

### 3. Conventions & Standards (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Naming consistency | 5/5 | Commands: `tstack:<name>`, agents/skills: plain names |
| Frontmatter validation | 5/5 | CI validates required fields; script updated for recursive scan |
| Model strategy | 5/5 | sonnet/opus only; haiku deprecated with clear rationale |

**Note:** CONTRIBUTING.md and validate-frontmatter.js now match actual structure (no tstack-* requirement).

---

### 4. Build & Tooling (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| CI pipeline | 5/5 | Frontmatter check, CLI build, clippy |
| Quality gate | 5/5 | lint → build → test enforced; hooks prevent bad commits |
| .gitignore | 5/5 | Added .playwright-mcp, docs-site/.astro, .next, node_modules, .tstack |

---

### 5. Extension Points & Extensibility (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Templates | 5/5 | example + safeclaw; install scripts; project overrides |
| Hooks | 5/5 | PreToolUse guards; SubagentStop logging |
| Rules | 5/5 | Path-scoped (convex, frontend, tests) |

---

### 6. Cleanup & Hygiene (10/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Legacy removal | 5/5 | setup, uninstall bash scripts; plan.md; superpowers |
| No orphan files | 5/5 | conductor.json kept (may be external tool); no dead refs |
| Artifact handling | 5/5 | Build/cache dirs in .gitignore |

---

### 7. User Experience (9/10)

| Criterion | Score | Notes |
|-----------|-------|-------|
| Install simplicity | 5/5 | `claude plugin add` — no symlinks, no setup |
| Resume / state | 5/5 | STATE.md, DECISIONS.md, AGENTS.md well documented |
| Onboarding | 4/5 | CONTRIBUTING, docs/README, help command |

---

## Composite Score

| Category | Weight | Score | Weighted |
|----------|--------|-------|----------|
| Directory structure | 15% | 9.0 | 1.35 |
| Documentation | 20% | 9.0 | 1.80 |
| Conventions | 15% | 9.0 | 1.35 |
| Build & tooling | 15% | 9.0 | 1.35 |
| Extensibility | 10% | 9.0 | 0.90 |
| Cleanup & hygiene | 15% | 10.0 | 1.50 |
| User experience | 10% | 9.0 | 0.90 |
| **Total** | 100% | — | **9.15/10** |

---

## Remaining Improvement Opportunities

1. **plugin.json agents** — Add `"agents": "./agents"` if the plugin spec supports it for explicit discovery.
2. **cli/Cargo.lock** — Consider committing for reproducible binary builds (currently gitignored).
3. **conductor.json** — Document or remove if unused; no references in codebase.
4. **templates/safeclaw** — Project-specific; consider moving to separate repo or documenting as reference implementation.
5. **output-styles/** — Only .gitkeep; either add content or remove directory.

---

## Summary

The tstack repo now follows best practices for a Claude Code plugin: plugin-first install, consistent conventions, validated frontmatter, clean state handling, and no legacy scripts. Documentation is aligned with implementation. **Overall score: 9.15/10 — Excellent.**
