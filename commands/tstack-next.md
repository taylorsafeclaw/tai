---
name: tstack-next
description: Advance a mission to the next feature — verifies current, opens PR, updates state.json, shows next goal.
argument-hint: ""
model: sonnet
---

You are the tstack mission advancer. Close out the current feature and move to the next.

## Step 1 — Read state

Read `.tstack/state.json` and `.tstack/ROADMAP.md`.

Identify current feature number and its goal.

## Step 2 — Verify (inlined)

Run verification directly — do not invoke `/tstack-verify` as a separate command.

### 2a. Check success criteria

Read the current feature's success criteria from `.tstack/ROADMAP.md`.

For each criterion:
- **Code existence:** Use Glob/Grep to verify the file/function/component exists
- **Behavior wiring:** Read the code to verify mutations are called, UI renders the right state
- Report pass/fail per criterion

### 2b. Quality pipeline

Run in order. Stop on first failure.
1. `pnpm lint` — if project has lint script
2. `pnpm build` — if project has build script
3. `pnpm test` — if project has test script

Detection: read `package.json` scripts to verify each exists before running.
If a script doesn't exist, skip it.

### 2c. Verification result

If any criterion fails OR quality pipeline fails:
- Show what's missing/failing
- Say: "Feature <N> is not complete. Fix the issues above, then run `/tstack-next` again."
- Stop here.

## Step 3 — Open PR (if verify passes)

```bash
git push -u origin feat/<slug>
```

Create PR with rich description:
```bash
gh pr create --title "feat(<scope>): <feature name>" --body "$(cat <<'EOF'
## Summary
<what this feature implements>

## Success criteria
<from ROADMAP.md — checked off>

## Test plan
- [ ] Quality pipeline passes
- [ ] <specific verification items>
EOF
)"
```

**Error recovery for PR creation:**
If PR creation fails (no remote, auth issues, branch not pushed):
1. Show the exact error
2. Suggest manual steps: "Push with `git push -u origin <branch>`, then create PR manually or re-run `/tstack-next`"
3. Do NOT update state.json — feature is not advanced until PR exists

Return the PR URL.

## Step 4 — Update state

Update `.tstack/state.json`:
- Mark current feature as `complete` with today's date and PR number
- Set `currentFeature` to `currentFeature + 1`

```json
{
  "features": {
    "<N>": {
      "status": "complete",
      "completedAt": "<ISO date>",
      "pr": "#<number>"
    }
  },
  "currentFeature": <N+1>
}
```

## Step 5 — Show next feature

If there is a next feature:
```
Feature <N> complete → PR #<number>

Next: Feature <N+1> — <name>
Goal: <goal>

Run /tstack-scope to research it, then /tstack-plan to plan it.
Or run /tstack-execute if you already have a plan.
```

If no next feature (mission complete):
```
Feature <N> complete → PR #<number>

Mission complete! All features shipped.
Run /tstack-status to see the full summary.
```

## Rules

- Never advance if verify fails
- Always open a PR before advancing
- State update is the source of truth — keep it accurate
- If PR creation fails, do NOT advance the mission state
