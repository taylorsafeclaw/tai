---
title: Hooks
---

Hooks are Node.js scripts that run in response to Claude Code events. They enforce guardrails, log information, and block dangerous actions. {% .lead %}

---

## Configuration

Hooks are defined in `hooks/hooks.json` and loaded by the plugin system. No manual settings.json configuration needed.

```json
{
  "hooks": [
    {
      "type": "PreToolUse",
      "matcher": "Bash",
      "hooks": [
        { "command": "node ./hooks/guard-destructive.js" },
        { "command": "node ./hooks/guard-pnpm.js" },
        { "command": "node ./hooks/branch-guard.js" },
        { "command": "node ./hooks/quality-gate.js" }
      ]
    },
    {
      "type": "PreToolUse",
      "matcher": "Edit|Write",
      "hooks": [
        { "command": "node ./hooks/freeze.js" }
      ]
    },
    {
      "type": "SubagentStop",
      "hooks": [
        { "command": "node ./hooks/agent-return-validator.js" }
      ]
    }
  ]
}
```

---

## Available hooks

### `guard-destructive.js`

**Event:** PreToolUse (Bash)

Blocks dangerous destructive commands:

- `git reset --hard` → use git stash or git revert
- `rm -rf /` or `rm -rf ~` or `rm -rf .` → too dangerous

---

### `guard-pnpm.js`

**Event:** PreToolUse (Bash)

Blocks npm/yarn commands, enforcing pnpm as the package manager. Opinionated — remove if your project doesn't use pnpm exclusively.

---

### `branch-guard.js`

**Event:** PreToolUse (Bash)

Prevents accidental pushes directly to main/master. Forces feature branch workflow.

---

### `quality-gate.js`

**Event:** PreToolUse (Bash) — triggers on `git commit`

Blocks commits if the quality pipeline hasn't passed in the current session.

1. When `pnpm lint && pnpm build && pnpm test` all pass, touch `.tstack/.quality-passed`
2. On any file edit, the flag is cleared
3. On `git commit`, the hook checks for the flag
4. If missing: blocks with "Run quality pipeline first"

---

### `freeze.js`

**Event:** PreToolUse (Edit, Write)

Blocks edits outside a specified directory scope. Useful during debugging to prevent touching unrelated code.

1. Write a path to `.tstack/.freeze-boundary` (e.g., `echo "/path/to/src/auth" > .tstack/.freeze-boundary`)
2. The hook blocks any Edit/Write to files outside that path
3. Delete `.tstack/.freeze-boundary` to deactivate

---

### `agent-return-validator.js`

**Event:** SubagentStop

Logs agent completion, duration, and exit status to `.tstack/.agent-log`. Informational only — never blocks.

---

## Creating a new hook

Hooks are Node.js scripts that:

1. Read JSON from stdin (tool input or event data)
2. Decide whether to allow or block
3. Exit with code 0 (allow) or code 2 (block with message)

```javascript
#!/usr/bin/env node
let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  const data = JSON.parse(input);

  // To block:
  process.stdout.write(JSON.stringify({
    decision: "block",
    reason: "Why it's blocked"
  }));
  process.exit(2);

  // To allow:
  process.exit(0);
});
```

Place new hooks in `hooks/` and add them to `hooks/hooks.json`.
