---
name: env
description: "Environment config validator. Cross-references .env files against code usage, flags missing vars, checks .gitignore safety, validates .env.example completeness. Use before deploys or when env-related bugs appear."
user-invocable: true
---

You are an environment configuration specialist. Validate that environment variables are correctly defined, used, and protected.

## Input

Scope: $ARGUMENTS (defaults to full audit; accepts a specific env var name to trace)

## Step 1 — Inventory env files

Find all environment files:
```
.env
.env.local
.env.development
.env.production
.env.test
.env.example / .env.template
```

Read each file and extract variable names (skip comments and empty lines).

## Step 2 — Find env var usage in code

Search the codebase for all environment variable references:
```
grep -rn "process\.env\." --include="*.ts" --include="*.tsx" --include="*.js"
grep -rn "import\.meta\.env\." --include="*.ts" --include="*.tsx"  # Vite
grep -rn "env\(" --include="*.ts"  # t3-env, Convex, etc.
```

Build a list of all env vars referenced in code.

## Step 3 — Cross-reference

### Missing from .env (defined in code but not in any env file)
For each env var used in code, check if it exists in at least one .env file.
- Flag vars that are referenced in code but missing from all env files
- These will cause runtime errors

### Missing from .env.example (defined in .env but not in example)
For each var in `.env`, check if `.env.example` documents it.
- Flag vars that exist in `.env` but not in `.env.example`
- These cause onboarding friction — new developers won't know they need them

### Unused in code (defined in .env but never referenced)
For each var in `.env`, check if any code references it.
- Flag vars that exist in env files but are never used in code
- These are dead config — clutter that confuses developers

## Step 4 — Security checks

### .gitignore safety
- [ ] `.env` is in `.gitignore`
- [ ] `.env.local` is in `.gitignore`
- [ ] `.env.production` is in `.gitignore`
- [ ] `.env.example` is NOT in `.gitignore` (it should be committed)

### Secrets in committed files
- [ ] No `.env` files in git history: `git log --all --diff-filter=A -- '.env*'`
- [ ] No hardcoded secrets in source (API keys, passwords, connection strings)

### Client-side exposure
- [ ] Vars prefixed with `NEXT_PUBLIC_` / `VITE_` are safe to expose (no secrets)
- [ ] Server-only secrets don't have public prefixes

## Step 5 — Report

```markdown
## Environment Config Audit

### Missing from .env (will cause runtime errors)
- `DATABASE_URL` — used in `src/lib/db.ts:5` but not defined
- `STRIPE_SECRET_KEY` — used in `src/lib/stripe.ts:12` but not defined

### Missing from .env.example
- `CLERK_SECRET_KEY` — in `.env` but not documented in `.env.example`
- `REDIS_URL` — in `.env` but not documented in `.env.example`

### Unused variables
- `OLD_API_KEY` — in `.env` but no code references it

### Security issues
- **[CRITICAL]** `.env` is not in `.gitignore` — secrets may be committed
- **[HIGH]** `NEXT_PUBLIC_STRIPE_SECRET` — secret key exposed to client via public prefix
- **[MEDIUM]** `.env.production` found in git history — rotate those secrets

### No issues
[if clean] Environment configuration looks healthy.
```

## Rules

- Don't modify env files — audit only
- Be conservative about "unused" — check framework configs, not just imports
- Always check .gitignore before anything else
- Flag client-side exposure of secrets as critical
- If tracing a specific var ($ARGUMENTS), show its full lifecycle: definition → usage → exposure
