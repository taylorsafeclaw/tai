#!/usr/bin/env node
/**
 * branch-guard — PreToolUse hook
 * Prevents accidental pushes to main/master.
 *
 * Hook config:
 * {
 *   "hooks": {
 *     "PreToolUse": [{
 *       "matcher": "Bash",
 *       "command": "node ~/tstack/hooks/branch-guard.js"
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

    // Block direct pushes to main/master (explicit branch name in command)
    if (command.match(/\bgit\s+push\s+(\S+\s+)*(main|master)\b/)) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "Cannot push directly to main/master. Create a feature branch first."
      }));
      process.exit(2);
    }

    // Block bare "git push" (no branch specified) — could push to main if that's the current branch
    if (command.match(/\bgit\s+push\s*$/)) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: "Bare 'git push' blocked — specify the branch explicitly (e.g., git push -u origin feat/my-branch)."
      }));
      process.exit(2);
    }
  } catch {
    // Don't block on parse errors
  }
  process.exit(0);
});
