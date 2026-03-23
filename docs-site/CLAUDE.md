# CLAUDE.md — tstack docs site

## Quick reference

```bash
pnpm dev          # start dev server (port 3000)
pnpm build        # production build
pnpm lint         # eslint
```

## Stack

- **Next.js 16** (App Router) + React 19 + TypeScript
- **Tailwind CSS 4** via `@tailwindcss/postcss` (not v3 config — no `tailwind.config.js`)
- **Markdoc** for docs content (`@markdoc/next.js` plugin wraps Next config)
- **shadcn/ui** (base-nova style, lucide icons) — components in `src/components/ui/`
- **motion** (framer-motion successor) for animations
- Path alias: `@/*` → `./src/*`

## Architecture

```
src/
├── app/
│   ├── layout.tsx              ← root layout (fonts, dark mode, wraps Layout)
│   ├── page.md                 ← homepage (markdoc)
│   └── docs/<topic>/page.md    ← each doc page is a markdoc file
├── components/                   ← all kebab-case filenames
│   ├── layout.tsx              ← shell: Header + Hero (homepage only) + sidebar + content
│   ├── docs-layout.tsx         ← markdoc document renderer (header + prose + TOC)
│   ├── hero.tsx                ← homepage hero (terminal animation, CTA)
│   ├── navigation.tsx          ← sidebar nav (reads from lib/navigation.ts)
│   ├── ui/                     ← shadcn + custom UI primitives
│   └── icons/                  ← SVG icon components (kebab-case)
├── lib/
│   ├── navigation.ts           ← sidebar nav structure (sections + links)
│   ├── sections.ts             ← TOC section collector
│   └── utils.ts                ← cn() helper (clsx + twMerge)
├── markdoc/
│   ├── tags.js                 ← custom markdoc tags: callout, figure, quick-links, quick-link
│   ├── nodes.js                ← document/heading/fence node overrides
│   └── search.mjs              ← search index builder
├── styles/
│   ├── tailwind.css            ← main stylesheet (theme, base, CSS vars)
│   └── prism.css               ← code block syntax highlighting
└── fonts/
    └── lexend.woff2            ← local font
```

## Design system — neobrutalist

Everything follows a strict neobrutalist aesthetic. Do not deviate.

- **No rounded corners** — global `border-radius: 0 !important` in base styles
- **No box shadows** — removed globally
- **Accent color**: lime-400 (`#a3e635`) / lime-500 / lime-600
- **Background**: `#0a0a0a` (near-black)
- **Borders**: `border-2 border-neutral-800` (thick, visible)
- **Dark mode only** — `dark` class is always on `<html>`

### Fonts

| Variable | Font | Usage |
|----------|------|-------|
| `--font-sans` | Geist | Body text, default |
| `--font-display` | Syne | Hero headlines, section titles |
| `--font-mono` | JetBrains Mono | Code, nav labels, UI chrome, buttons |
| `--font-lexend` | Lexend (local) | Available but sparingly used |

### Typography patterns

- Nav section headings: `font-mono text-xs font-bold uppercase tracking-widest text-neutral-400`
- Nav links: `font-mono`, lime-400 when active, neutral-500 otherwise
- Hero headline: `font-display text-5xl+ font-bold tracking-tighter uppercase`
- Buttons: `font-mono text-sm font-bold tracking-wider uppercase`
- Body: `font-sans text-lg text-neutral-400`

## Adding a new docs page

1. Create `src/app/docs/<slug>/page.md` with markdoc frontmatter:
   ```md
   ---
   title: My Page
   ---
   Content here using markdoc syntax.
   ```
2. Add entry to `src/lib/navigation.ts` in the appropriate section
3. Available markdoc tags: `{% callout %}`, `{% figure %}`, `{% quick-links %}`, `{% quick-link %}`

## Key conventions

- **Kebab-case filenames** — all components use `kebab-case.tsx` (e.g., `docs-header.tsx`, `mobile-navigation.tsx`), NOT PascalCase
- Use `cn()` from `@/lib/utils` for ALL conditional classes (never raw `clsx`)
- Use `const` for all non-reassigned variables (never `let` for hooks, destructured state, etc.)
- `'use client'` directive on any component using hooks or browser APIs
- `layout.tsx` shows Hero only on homepage (`pathname === '/'`)
- No `forwardRef` — use React 19 ref-as-prop pattern instead
- shadcn components go in `src/components/ui/`
- Icons go in `src/components/icons/`

## Gotchas

- `next.config.mjs` wraps config with both `withMarkdoc` and `withSearch` — don't break the chain
- Tailwind v4 uses `@theme` blocks in CSS, not `tailwind.config.js` — extend theme in `src/styles/tailwind.css`
- `pageExtensions` includes `md` — that's how markdoc pages work as routes
- The `dark` class is hardcoded on `<html>`, not toggled — this is a dark-only site
- Dev server uses `--webpack` flag (not turbopack) due to markdoc compatibility
