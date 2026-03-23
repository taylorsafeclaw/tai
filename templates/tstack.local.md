---
mode: normal
---

# tstack Local Config

Set `mode` in frontmatter to adjust model selection:

- **normal** — default model assignments (haiku/sonnet/opus by tier)
- **boost** — shift all models one tier up (harder problems, more cost)
- **economy** — shift all models one tier down (faster, cheaper)

Place this file in your project root or `~/.claude/` to configure globally.
