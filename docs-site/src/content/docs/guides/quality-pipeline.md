---
title: Quality Pipeline
description: Automated lint, build, test, and browser checks that run after every implementation
---

Every tier runs a quality check after implementation. There's no opt-out for the core three steps.

```
┌────────────────────────────────────────────────────────────────┐
│  1. LINT      pnpm lint                    (always)           │
│  2. BUILD     pnpm build                   (always)           │
│  3. TEST      pnpm test                    (always)           │
│  4. BROWSER   playwright / dogfood         (smart detect)     │
│                                                                │
│  Stop on first failure. Fix. Re-run from that step.           │
│  Never commit broken code.                                    │
└────────────────────────────────────────────────────────────────┘
```

## Core steps

### 1. Lint — `pnpm lint`

Catches code style violations, unused imports, type errors in non-compiled code. If lint fails, the pipeline stops — don't move to build.

### 2. Build — `pnpm build`

TypeScript compilation + Next.js build. Catches type errors, missing imports, broken modules. If build fails, stop.

### 3. Test — `pnpm test`

Vitest (or Jest) unit/integration tests. Catches logic errors and regressions. If tests fail, stop.

---

## Browser testing (step 4)

Browser tests are smart-detected — tstack checks what's configured before deciding whether to run them.

### Playwright detection

Before running, tstack checks:
1. Does `playwright.config.ts` exist?
2. Is `@playwright/test` in `package.json`?
3. Are there `*.spec.ts` files for the area that was touched?

| Situation | Behavior |
|-----------|----------|
| Config + specs for touched area | Auto-run playwright |
| Config exists, no relevant specs | Skip (nothing to test) |
| No config at all | Skip, note "playwright not configured" |

### Dogfood detection

1. Does `.claude/dogfood.json` exist?
2. Does it have a target URL and auth configured?

Dogfood is never auto-run (it's stateful and interacts with a live environment). It requires explicit opt-in via `--dogfood` flag or `/tstack-test dogfood`.

### When browser tests run

| Tier | Playwright | Dogfood |
|------|-----------|---------|
| Task | Never | Never |
| Feature | Auto if specs exist for touched files | Opt-in: `--dogfood` |
| Mission | Auto per feature if specs exist | Opt-in per feature |

---

## Standalone commands

### `/tstack-validate`

Runs steps 1–3 (+ playwright if configured) and reports results. Used standalone or spawned by other commands.

Does **not** fix anything. Single pass. Stops on first failure.

### `/tstack-test [playwright|dogfood|all]`

Runs browser tests only (steps 4). Use after code is confirmed passing steps 1–3.

---

## Failure behavior

When a step fails:

1. Show the exact error output (never summarize — paste it)
2. Stop — don't run subsequent steps
3. Fix the issue
4. Re-run from the failing step (not from step 1)
5. If still failing after one fix attempt: stop and report, don't spiral

**The pipeline never commits broken code.** Commit only runs after a full pass.

---

## Output format

```
## Validation

✓ lint — pass
✓ build — pass
✗ test — FAIL

### Test failures
[exact error output]

### Result: FAIL
Fix the test failures before committing.
```

Full pass:
```
## Validation

✓ lint — pass
✓ build — pass
✓ test — pass (47 tests)
✓ browser — pass (12 playwright tests)

### Result: PASS
Ready to commit.
```
