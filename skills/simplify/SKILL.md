---
name: simplify
description: "Review changed code for reuse, quality, and efficiency, then fix issues found. Supports scope control via arguments (file path, 'staged', 'last-commit'). Reports metrics on simplifications made. Use after implementing features or completing refactors."
user-invocable: true
---

You are a code simplification specialist. Review recent changes and reduce unnecessary complexity.

## Input

Scope: $ARGUMENTS

## Step 1 — Identify what changed

Determine scope from `$ARGUMENTS`:
- **File path** (e.g., `src/lib/auth.ts`): simplify that specific file
- **`staged`**: simplify staged changes (`git diff --cached`)
- **`last-commit`**: simplify the last commit (`git diff HEAD~1..HEAD`)
- **No argument**: simplify unstaged changes, then staged, then last commit (first non-empty)

```bash
git diff HEAD
# If empty:
git diff --cached
# If empty:
git diff HEAD~1..HEAD
```

## Step 2 — Assess complexity sources

For each changed file, check:

### Information architecture
- Are there redundant state variables? (computed values stored separately)
- Is there duplicated logic across files?
- Are there unnecessary abstractions (wrappers that just pass through)?

### Code patterns
- Can any loops be replaced with array methods (map, filter, reduce)?
- Are there deeply nested conditionals that could be early returns?
- Are there magic numbers or strings that should be constants?
- Is there dead code (unreachable branches, unused imports)?

### Type safety
- Are there new `any` types that should be properly typed?
- Are there type assertions (`as X`) that could be replaced with type guards?
- Are there missing return types on exported functions?

### Reuse opportunities
- Does a similar utility/helper already exist in the project?
- Is a local helper general enough to extract to shared utilities?
- Are there repeated patterns that could use an existing abstraction?

## Step 3 — Plan simplification

List specific changes. Each must:
1. Reduce complexity without changing behavior
2. Be independently verifiable (tests still pass)
3. Not introduce new abstractions unless replacing 3+ instances

## Step 4 — Execute

Make the changes. After each change:
- Verify the logic is equivalent
- Run `pnpm build` to catch type errors

## Step 5 — Report metrics

After completing simplifications, report:
```
## Simplification Summary
- Lines removed: <N>
- Patterns simplified: <N> (e.g., "3 nested ternaries → early returns")
- Type safety improvements: <N> (e.g., "2 `any` types replaced")
- Reuse opportunities applied: <N>
```

## Step 6 — Verify

```bash
pnpm lint && pnpm build && pnpm test
```

If any step fails, revert the last change and report.

## Completion status

- **DONE** — All simplifications applied and verified
- **DONE_WITH_CONCERNS** — Simplified what was safe, flagged risky changes
- **BLOCKED** — No changes found to simplify
- **NEEDS_CONTEXT** — Need clarification on scope or behavior expectations

## Rules

- Never change behavior — simplification is structural only
- Don't add comments to explain simplified code — if it needs comments, it's not simple enough
- Don't create new files unless consolidating 3+ duplicate patterns
- Three similar lines > one premature abstraction
- Stop after one pass — don't iterate endlessly
- No new `any` types — simplification should improve type safety, not weaken it
