#!/usr/bin/env node
/**
 * freeze — PreToolUse hook
 * Blocks Edit and Write operations outside a specified directory scope.
 *
 * Activated by writing a path to `.tstack/.freeze-boundary`.
 * Delete the file to deactivate.
 *
 * Hook config (~/.claude/settings.json):
 * {
 *   "hooks": {
 *     "PreToolUse": [
 *       {
 *         "matcher": "Edit|Write",
 *         "command": "node ~/tstack/hooks/freeze.js"
 *       }
 *     ]
 *   }
 * }
 */

const fs = require('fs');
const path = require('path');

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  try {
    // Find the freeze boundary file
    const freezePath = path.join(process.cwd(), '.tstack', '.freeze-boundary');

    if (!fs.existsSync(freezePath)) {
      // No freeze active — allow everything
      process.exit(0);
    }

    const allowedPath = fs.readFileSync(freezePath, 'utf8').trim();
    if (!allowedPath) {
      process.exit(0);
    }

    const data = JSON.parse(input);
    const toolInput = data.tool_input || {};
    const filePath = toolInput.file_path || '';

    if (!filePath) {
      process.exit(0);
    }

    // Resolve both paths to absolute for comparison
    const resolvedAllowed = path.resolve(allowedPath);
    const resolvedFile = path.resolve(filePath);

    // Check if the file being edited is within the allowed path
    if (!resolvedFile.startsWith(resolvedAllowed + path.sep) && resolvedFile !== resolvedAllowed) {
      process.stdout.write(JSON.stringify({
        decision: "block",
        reason: `Freeze active: edits restricted to ${allowedPath}. This file is outside the boundary. Delete .tstack/.freeze-boundary to unfreeze.`
      }));
      process.exit(2);
    }
  } catch {
    // Don't block on errors
  }
  process.exit(0);
});
