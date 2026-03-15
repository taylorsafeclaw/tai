#!/usr/bin/env node
/**
 * Validates that all tstack-*.md files have required frontmatter fields.
 */

const fs = require('fs');
const path = require('path');

const ROOT = path.join(__dirname, '..', '..');

const COMMAND_REQUIRED = ['name', 'description'];
const AGENT_REQUIRED = ['name', 'description', 'model', 'tools'];
const SKILL_REQUIRED = ['name', 'description', 'user-invocable'];

function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return null;
  const fields = {};
  for (const line of match[1].split('\n')) {
    const [key, ...rest] = line.split(':');
    if (key && rest.length) fields[key.trim()] = rest.join(':').trim();
  }
  return fields;
}

function checkDir(dir, required, kind) {
  const errors = [];
  if (!fs.existsSync(dir)) return errors;
  for (const file of fs.readdirSync(dir)) {
    if (!file.startsWith('tstack-') || !file.endsWith('.md')) continue;
    const content = fs.readFileSync(path.join(dir, file), 'utf8');
    const fm = parseFrontmatter(content);
    if (!fm) {
      errors.push(`${kind}/${file}: missing frontmatter`);
      continue;
    }
    for (const field of required) {
      if (!fm[field]) errors.push(`${kind}/${file}: missing field '${field}'`);
    }
  }
  return errors;
}

function checkSkills(dir) {
  const errors = [];
  if (!fs.existsSync(dir)) return errors;
  for (const skillDir of fs.readdirSync(dir)) {
    if (!skillDir.startsWith('tstack-')) continue;
    const skillFile = path.join(dir, skillDir, 'SKILL.md');
    if (!fs.existsSync(skillFile)) {
      errors.push(`skills/${skillDir}: missing SKILL.md`);
      continue;
    }
    const content = fs.readFileSync(skillFile, 'utf8');
    const fm = parseFrontmatter(content);
    if (!fm) {
      errors.push(`skills/${skillDir}/SKILL.md: missing frontmatter`);
      continue;
    }
    for (const field of SKILL_REQUIRED) {
      if (!fm[field]) errors.push(`skills/${skillDir}/SKILL.md: missing field '${field}'`);
    }
  }
  return errors;
}

const errors = [
  ...checkDir(path.join(ROOT, 'commands'), COMMAND_REQUIRED, 'commands'),
  ...checkDir(path.join(ROOT, 'agents'), AGENT_REQUIRED, 'agents'),
  ...checkSkills(path.join(ROOT, 'skills')),
];

if (errors.length > 0) {
  console.error('Frontmatter validation failed:\n');
  for (const e of errors) console.error('  ✗', e);
  process.exit(1);
} else {
  console.log('All frontmatter valid.');
}
