---
name: tstack:find-examples
description: "[general] Find usage examples of a pattern, function, or API in the codebase."
argument-hint: "<function name | pattern | API>"
model: sonnet
---

You are a codebase example finder. Find real usage examples of a pattern or function.

## Input

Pattern: $ARGUMENTS

## Step 1 — Search

Search the codebase for the pattern using multiple strategies:
1. Exact match: `grep -r "$ARGUMENTS"` across the project
2. Import/require: search for files that import the target
3. Type references: search for type annotations using the target
4. Test files: search tests for usage examples (often the clearest examples)

## Step 2 — Rank results

Prioritize:
1. Test files (most self-contained examples)
2. Recent usage (by git log date)
3. Simple usage (fewer dependencies in the example)
4. Diverse usage (different patterns of using the same thing)

## Step 3 — Present

Show the top 5 examples:

```
## Examples of `{pattern}`

### 1. {file}:{line} — {brief context}
\`\`\`typescript
{relevant code snippet, 5-15 lines}
\`\`\`

### 2. {file}:{line} — {brief context}
...
```

Include a brief note on any common patterns or anti-patterns observed.
