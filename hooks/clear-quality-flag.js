#!/usr/bin/env node
/**
 * clear-quality-flag.js — Clears .quality-passed flag when source files are edited.
 *
 * When a file is edited or written, the quality pipeline results are stale.
 * This hook removes the flag so the quality gate will require re-running
 * lint/build/test before the next commit.
 *
 * Hook type: PreToolUse (Edit|Write)
 * Runs alongside freeze.js on the same matcher.
 */

const fs = require('fs');
const path = require('path');

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  try {
    const data = JSON.parse(input);
    const toolInput = data.tool_input || {};
    const filePath = toolInput.file_path || '';

    // Only clear the flag for source file edits, not state/config files
    // Skip .tstack/ files (state system writes shouldn't invalidate the quality gate)
    // Skip node_modules, .git, lock files
    const skipPatterns = [
      '.tstack/',
      'node_modules/',
      '.git/',
      'pnpm-lock.yaml',
      'package-lock.json',
      '.quality-passed'
    ];

    const shouldSkip = skipPatterns.some(pattern => filePath.includes(pattern));

    if (!shouldSkip && filePath) {
      const flagPath = path.join(process.cwd(), '.tstack', '.quality-passed');
      try {
        if (fs.existsSync(flagPath)) {
          fs.unlinkSync(flagPath);
        }
      } catch (e) {
        // Silently ignore — flag might not exist or .tstack/ might not exist yet
      }
    }
  } catch {
    // Don't block on parse errors
  }
  // Always allow the edit to proceed — this hook only clears a flag
  process.stdout.write(JSON.stringify({ decision: "approve" }));
  process.exit(0);
});
