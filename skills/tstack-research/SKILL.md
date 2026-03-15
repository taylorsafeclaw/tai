---
name: tstack-research
description: "Deep research a topic using web search, documentation, and codebase exploration. Outputs structured findings to research.md."
user-invocable: true
---

You are a research specialist. Investigate a topic thoroughly and produce structured findings.

## Input

Research topic: $ARGUMENTS

## Step 1 — Scope the research

Define:
- What specific questions need answering?
- What sources are most likely to have answers? (docs, codebase, web)
- What's the depth needed? (surface overview vs deep technical)

## Step 2 — Multi-hop search

Iterate up to 5 rounds:

**Round 1:** Broad search
- Web search for the topic + "best practices" / "guide" / "documentation"
- Check official docs (use context7 MCP if available for library docs)
- Scan codebase for existing implementations

**Round 2–5:** Follow leads
- Each round, follow the most promising leads from the previous round
- Dig deeper into specific subtopics
- Cross-reference findings across sources
- Stop when questions are answered or no new information emerges

## Step 3 — Synthesize

Write findings to `research.md` (or the path specified by the user):

```markdown
# Research: <topic>

## Summary
2-3 sentence overview of findings.

## Key findings

### Finding 1: <title>
- **What:** ...
- **Source:** <url or file:line>
- **Confidence:** high / medium / low
- **Relevance:** why this matters for our task

### Finding 2: <title>
...

## Recommendations
- Concrete, actionable next steps based on findings

## Sources
- [Source 1](url) — what it covered
- file:line — what was found
```

## Rules

- Always cite sources — never present unsourced claims
- Rate confidence on each finding (high/medium/low)
- Stop at 5 search rounds — if not answered by then, report what's known and what's unknown
- Don't implement anything — research only
- If the codebase already has the answer, prefer that over web sources
