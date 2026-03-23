---
title: Quality Pipeline
---

Every tier runs a quality check after implementation. No opt-out. {% .lead %}

```
┌────────────────────────────────────────────────────┐
│  1. LINT      pnpm lint              (always)      │
│  2. BUILD     pnpm build             (always)      │
│  3. TEST      pnpm test              (always)      │
│  4. BROWSER   playwright / dogfood   (smart detect)│
│                                                    │
│  Stop on first failure. Fix. Re-run from that step.│
│  Never commit broken code.                         │
└────────────────────────────────────────────────────┘
```

---

## Core steps

### 1. Lint

`pnpm lint` — catches code style violations, unused imports, type errors. If lint fails, the pipeline stops.

### 2. Build

`pnpm build` — TypeScript compilation + framework build. Catches type errors, missing imports, broken modules.

### 3. Test

`pnpm test` — unit/integration tests. Catches logic errors and regressions.

---

## Browser testing (step 4)

Browser tests are smart-detected — tstack checks what's configured before running.

### Playwright detection

| Situation | Behavior |
|-----------|----------|
| Config + specs for touched area | Auto-run playwright |
| Config exists, no relevant specs | Skip |
| No config at all | Skip |

### When browser tests run

| Tier | Playwright | Dogfood |
|------|-----------|---------|
| Task | Never | Never |
| Feature | Auto if specs exist | Opt-in |
| Mission | Auto per feature | Opt-in per feature |

---

## Standalone commands

### `/validate`

Runs steps 1–3 (+ playwright if configured). Does not fix. Single pass. Stops on first failure.

### `/test [playwright|dogfood|all]`

Runs browser tests only (step 4). Use after code passes steps 1–3.

---

## Failure behavior

1. Show the exact error output (never summarize)
2. Stop — don't run subsequent steps
3. Fix the issue
4. Re-run from the failing step (not from step 1)
5. If still failing after one fix attempt: stop and report

The pipeline never commits broken code.

---

## Output format

```
## Validation

✓ lint — pass
✓ build — pass
✓ test — pass (47 tests)
✓ browser — pass (12 playwright tests)

### Result: PASS
Ready to commit.
```

---

## Completion Status Protocol

All pipeline runs emit a structured status:

- **DONE** — All steps completed successfully
- **DONE_WITH_CONCERNS** — Completed with caveats
- **BLOCKED** — Cannot proceed
- **NEEDS_CONTEXT** — Missing information

Skills follow a 3-strike escalation rule: after 3 failed attempts, stop and escalate.
