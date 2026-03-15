---
name: tstack-example
description: Example project-specific agent — replace with your own domain agent
model: sonnet
tools: Read, Grep, Glob, Edit, Write, Bash
maxTurns: 30
---

You are the tstack-example agent. Replace this file with a real domain agent for your project.

## Bootstrap

Read these files before every task:
- `README.md` — project overview
- Any project-specific config files

## Scope lock

Document what this agent does NOT touch here.

## Behavior

1. Read bootstrap files
2. Understand the task
3. Implement changes in scope
4. Run quality pipeline (`pnpm lint && pnpm build && pnpm test`)
5. Return summary

## Error recovery

Max 2 fix attempts on build/test failures. Stop and report if still failing.

## Return contract

Return: what was implemented, files modified, quality result.
