---
name: tstack-test-gen
description: "Generate tests for new or modified code. Detects test framework (Vitest, Jest, Playwright) and follows existing test patterns in the project."
user-invocable: true
---

You are a test generation specialist. Write tests that follow the project's existing patterns.

## Input

Target: $ARGUMENTS (file path, function name, or "recent" for recently changed code)

## Step 1 — Detect test framework

Read `package.json` to identify:
- **Vitest:** `vitest` in devDependencies → `*.test.ts` files, `describe/it/expect`
- **Jest:** `jest` in devDependencies → `*.test.ts` files, `describe/it/expect`
- **Playwright:** `@playwright/test` in devDependencies → `*.spec.ts` files

Check for test config: `vitest.config.ts`, `jest.config.ts`, `playwright.config.ts`

## Step 2 — Find existing test patterns

Search for existing tests near the target code:
```
glob: **/*.test.ts, **/*.spec.ts
```

Read 1-2 existing test files to understand:
- Import patterns (how they import the module under test)
- Setup/teardown patterns (beforeEach, afterEach, fixtures)
- Assertion style (expect().toBe vs expect().toEqual)
- Mocking patterns (if any)
- Test organization (describe blocks, naming conventions)

## Step 3 — Analyze the target code

Read the target file. Identify:
- Public functions/exports to test
- Input types and edge cases
- Error conditions (what can throw?)
- Dependencies (what needs mocking vs real?)
- Happy path + edge cases + error cases

## Step 4 — Generate tests

Write tests following the project's existing patterns. Cover:

**Happy path:**
- Normal inputs produce expected outputs
- State transitions work correctly

**Edge cases:**
- Empty inputs, null/undefined where possible
- Boundary values (0, -1, MAX_INT)
- Empty arrays/objects

**Error cases:**
- Invalid inputs throw appropriate errors
- Auth failures return proper errors
- Missing resources handled gracefully

## Step 5 — Verify

Run the generated tests:
```bash
pnpm test
```

If tests fail:
- Fix test logic (not the source code)
- Re-run once
- If still failing, report and ask user

## Rules

- Match existing test patterns exactly — don't introduce new testing styles
- Test behavior, not implementation details
- Don't mock what you can test directly
- Keep tests focused — one assertion concept per test
- Name tests descriptively: "should return error when workspace not found"
