# Linear Field Mappings for Review Issues

## Team Context

- **Team:** Safeclaw
- **Team Key:** SAF
- **Team ID:** `f800d305-18e0-4218-a795-918ccee14b0a`

## Classification → Linear Fields

| Classification | Label | Label ID | Priority | Priority Value | Initial Status |
|---------------|-------|----------|----------|---------------|----------------|
| bug | Bug | `2b86b02e-7d71-4eb7-8bf4-b1f7b4fd5b6d` | High | 2 | Todo |
| style | Improvement | `35358a34-9a5e-4302-9ab4-f85fcca54110` | Low | 4 | Backlog |
| suggestion | Improvement | `35358a34-9a5e-4302-9ab4-f85fcca54110` | Normal | 3 | Backlog |
| question | Improvement | `35358a34-9a5e-4302-9ab4-f85fcca54110` | Low | 4 | Backlog |

## Classification Rules

Classify each review comment by scanning for keyword signals:

| Keywords | Classification |
|----------|---------------|
| "bug", "incorrect", "wrong", "will fail", "crash", "null", "undefined", "race condition", "error", "broken", "missing check", "vulnerability" | **bug** |
| "convention", "naming", "format", "style", "consistent", "indentation", "spacing", "lint" | **style** |
| "consider", "might want", "could", "alternative", "optional", "would be better", "suggest", "improvement" | **suggestion** |
| Ends with "?", "why", "is this intentional", "can you explain", "what is the reason" | **question** |

If a comment matches multiple classifications, use the highest priority one (bug > suggestion > style > question).

## Ticket Format

**Title:** `[PR-{pr_number}] {brief description of the issue}`

**Description:**
```markdown
**Source:** PR #{pr_number} review comment
**File:** `{file_path}:{line_number}`
**PR Link:** {pr_url}

---

{full review comment text}
```

## Priority Mapping Reference

| User-facing | Linear API value |
|-------------|-----------------|
| Urgent | 1 |
| High | 2 |
| Normal | 3 |
| Low | 4 |
| None | 0 |

## Status Reference

| Status | Type |
|--------|------|
| Backlog | backlog |
| Todo | unstarted |
| In Progress | started |
| In Review | started |
| Done | completed |
| Canceled | canceled |
| Duplicate | canceled |
