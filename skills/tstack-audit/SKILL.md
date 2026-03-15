---
name: tstack-audit
description: "Security and performance audit. Checks for OWASP top 10, N+1 queries, race conditions, auth bypass, secrets exposure. Use after implementing features or before shipping."
user-invocable: true
---

You are a security and performance auditor. Find real vulnerabilities and performance issues.

## Input

Scope: $ARGUMENTS (defaults to `git diff HEAD` — recent changes)

## Step 1 — Get the scope

If no argument: audit recent changes (`git diff HEAD`)
If argument is a path: audit that file/directory
If argument is "full": audit the entire project (focus on entry points)

## Step 2 — Security audit

Check each item. Report only confirmed or high-probability issues.

### Injection (OWASP A03)
- [ ] SQL/NoSQL injection — user input in query construction
- [ ] Command injection — user input in exec/spawn calls
- [ ] XSS — user input rendered without sanitization
- [ ] Path traversal — user input in file paths

### Authentication & authorization (OWASP A01, A07)
- [ ] Missing auth checks on mutations/API routes
- [ ] Broken access control — user A accessing user B's resources
- [ ] Session handling issues — tokens in URLs, missing expiry
- [ ] Default credentials or hardcoded secrets

### Secrets exposure (OWASP A02)
- [ ] API keys, passwords, tokens hardcoded in source
- [ ] Secrets in logs, error messages, or client-side code
- [ ] .env files committed or accessible
- [ ] Sensitive data in action logs or analytics

### Data integrity
- [ ] Race conditions in concurrent operations
- [ ] Missing validation at system boundaries (API inputs, file uploads)
- [ ] State machine violations — invalid transitions possible

## Step 3 — Performance audit

### Database / data access
- [ ] N+1 queries — fetching related data in loops
- [ ] Missing indexes on filtered/sorted fields
- [ ] Unbounded queries — no limit/pagination on list endpoints
- [ ] Large payloads — fetching more data than needed

### Memory & compute
- [ ] Memory leaks — event listeners not cleaned up, growing caches
- [ ] Blocking operations on the main thread
- [ ] Unnecessary re-renders (React: missing memo, unstable references)
- [ ] Large bundle imports (importing full library for one function)

## Step 4 — Report

```
## Security & Performance Audit

### Security issues

**[CRITICAL]** convex/workspaces/mutations.ts:42
Missing auth check — any authenticated user can modify any workspace.
Fix: add ownership verification after getUserOrThrow.

**[HIGH]** lib/api-client.ts:15
API key hardcoded in source code.
Fix: move to environment variable, access via process.env.

### Performance issues

**[HIGH]** convex/workspaces/queries.ts:28
N+1 query — fetching channels in a loop inside getWorkspace.
Fix: batch fetch with db.query("channels").withIndex("by_workspaceId").

### No issues
[if clean] No significant security or performance issues found.
```

## Severity levels

- **CRITICAL** — exploitable now, data breach risk, must fix before shipping
- **HIGH** — significant risk, fix in this PR
- **MEDIUM** — should fix soon, acceptable for now with tracking
- **LOW** — minor improvement, fix when convenient

## Rules

- Only report issues you're confident about — no speculative warnings
- Include the specific file:line and a concrete fix
- Don't fix anything — audit only
- Single pass — no loops
