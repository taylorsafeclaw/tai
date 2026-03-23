#!/usr/bin/env node
/**
 * quality-gate — PreToolUse hook
 * Blocks git commits if quality pipeline hasn't passed in this session.
 *
 * Checks for .tstack/.quality-passed flag file.
 * Set by running: pnpm lint && pnpm build && pnpm test
 *
 * Hook config (add to .claude/settings.json):
 * {
 *   "hooks": {
 *     "PreToolUse": [{
 *       "matcher": "Bash",
 *       "command": "node ~/tstack/hooks/quality-gate.js"
 *     }]
 *   }
 * }
 */

const { readFileSync } = require('fs');
const { resolve } = require('path');

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  try {
    const data = JSON.parse(input);
    const toolInput = data.tool_input || {};
    const command = toolInput.command || '';

    // Only check git commit commands
    if (!command.match(/\bgit\s+commit\b/)) {
      process.exit(0);
    }

    // Resolve flag path relative to CWD (hook runs in project root)
    const flagPath = resolve(process.cwd(), '.tstack/.quality-passed');

    // Check if quality pipeline has passed
    try {
      readFileSync(flagPath, 'utf8');
      process.exit(0); // Flag exists — pipeline passed
    } catch {
      // Flag doesn't exist — block the commit
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "Quality pipeline hasn't passed. Run: pnpm lint && pnpm build && pnpm test"
      }));
      process.exit(2);
    }
  } catch {
    // If we can't parse input, don't block
    process.exit(0);
  }
});
