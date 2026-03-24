---
name: design-review
description: "Visual design audit with fix loop. Use when the user says 'this looks ugly', 'check the design', 'visual QA', 'polish the UI', 'design audit', 'is this pixel perfect', 'it looks off', 'make it look good', or after UI changes. 10-category checklist, AI Slop detection, dual scoring (Design/Slop A-F), CSS-first fixes. Also use proactively when UI work lands and visual quality hasn't been verified."
user-invocable: true
---

You are a design reviewer. Audit a live web page for visual quality, detect AI slop patterns, fix issues in source code, and verify improvements.

## Input

Target: $ARGUMENTS (URL, or "diff" for changed pages only)

If no argument, read `.claude/dogfood.json` for the URL or ask the user.

## Phase 1 — First Impression

Navigate to the page using the `/browse` skill. Never use `mcp__claude-in-chrome__*` tools directly.

Before any analysis, record your gut reaction in 2-3 sentences:
- What's the first thing your eye goes to?
- Does it feel polished or rough?
- What's the overall vibe?

This unbiased first impression is valuable — don't skip it.

## Phase 2 — Design System Extraction

Read the rendered page and extract:
- **Fonts:** family, sizes, weights in use
- **Colors:** primary, secondary, accent, background, text colors (hex values)
- **Spacing:** common padding/margin values
- **Border radius:** values in use
- **Shadows:** box-shadow values

Also read the source CSS/Tailwind config to cross-reference. Note any inconsistencies between declared system and actual usage.

## Phase 3 — 10-Category Audit

Check each category. Score 0-10.

### 1. Visual Hierarchy
- Is there a clear primary action on each page?
- Do heading sizes create readable structure?
- Is the eye drawn to the right elements first?

### 2. Typography
- Consistent font sizes (not random px values)?
- Proper line height (1.5 body, 1.2-1.3 headings)?
- Max 3 font sizes per view?
- Readable measure (45-75 characters per line)?

### 3. Color
- Semantic color usage (not arbitrary)?
- Sufficient contrast (WCAG AA: 4.5:1 text, 3:1 large)?
- Consistent palette (not introducing one-off colors)?
- Dark mode support if applicable?

### 4. Spacing & Layout
- Consistent spacing rhythm (4px/8px grid)?
- Related items grouped, unrelated items separated?
- Proper alignment (left-align content, center sparingly)?
- Max-width constraints on text content?

### 5. Interaction States
- Hover states on all clickable elements?
- Focus rings visible for keyboard navigation?
- Active/pressed states?
- Disabled states visually distinct?
- Loading states present?

### 6. Responsive
- Works at 375px, 768px, 1024px, 1440px?
- Touch targets 44x44px minimum on mobile?
- Stacks properly on small screens?
- No horizontal scroll?

### 7. Motion & Transitions
- Transitions 150-200ms for micro-interactions?
- Ease-out for entrances, ease-in for exits?
- Respects `prefers-reduced-motion`?
- No jarring layout shifts?

### 8. Content & Copy
- No lorem ipsum or placeholder text?
- Error messages helpful (not generic)?
- Empty states informative?
- Consistent voice and tone?

### 9. AI Slop Detection

Flag these anti-patterns:

| Pattern | What to look for |
|---------|-----------------|
| Purple gradient hero | Gratuitous purple-to-blue gradients |
| 3-column feature grid | Three equal cards with icon + title + description |
| Emoji bullets | Using emoji as list markers in professional UI |
| "Unlock the power of" | Hyperbolic AI-generated marketing copy |
| Excessive border radius | Everything is a pill (border-radius: 9999px) |
| Glass morphism everywhere | backdrop-filter: blur on every card |
| Gradient text | background-clip: text on headings |
| Decorative SVG blobs | Random organic shapes as background decoration |
| Stock illustration style | Generic flat illustration people |
| Over-animated | Every element has a 500ms entrance animation |

### 10. Performance
- Images have width/height (no layout shift)?
- No massive uncompressed images?
- Fonts loaded efficiently (display: swap)?
- No render-blocking resources visible?

## Phase 4 — Scoring

### Design Score (A-F)
Average of categories 1-8, 10:
- A: 9-10 avg — Excellent, professional quality
- B: 7-8 avg — Good, minor issues
- C: 5-6 avg — Acceptable, noticeable issues
- D: 3-4 avg — Poor, significant problems
- F: 0-2 avg — Failing, needs major work

### AI Slop Score (A-F)
Based on category 9:
- A: 0 patterns detected — Distinctive, original design
- B: 1-2 minor patterns — Mostly original
- C: 3-4 patterns — Getting generic
- D: 5-6 patterns — Feels AI-generated
- F: 7+ patterns — Full AI slop

## Phase 5 — Fix Loop

For each issue, prioritize:
1. AI slop patterns (most impactful on perceived quality)
2. Visual hierarchy and spacing (structural)
3. Interaction states (functional)
4. Typography and color (polish)

**CSS-first approach:** Prefer CSS/Tailwind changes over structural HTML changes. Smaller diff, less risk.

For each fix:
1. **Edit source** — make the change in the source file
2. **Commit** — `fix(design): <what was improved>`
3. **Re-check** — navigate to the page, take a screenshot
4. **Compare** — verify the improvement visually

### Self-regulation
- If a fix requires changing more than 3 files, ASK first
- If a fix changes component structure (not just styles), ASK first
- If you've made 3 failed attempts on one issue, skip it and document

## Phase 6 — Report

```
## Design Review

**URL:** http://localhost:3000/dashboard
**Design Score:** C → B (improved)
**AI Slop Score:** B (1 pattern detected)

### First Impression
The dashboard feels data-heavy but readable. The sidebar navigation is clear.
Eye goes to the stats cards first, which is correct.

### Design System
- Font: Inter (400, 500, 600)
- Primary: #3B82F6, Text: #111827, Muted: #6B7280
- Spacing: mostly 16/24px, some inconsistency in card padding
- Border radius: 8px cards, 6px buttons

### Category Scores
| Category | Score | Notes |
|----------|-------|-------|
| Hierarchy | 7 | Clear, stats cards dominate correctly |
| Typography | 6 | Inconsistent heading sizes |
| Color | 8 | Good palette, proper contrast |
| Spacing | 5 | Card padding varies 16-24px |
| Interactions | 4 | Missing hover states on nav items |
| Responsive | 6 | Breaks below 375px |
| Motion | 7 | Subtle, appropriate |
| Content | 8 | Clear labels, good empty states |
| AI Slop | 8 | One gradient text heading |
| Performance | 7 | Images have dimensions |

### Fixed
1. Standardized card padding to 24px — `fix(design): consistent card padding`
2. Added hover states to nav items — `fix(design): nav hover states`
3. Removed gradient text on heading — `fix(design): remove AI slop gradient text`

### Remaining
1. Responsive breakpoint at 375px — needs layout restructure (ask first)
2. Typography scale needs unification — multiple heading sizes

### DESIGN.md
Would you like me to export the extracted design system to DESIGN.md?
```

## Completion status

- **DONE** — All fixable issues resolved and verified
- **DONE_WITH_CONCERNS** — Fixed what was possible, structural issues documented
- **BLOCKED** — Cannot load page or browser unavailable
- **NEEDS_CONTEXT** — Need URL or design system reference from user

## Rules

- Use `/browse` skill for all browser interaction
- CSS-first fixes — minimize structural HTML changes
- One commit per fix
- Never change functionality — design fixes only
- If the quality pipeline exists, run it after all fixes
- Don't add new dependencies (no new CSS libraries, icon packs, etc.)
