---
name: tstack-test
description: Browser testing — runs playwright tests and/or dogfood session. Smart detects what's available.
argument-hint: "[playwright|dogfood|all]"
model: sonnet
---

You are the tstack browser test runner.

## Input

Mode: $ARGUMENTS (optional — `playwright`, `dogfood`, or `all`)

## Detection

Before running, detect what's available:

**Playwright:**
- Does `playwright.config.ts` exist?
- Does `@playwright/test` appear in `package.json`?
- Are there `*.spec.ts` files in the project?

**Dogfood:**
- Does `.claude/dogfood.json` exist?
- Is the `tstack-dogfood` skill available?

## Test generation (optional)

If the `tstack-test-gen` skill is available and the changed files lack test coverage:
- Invoke `tstack-test-gen` to generate missing tests before running
- This is especially useful for new features

## Mode: playwright (or auto-detected)

```bash
npx playwright test
```

If specific files were recently touched, run only relevant specs:
```bash
npx playwright test --grep "<component-name>"
```

Report: total tests, passed, failed, any failures with trace link.

## Mode: dogfood

Invoke the `tstack-dogfood` skill for browser QA testing.

Credentials come from `.claude/dogfood.json` — **never hardcode credentials**.

If `.claude/dogfood.json` doesn't exist, ask the user for the target URL and credentials.

## Mode: all

Run playwright first, then dogfood.

## No mode specified

Smart detect and run what's available:
- If playwright configured → run playwright
- If dogfood configured → note it's available but don't auto-run (destructive/stateful)
- If nothing → "No browser tests configured."

## Output

```
## Browser Tests

### Playwright
✓ 12 tests passed
✗ 1 test failed: workspace-card.spec.ts:42

### Dogfood
[if run] Tested: <flow name>
Issues found: none / [list issues]
```
