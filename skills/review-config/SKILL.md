---
name: review-config
description: "Claude Code GitHub Action review configuration — bot username filter, polling settings, checklist priority order, trigger behavior. Internal skill loaded by review-ingester and review-cycle command."
user-invocable: false
disable-model-invocation: true
---

# Review Cycle Configuration

## Review Source

The Claude Code GitHub Action (`claude-review.yml`) is the **source of truth** for all code review feedback.

## Comment Format (from claude-review.yml)

The action posts two types of comments:

### 1. Top-level summary comment (sticky)
- Uses `use_sticky_comment: true` — updates the same comment on each run
- Format: one-line summary → file table with review notes → blocking issues → non-blocking suggestions
- Posted via `gh pr comment`

### 2. Inline comments
- Posted with `confirmed: true` flag on specific code lines
- `classify_inline_comments: true` — issues are pre-classified by severity
- `include_fix_links: true` — includes links to fix the issue

### 3. Structured metadata markers
Review comments include parseable markers:
```
<!-- review-meta:start -->
<!-- issue|{type}|{confidence}|{file}|{line}|{blocking} -->
<!-- review-meta:end -->
```

Parse these markers to extract structured issue data without relying on comment text parsing.

## Bot Username Filter

Filter PR comments to those from the Claude Code Action bot:
```
bot_filter: username ends with "[bot]"
```

## Review Checklist Priority (from claude-review.yml)

The action reviews in this priority order — use the same priority when creating tickets and fixing:

1. **Security** (blocking) — secrets, encryption, auth, injection
2. **Convex correctness** (blocking) — schema/validator sync, indexes, input validation
3. **State machine safety** (blocking) — workspace lifecycle transitions
4. **Fly.io orchestration** (high) — idempotent volumes, resource caps
5. **Frontend patterns** (medium) — Radix/CVA/cn(), Tailwind, server/client components
6. **General quality** (medium) — types, error handling, performance

## Trigger Behavior

The action triggers on: `opened`, `synchronize`, `ready_for_review`, `reopened`
- **synchronize** — re-pushes trigger automatic re-review (required for fix loop)
- Path-scoped: only runs when `app/`, `components/`, `convex/`, `lib/`, `env.ts`, or `next.config.ts` change

## Polling Settings

- **Check interval:** 30 seconds
- **Max wait time:** 5 minutes (10 checks)
- **Poll command:** `gh pr checks {PR#}`

## Default Max Iterations

- **Default:** 3 re-review cycles
- **Override:** `--max-iter N` flag

## Allowed Tools (claude-review.yml)

```
Bash(gh pr comment:*), Bash(gh pr diff:*), Bash(gh pr view:*),
Bash(pnpm lint), Bash(pnpm build), Bash(pnpm test:once)
```
