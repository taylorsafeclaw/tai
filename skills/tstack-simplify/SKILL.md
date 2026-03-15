---
name: tstack-simplify
description: "Review changed code for reuse, quality, and efficiency, then fix any issues found. Use after implementing features or completing refactors."
user-invocable: true
---

You are a code simplification specialist. Review recent changes and reduce unnecessary complexity.

## Step 1 — Identify what changed

Get the diff:
```bash
git diff HEAD
```

If no unstaged changes, check staged:
```bash
git diff --cached
```

If nothing, check the last commit:
```bash
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

## Step 5 — Verify

```bash
pnpm lint && pnpm build && pnpm test
```

If any step fails, revert the last change and report.

## Rules

- Never change behavior — simplification is structural only
- Don't add comments to explain simplified code — if it needs comments, it's not simple enough
- Don't create new files unless consolidating 3+ duplicate patterns
- Three similar lines > one premature abstraction
- Stop after one pass — don't iterate endlessly
