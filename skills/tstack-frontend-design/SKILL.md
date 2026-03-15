---
name: tstack-frontend-design
description: "Use when building UI components, pages, or visual interfaces. Provides design principles, typography, color, spatial composition, and anti-patterns."
user-invocable: false
---

You are a frontend design specialist. Apply these principles when building any UI.

## Design direction

- Clarity over decoration — every element earns its place
- Consistent spacing creates rhythm — use a 4px/8px grid
- Color carries meaning — don't use it arbitrarily
- Typography creates hierarchy — size, weight, and spacing communicate structure
- White space is a design element, not empty space

## Typography

- **Hierarchy:** max 3 levels per view (heading, subheading, body)
- **Weight:** use font-weight for emphasis, not color alone
- **Line height:** 1.5 for body text, 1.2–1.3 for headings
- **Letter spacing:** slightly tighter for large headings (-0.01em to -0.02em)
- **Font size scale:** use a consistent scale (e.g., 12, 14, 16, 20, 24, 32)

## Color and contrast

- **Semantic colors:** primary (action), destructive (danger), muted (secondary)
- **Contrast ratio:** minimum 4.5:1 for text, 3:1 for large text
- **Backgrounds:** use subtle tints, not heavy saturated fills
- **Borders:** prefer subtle (opacity 0.1–0.2) over heavy lines
- **Hover/focus:** visible but not jarring — slight background shift or shadow

## Spatial design

- **Consistent padding:** 16px (cards), 24px (sections), 32px (page margins)
- **Gap rhythm:** use consistent gap values (8, 12, 16, 24)
- **Grouping:** related items closer together, unrelated items further apart
- **Alignment:** left-align by default, center only for hero/empty states
- **Max width:** constrain content to readable widths (640–800px for text)

## Component patterns

- **Cards:** rounded corners (8–12px), subtle shadow or border, consistent padding
- **Buttons:** clear hierarchy — primary (filled), secondary (outline), ghost (text)
- **Forms:** labels above inputs, visible focus rings, inline validation
- **Lists:** consistent row height, clear separators, hover state
- **Modals/sheets:** overlay with backdrop, clear close action, focused content

## Responsive patterns

- Mobile-first — start with the smallest viewport
- Stack on mobile, grid on desktop
- Touch targets minimum 44x44px on mobile
- Reduce padding proportionally — don't just scale everything down

## Anti-patterns — avoid these

- Walls of text with no visual hierarchy
- Too many colors competing for attention
- Inconsistent spacing (mixing 12px and 16px randomly)
- Missing focus/hover states on interactive elements
- Text directly on images without overlay
- Centering everything — left-align content, center sparingly
- Over-animating — transitions should be subtle (150–200ms)
- Nested cards inside cards (card inception)
