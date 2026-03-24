#!/usr/bin/env node
/**
 * Validates that all commands, agents, and skills have required frontmatter fields.
 * Scans commands/ and agents/ recursively; skills/<name>/SKILL.md.
 */

const fs = require("fs");
const path = require("path");

const ROOT = path.join(__dirname, "..", "..");

const COMMAND_REQUIRED = ["name", "description"];
const AGENT_REQUIRED = ["name", "description", "model", "tools"];
const SKILL_REQUIRED = ["name", "description", "user-invocable"];

function parseFrontmatter(content) {
  const match = content.match(/^---\n([\s\S]*?)\n---/);
  if (!match) return null;
  const fields = {};
  for (const line of match[1].split("\n")) {
    const [key, ...rest] = line.split(":");
    if (key && rest.length) fields[key.trim()] = rest.join(":").trim();
  }
  return fields;
}

function findMdFiles(dir, base = dir) {
  const results = [];
  if (!fs.existsSync(dir)) return results;
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    const rel = path.relative(base, full);
    if (entry.isDirectory()) {
      results.push(...findMdFiles(full, base));
    } else if (entry.name.endsWith(".md") && entry.name !== "README.md") {
      results.push(rel);
    }
  }
  return results;
}

function checkCommands() {
  const errors = [];
  const dir = path.join(ROOT, "commands");
  const files = findMdFiles(dir);
  for (const rel of files) {
    const content = fs.readFileSync(path.join(dir, rel), "utf8");
    const fm = parseFrontmatter(content);
    if (!fm) {
      errors.push(`commands/${rel}: missing frontmatter`);
      continue;
    }
    for (const field of COMMAND_REQUIRED) {
      if (!fm[field]) errors.push(`commands/${rel}: missing field '${field}'`);
    }
  }
  return errors;
}

function checkAgents() {
  const errors = [];
  const dir = path.join(ROOT, "agents");
  const files = findMdFiles(dir);
  for (const rel of files) {
    const content = fs.readFileSync(path.join(dir, rel), "utf8");
    const fm = parseFrontmatter(content);
    if (!fm) {
      errors.push(`agents/${rel}: missing frontmatter`);
      continue;
    }
    for (const field of AGENT_REQUIRED) {
      if (!fm[field]) errors.push(`agents/${rel}: missing field '${field}'`);
    }
  }
  return errors;
}

function checkSkills() {
  const errors = [];
  const dir = path.join(ROOT, "skills");
  if (!fs.existsSync(dir)) return errors;
  for (const name of fs.readdirSync(dir)) {
    const skillDir = path.join(dir, name);
    if (!fs.statSync(skillDir).isDirectory()) continue;
    const skillFile = path.join(skillDir, "SKILL.md");
    if (!fs.existsSync(skillFile)) {
      errors.push(`skills/${name}: missing SKILL.md`);
      continue;
    }
    const content = fs.readFileSync(skillFile, "utf8");
    const fm = parseFrontmatter(content);
    if (!fm) {
      errors.push(`skills/${name}/SKILL.md: missing frontmatter`);
      continue;
    }
    for (const field of SKILL_REQUIRED) {
      if (!fm[field])
        errors.push(`skills/${name}/SKILL.md: missing field '${field}'`);
    }
  }
  return errors;
}

const errors = [...checkCommands(), ...checkAgents(), ...checkSkills()];

if (errors.length > 0) {
  console.error("Frontmatter validation failed:\n");
  for (const e of errors) console.error("  ✗", e);
  process.exit(1);
} else {
  console.log("All frontmatter valid.");
}
