#!/usr/bin/env node
/**
 * guard-pnpm — PreToolUse hook (opinionated, opt-in)
 * Blocks npm and yarn commands, enforcing pnpm as the package manager.
 *
 * This hook is opinionated — only add it if your project uses pnpm exclusively.
 *
 * Hook config (~/.claude/settings.json):
 * {
 *   "hooks": {
 *     "PreToolUse": [{
 *       "matcher": "Bash",
 *       "command": "node ~/tstack/hooks/guard-pnpm.js"
 *     }]
 *   }
 * }
 */

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  try {
    const data = JSON.parse(input);
    const toolInput = data.tool_input || {};
    const command = toolInput.command || '';

    // Block npm and yarn (enforce pnpm)
    if (command.match(/\b(npm|yarn)\s+(install|add|remove|i)\b/)) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "Use pnpm instead of npm/yarn. Run: pnpm install, pnpm add, etc."
      }));
      process.exit(2);
    }
  } catch {
    // Don't block on parse errors
  }
  process.exit(0);
});
