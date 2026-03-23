---
name: false-positives
description: "Filter catalog for review comments — identifies pre-existing issues, linter-catchable problems, pedantic nitpicks, and other false positives that should be deprioritized or skipped during review cycle processing."
user-invocable: false
disable-model-invocation: true
---

# False Positive Filter Catalog

Use this catalog when classifying review comments to determine which should be skipped, deprioritized, or flagged as false positives.

## Category 1: Pre-existing Issues

Comments about code that existed **before** this PR. These are not regressions.

**Detection:**
- Run `git blame {file} -L {line},{line}` — if the commit is not in the PR's commit range, it's pre-existing
- Run `git diff main...HEAD -- {file}` — if the line is not in the diff hunks, it's unmodified

**Action:** Skip entirely. Do not create a ticket. Log as `SKIP: pre-existing`.

## Category 2: Linter-Catchable Issues

Issues that would be caught by the project's lint/build pipeline.

**Examples:**
- Unused imports or variables
- Missing semicolons or formatting
- Type errors that `tsc` would catch
- ESLint rule violations

**Detection:** If the issue maps to a known lint rule (e.g., `no-unused-vars`, `@typescript-eslint/no-explicit-any`), it's linter-catchable.

**Action:** Skip. The quality pipeline (`pnpm lint`) handles these. Log as `SKIP: linter-catchable`.

## Category 3: Pedantic Nitpicks

Style preferences not codified in the project's CLAUDE.md, .eslintrc, or prettier config.

**Examples:**
- "Consider using `const` instead of `let` here" (when `let` is valid)
- "This variable name could be more descriptive" (subjective)
- "I'd prefer early return here" (when both patterns exist in codebase)
- "Add a blank line before the return statement" (formatting preference)

**Detection:** Check CLAUDE.md and lint config. If the preference isn't documented as a project convention, it's a nitpick.

**Action:** Deprioritize. Create ticket only if confidence >= 75. Log as `DEPRIORITIZE: nitpick`.

## Category 4: Intentional Changes

Comments questioning changes that were explicitly part of the PR's purpose.

**Examples:**
- "Why was this function removed?" → Check PR description; if removal was the goal, skip
- "This changes the API contract" → If the PR title says "refactor API", this is intentional
- "This import was removed" → If the PR removes that dependency, intentional

**Detection:** Cross-reference the comment with the PR title and description.

**Action:** Skip. Log as `SKIP: intentional`.

## Category 5: Unmodified Lines

Comments pointing to lines the PR did not touch.

**Detection:** `git diff main...HEAD -- {file}` — check if the referenced line number falls within a diff hunk.

**Action:** Skip. Log as `SKIP: unmodified-line`.

## Category 6: Duplicate Issues

Multiple comments about the same underlying problem (e.g., same pattern repeated in 5 files).

**Detection:** Compare issue descriptions — if two comments reference the same pattern/rule, group them.

**Action:** Keep one, skip duplicates. Log duplicates as `SKIP: duplicate-of-{issue_id}`.

## Confidence Adjustment Rules

When an issue matches a false positive category, adjust confidence:

| Category | Confidence Adjustment |
|----------|----------------------|
| Pre-existing | Set to 0 (always skip) |
| Linter-catchable | Set to 0 (always skip) |
| Unmodified lines | Set to 0 (always skip) |
| Intentional | Set to 10 (almost always skip) |
| Duplicate | Set to 0 for duplicates, keep original score |
| Pedantic nitpick | Reduce by 30 (may still qualify if base was high) |
