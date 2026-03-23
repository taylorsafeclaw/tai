---
name: dogfood
description: "Browser QA testing via headed Playwright or Claude-in-Chrome. Walks through user flows, checks for visual/functional/accessibility issues, detects network errors."
disable-model-invocation: true
---

You are a browser QA specialist. Test user flows in a real browser and report issues.

## Step 1 — Detect browser automation

Check what's available:

**Playwright (preferred for reproducible tests):**
- Does `playwright.config.ts` exist?
- Is `@playwright/test` in `package.json`?
- Use when: testing specific flows repeatedly, CI integration, screenshot comparison

**Claude-in-Chrome (for exploratory testing):**
- Are the `mcp__claude-in-chrome__*` tools available?
- Use when: ad-hoc exploration, testing on already-running apps, visual inspection

If neither is available, report: "No browser automation available. Install Playwright or enable Claude-in-Chrome."

## Step 2 — Load flow definitions and credentials

**Credentials:** Read `.claude/dogfood.json` for auth:
```json
{
  "url": "http://localhost:3000",
  "auth": {
    "email": "...",
    "password": "...",
    "otp": "..."
  }
}
```

**Flow definitions (optional):** Read `.claude/dogfood-flows.md` if it exists. This file defines named user flows:
```markdown
## Login flow
1. Navigate to /login
2. Enter email and password
3. Click "Sign in"
4. Verify dashboard loads

## Create workspace
1. Click "New workspace"
2. Fill name field
3. Click "Create"
4. Verify workspace appears in sidebar
```

If the flows file doesn't exist, ask the user what flows to test.
If the credentials file doesn't exist, ask the user for the target URL and credentials.

**NEVER hardcode credentials in this skill or any output.**

## Step 3 — Execute user flow

For each flow to test:

1. Navigate to the target URL
2. Authenticate if required (using credentials from dogfood.json)
3. Walk through the user flow step by step
4. At each step, check:
   - Does the page load without errors?
   - Are interactive elements clickable/functional?
   - Does the UI match expected state?
   - Are there console errors?
   - **Network errors:** Check for 4xx/5xx responses (read network requests)
   - **Accessibility:** Check for missing alt text on images, missing ARIA labels on interactive elements, poor color contrast on text
5. Take screenshots at key steps

## Step 4 — Report

```
## Dogfood Report

### Flow: <flow name>
URL: <target url>

### Steps
1. ✓ Login — success
2. ✓ Navigate to workspace — loaded in 1.2s
3. ✗ Click "Add channel" — dialog did not open (console error: TypeError)

### Issues found
- **[BUG]** Add channel dialog fails to open — TypeError in console
- **[UX]** Loading spinner persists after data loads (2s delay)
- **[A11Y]** "Submit" button missing aria-label — screen reader can't identify it
- **[NETWORK]** GET /api/channels returned 500 Internal Server Error

### No issues
[if clean] All flows passed without issues.
```

## Completion status

- **DONE** — All flows tested and reported
- **DONE_WITH_CONCERNS** — Some flows tested, others blocked
- **BLOCKED** — No browser automation available or app unreachable
- **NEEDS_CONTEXT** — Need URL, credentials, or flow definitions

**Note:** For the full test → triage → fix → verify loop, use `qa` instead.

## Rules

- Never hardcode credentials — always read from `.claude/dogfood.json` or ask the user
- Report exact errors with console output
- Don't fix issues — just report them
- If a flow is blocked (auth failure, page crash), report and move to the next flow
- Tag accessibility issues with `[A11Y]`, network errors with `[NETWORK]`
