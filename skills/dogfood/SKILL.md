---
name: dogfood
description: "DEPRECATED — use the qa skill instead. Browser QA testing has been consolidated into the qa skill which supports three tiers (Quick/Standard/Exhaustive) and includes a fix loop."
user-invocable: false
deprecated: true
---

# Deprecated

This skill has been superseded by the `qa` skill.

Use `/qa` instead, which provides:
- Three tiers: Quick (critical/high only), Standard (+medium), Exhaustive (+cosmetic)
- Fix loop with atomic commits
- Health scoring with before/after comparison
- Diff-aware mode
- Uses `/browse` skill for browser automation (not claude-in-chrome)

All functionality from dogfood is available in qa.
