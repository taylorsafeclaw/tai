# Review Cycle Configuration

## Review Source

The Claude Code GitHub Action (`claude-review.yml`) is the **source of truth** for all code review feedback. Other review tools (Greptile, etc.) may also comment but are secondary.

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

## Bot Username Filter

Filter PR comments to those from the Claude Code Action. The bot username is the OAuth app identity — filter by any username ending in `[bot]`.

```
bot_filter: username ends with "[bot]"
```

## Review Checklist Priority (from claude-review.yml)

The action reviews in this priority order — use the same priority when creating Linear tickets:

1. **Security** (blocking) — secrets, encryption, auth, injection
2. **Convex correctness** (blocking) — schema/validator sync, indexes, input validation
3. **State machine safety** (blocking) — workspace lifecycle transitions
4. **Fly.io orchestration** (high) — idempotent volumes, resource caps
5. **Frontend patterns** (medium) — Radix/CVA/cn(), Tailwind, server/client components
6. **General quality** (medium) — types, error handling, performance

## Trigger Behavior

The action triggers on: `opened`, `ready_for_review`, `reopened`
- Does NOT trigger on `synchronize` (re-pushes) — re-review requires manual trigger or closing/reopening the PR
- Path-scoped: only runs when `app/`, `components/`, `convex/`, `lib/`, `env.ts`, or `next.config.ts` change

## Polling Settings

- **Check interval:** 30 seconds
- **Max wait time:** 5 minutes (10 checks)
- **Poll command:** `gh pr checks {PR#}`

## Default Max Iterations

- **Default:** 3 re-review cycles
- **Override:** `--max-iter N` flag
