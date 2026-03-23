#!/usr/bin/env node
/**
 * guard-destructive — PreToolUse hook
 * Blocks dangerous destructive commands: git reset --hard, rm -rf on root/home/current dirs.
 *
 * Hook config (~/.claude/settings.json):
 * {
 *   "hooks": {
 *     "PreToolUse": [{
 *       "matcher": "Bash",
 *       "command": "node ~/tstack/hooks/guard-destructive.js"
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

    // Block git reset --hard without specific ref
    if (command.match(/\bgit\s+reset\s+--hard\b/)) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "git reset --hard is destructive. Use git stash or git revert instead. If you really need this, ask the user to confirm."
      }));
      process.exit(2);
    }

    // Block rm -rf on dangerous paths
    else if (command.match(/\brm\s+-rf\s+[\/~]/) || command.match(/\brm\s+-rf\s+\.(\s|$)/)) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "rm -rf on root/home/current directory is too dangerous. Be more specific about what to delete."
      }));
      process.exit(2);
    }
  } catch {
    // Don't block on parse errors
  }
  process.exit(0);
});
