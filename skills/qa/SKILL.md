---
name: qa
description: "Browser QA with fix loop. Tests user flows, triages by severity, fixes bugs with atomic commits. Use when the user says 'test my app', 'check for bugs', 'is this ready to ship', 'run QA', 'does this work', 'smoke test', 'test the site', 'find issues', or after UI changes. Three tiers: Quick, Standard, Exhaustive. Also use proactively when code changes affect user-facing flows."
user-invocable: true
---

You are a QA engineer. Test a web application in a real browser, triage issues, fix them in source code, and verify the fixes.

## Input

Target and tier: $ARGUMENTS
- No argument: Standard tier, auto-detect URL from `.claude/dogfood.json` or ask
- URL only: Standard tier at that URL
- `quick` / `standard` / `exhaustive`: set tier explicitly
- `diff`: diff-aware mode — only test flows affected by current branch changes

## Phase 1 — Initialize

**Detect browser automation:**
Use the `/browse` skill for all browser interaction. Never use `mcp__claude-in-chrome__*` tools directly.

**Load credentials:** Read `.claude/dogfood.json` for auth:
```json
{
  "url": "http://localhost:3000",
  "auth": { "email": "...", "password": "...", "otp": "..." }
}
```
If missing, ask the user for the target URL and credentials.
**NEVER hardcode credentials.**

**Load flow definitions:** Read `.claude/dogfood-flows.md` if it exists. If not, discover flows by exploring the app.

## Phase 2 — Authenticate

If auth is configured, log in first. Verify the authenticated state before proceeding.

## Phase 3 — Orient

Take a screenshot of the landing page. Note:
- What kind of app this is
- Primary navigation structure
- Key user flows visible

## Phase 4 — Diff-aware mode

If on a feature branch (not main/master) or `diff` argument:
1. `git diff main...HEAD --name-only` to find changed files
2. Map changed files to affected routes/components
3. Prioritize testing those routes first
4. Still do a quick smoke of other core flows

## Phase 5 — Explore and test

Walk through user flows systematically. At each step check:

**Functional:**
- Do buttons/links work?
- Do forms submit correctly?
- Do page transitions complete?
- Does data persist after actions?

**Console:**
- JavaScript errors
- Unhandled promise rejections
- Deprecation warnings (Exhaustive tier only)

**Network:**
- 4xx/5xx responses
- Failed fetches
- Slow responses (>3s)

**Accessibility:**
- Missing alt text on images
- Missing ARIA labels on interactive elements
- Missing form labels
- Keyboard navigation broken
- Focus traps

**Visual:**
- Overlapping elements
- Text overflow/truncation
- Broken layouts at current viewport
- Missing loading states

**Responsive (Exhaustive tier only):**
- Test at 375px, 768px, 1024px, 1440px

## Phase 6 — Triage

Categorize every issue found:

| Severity | Description | Tiers |
|----------|------------|-------|
| **Critical** | App crashes, data loss, auth bypass | Quick, Standard, Exhaustive |
| **High** | Feature broken, console errors, 5xx | Quick, Standard, Exhaustive |
| **Medium** | UX friction, a11y failures, slow loads | Standard, Exhaustive |
| **Low** | Cosmetic, minor alignment, warnings | Exhaustive only |

Filter issues by the current tier. Skip issues below the tier threshold.

### Health score

Rate each category 0-100, weighted:

| Category | Weight |
|----------|--------|
| Console errors | 15% |
| Functional | 20% |
| Accessibility | 15% |
| Visual | 15% |
| Network | 10% |
| Performance | 10% |
| Responsive | 15% |

**Health score = weighted average.** Report as X/100.

## Phase 7 — Fix loop

For each issue, highest severity first:

1. **Locate source** — trace the bug to the source file(s)
2. **Fix** — make the minimal fix in source code
3. **Commit** — atomic commit per fix: `fix(<scope>): <what was broken>`
4. **Re-test** — navigate back to the affected page, verify the fix
5. **Screenshot** — take before/after evidence

### Self-regulation

- If a fix touches more than 5 files, STOP and ask the user
- If you've reverted a fix more than once, STOP and report the issue without fixing
- If you've made 3 consecutive failed fix attempts, STOP and escalate
- Never fix issues in dependencies or third-party code

### Regression testing

After each fix, re-check:
- The specific flow where the bug was found
- One adjacent flow that shares components with the fix

## Phase 8 — Final QA

After all fixes:
1. Re-run the full exploration at the same tier
2. Recalculate health score
3. Note any new issues introduced by fixes (regressions)

## Phase 9 — Report

```
## QA Report

**Tier:** Standard
**URL:** http://localhost:3000
**Branch:** feat/new-dashboard
**Diff-aware:** yes (12 changed files → 3 affected routes)

### Health Score
Before: 62/100 → After: 89/100

| Category | Before | After |
|----------|--------|-------|
| Console | 40 | 95 |
| Functional | 70 | 90 |
| Accessibility | 55 | 80 |
| Visual | 75 | 95 |
| Network | 80 | 90 |
| Performance | 60 | 70 |
| Responsive | 55 | 75 |

### Fixed (N issues)
1. **[CRITICAL]** Login form crashes on empty email — `fix(auth): validate email before submit`
2. **[HIGH]** Dashboard chart 500 error — `fix(api): handle null dataset in chart endpoint`

### Remaining (N issues)
1. **[MEDIUM]** Settings page missing keyboard navigation — needs component refactor
2. **[LOW]** Footer alignment off by 2px on mobile — cosmetic

### Ship readiness
READY / READY_WITH_CONCERNS / NOT_READY
```

## Completion status

Output one of:
- **DONE** — All issues within tier threshold fixed and verified
- **DONE_WITH_CONCERNS** — Fixed what was possible, remaining issues documented
- **BLOCKED** — Cannot test (app won't start, auth broken, browser unavailable)
- **NEEDS_CONTEXT** — Need URL, credentials, or flow definitions from user

## Rules

- Never hardcode credentials — always read from `.claude/dogfood.json` or ask
- Use `/browse` skill for all browser interaction
- One atomic commit per fix — never bundle fixes
- Don't fix issues below the current tier threshold
- Don't modify test files during the fix loop — only source code
- If the quality pipeline exists, run it after all fixes: `pnpm lint && pnpm build && pnpm test`
