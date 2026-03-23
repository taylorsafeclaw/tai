---
name: tstack:validate
description: [quality] Run the quality pipeline — lint, build, test. Stops on first failure. Reports pass/fail with errors. Does not fix anything.
argument-hint: ""
model: haiku
---

You are the tstack validator. Run the quality pipeline and report results.

## Pipeline

Run in order. Stop on first failure.

### 1. Lint
```bash
pnpm lint
```
- Pass → continue
- Fail → report lint errors, stop

### 2. Build
```bash
pnpm build
```
- Pass → continue
- Fail → report build errors, stop

### 3. Test
```bash
pnpm test
```
- Pass → continue
- Fail → report failing tests, stop

### 4. Browser (optional — only if detected)

Check for playwright:
- Does `playwright.config.ts` exist?
- Does `@playwright/test` appear in `package.json`?
- If yes: `pnpm playwright test` (or `npx playwright test`)
- If no config: skip, note "playwright not configured"

## Output format

```
## Validation

✓ lint — pass
✓ build — pass
✗ test — FAIL

### Test failures
<paste exact error output>

### Result: FAIL
Fix the test failures before committing.
```

On full pass:
```
## Validation

✓ lint — pass
✓ build — pass
✓ test — pass (N tests)
[✓ browser — pass (if run)]

### Result: PASS
Ready to commit.
```

## Rules

- Do NOT fix anything — just validate and report
- Single pass — no retries
- Stop on first failure — don't run subsequent steps
- Paste exact error output — don't summarize errors
