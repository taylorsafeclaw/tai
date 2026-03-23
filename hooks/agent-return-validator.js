#!/usr/bin/env node
/**
 * agent-return-validator — SubagentStop hook
 * Logs agent completion + duration for debugging agent team coordination.
 *
 * Hook config:
 * {
 *   "hooks": {
 *     "SubagentStop": [{
 *       "command": "node ~/tstack/hooks/agent-return-validator.js"
 *     }]
 *   }
 * }
 */

const { appendFileSync, mkdirSync } = require('fs');
const { resolve } = require('path');

let input = '';
process.stdin.setEncoding('utf8');
process.stdin.on('data', (chunk) => { input += chunk; });
process.stdin.on('end', () => {
  try {
    const data = JSON.parse(input);
    const agentName = data.agent_name || 'unknown';
    const duration = data.duration_ms || 0;
    const status = data.exit_status || 'unknown';

    // Resolve paths relative to CWD (hook runs in project root)
    const taiDir = resolve(process.cwd(), '.tstack');
    const logPath = resolve(taiDir, '.agent-log');

    mkdirSync(taiDir, { recursive: true });

    const entry = `${new Date().toISOString()} | ${agentName} | ${duration}ms | ${status}\n`;
    appendFileSync(logPath, entry);
  } catch {
    // Informational only — never block
  }
  process.exit(0);
});
