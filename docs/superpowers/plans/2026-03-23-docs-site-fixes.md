# Docs-Site Landing Page Fixes Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix all landing page issues — broken Get Started link, missing Extensions page, dead code cleanup, font config fix — and add a live GitHub stars counter to the hero section.

**Architecture:** Five independent fixes plus one new component. The stars counter is a client component that fetches from a Next.js API route handler (which caches the GitHub API response server-side to avoid rate limits). All changes are in `docs-site/src/`.

**Tech Stack:** Next.js 16, React 19, Tailwind CSS 4, TypeScript, lucide-react (Star icon), GitHub REST API

---

## File Structure

| Action | File | Responsibility |
|--------|------|----------------|
| Modify | `src/components/hero.tsx` | Fix Get Started href, add GitHubStars component |
| Create | `src/app/api/github-stars/route.ts` | API route handler — caches GitHub star count server-side |
| Create | `src/app/docs/extensions/page.md` | Extensions documentation page |
| Modify | `src/styles/tailwind.css` | Fix font theme vars in `@theme` + `@theme inline` blocks |
| Delete | `src/components/hero-background.tsx` | Remove unused dead code |

---

### Task 1: Fix Get Started Button Link

**Files:**
- Modify: `docs-site/src/components/hero.tsx:405`

- [ ] **Step 1: Fix the href**

In `hero.tsx`, line 405, change:
```tsx
<Button href="/">
```
to:
```tsx
<Button href="/docs/quickstart">
```

- [ ] **Step 2: Verify the link works**

Run: `cd docs-site && pnpm dev`
Navigate to `http://localhost:3001` and click "Get Started" — should navigate to `/docs/quickstart`.

- [ ] **Step 3: Commit**

```bash
git add docs-site/src/components/hero.tsx
git commit -m "fix(docs): point Get Started button to /docs/quickstart"
```

---

### Task 2: Add GitHub Stars Counter to Hero

**Files:**
- Create: `docs-site/src/app/api/github-stars/route.ts`
- Modify: `docs-site/src/components/hero.tsx`

The stars counter has two parts:
1. **API route handler** — fetches from GitHub API server-side, caches for 1 hour. This avoids the 60 req/hr unauthenticated rate limit hitting visitors directly.
2. **Client component** — fetches from our API route, shows star icon + count with a static fallback to prevent layout shift.

- [ ] **Step 1: Create the API route handler**

Create `docs-site/src/app/api/github-stars/route.ts`:

```ts
const CACHE_DURATION = 3600 // 1 hour in seconds

export async function GET() {
  const res = await fetch(
    'https://api.github.com/repos/taylorsafeclaw/tstack',
    { next: { revalidate: CACHE_DURATION } },
  )

  if (!res.ok) {
    return Response.json({ stars: null }, { status: 502 })
  }

  const data = await res.json()
  return Response.json({ stars: data.stargazers_count ?? null })
}
```

- [ ] **Step 2: Add the GitHubStars component to hero.tsx**

Add the `Star` import at the top of `hero.tsx`:

```tsx
import { Star } from 'lucide-react'
```

Add this component inside `hero.tsx`, above the `Hero` export:

```tsx
function GitHubStars() {
  const [stars, setStars] = useState<number | null>(null)

  useEffect(() => {
    fetch('/api/github-stars')
      .then((res) => res.json())
      .then((data) => {
        if (typeof data.stars === 'number') {
          setStars(data.stars)
        }
      })
      .catch(() => {})
  }, [])

  return (
    <div className="mt-6 flex items-center gap-2 md:justify-center lg:justify-start">
      <Star className="h-4 w-4 fill-lime-400 text-lime-400" />
      <span className="font-mono text-sm tracking-wider text-neutral-400">
        {stars !== null ? `${stars.toLocaleString()} stars on GitHub` : 'Star on GitHub'}
      </span>
    </div>
  )
}
```

Note: The component always renders (no `return null`) — it shows "Star on GitHub" as fallback text while loading or on error, preventing layout shift.

- [ ] **Step 3: Place the component in the Hero**

In the `Hero` component, add `<GitHubStars />` right after the closing `</BlurFade>` of the buttons block (after line 420), wrapped in its own BlurFade:

```tsx
<BlurFade delay={0.3} direction="up">
  <GitHubStars />
</BlurFade>
```

- [ ] **Step 4: Verify it renders**

Run dev server, check the hero section — should see a star icon with the count below the buttons. During loading, shows "Star on GitHub" as fallback.

- [ ] **Step 5: Commit**

```bash
git add docs-site/src/app/api/github-stars/route.ts docs-site/src/components/hero.tsx
git commit -m "feat(docs): add live GitHub stars counter to hero with server-side caching"
```

---

### Task 3: Create Missing Extensions Page

**Files:**
- Create: `docs-site/src/app/docs/extensions/page.md`

The navigation sidebar links to `/docs/extensions` but no page exists. Create a documentation page covering tstack's extension system based on the main CLAUDE.md.

- [ ] **Step 1: Create the page**

Create `docs-site/src/app/docs/extensions/page.md`:

```md
---
title: Extensions
---

Customize and extend tstack with project-level overrides. {% .lead %}

---

## Extension points

All extension points support project-level overrides at `.claude/<type>/`. Priority resolution: project > personal (`extensions/`) > core.

| Extension point | Location | Purpose |
|----------------|----------|---------|
| Commands | `commands/<category>/<name>.md` | Slash commands |
| Agents | `agents/<category>/<name>.md` | Subagents |
| Skills | `skills/<name>/SKILL.md` | Reusable skill modules |
| Hooks | `hooks/hooks.json` + `hooks/*.js` | Pre/post event scripts |
| CLAUDE.md | `CLAUDE.md` | Project-level instructions |
| Settings | `settings.json` | Claude behavior defaults |
| Rules | `rules/` | Path-scoped rule files |
| MCP | `.mcp.json` | MCP server configuration |
| LSP | `.lsp.json` | LSP server configuration |
| Output styles | `output-styles/` | Named response format presets |
| Agent teams | `agents/` composition | Multi-agent coordination |

---

## Adding a command

Drop a `.md` file into `commands/<category>/`:

```yaml
---
name: my-command
description: What it does
argument-hint: "<arg>"
model: sonnet
---
```

Commands are auto-discovered — no registration needed.

---

## Adding an agent

Drop a `.md` file into `agents/<category>/`:

```yaml
---
name: my-agent
description: What it does
model: sonnet
tools: Read, Grep, Glob, Edit, Write, Bash
domain: backend
maxTurns: 30
---
```

The `domain` field enables dynamic discovery. Commands like `/feature` dispatch agents by domain.

---

## Adding a skill

Create `skills/<name>/SKILL.md`:

```yaml
---
name: my-skill
description: What it does
user-invocable: true
---
```

---

## Personal extensions

The `extensions/` directory (gitignored) holds personal add-ons. Same structure as core — commands, agents, skills. Lower priority than project-level overrides but higher than core.

---

## Project templates

Templates in `templates/<project>/` include an `install` script for bootstrapping new projects with tstack conventions.
```

- [ ] **Step 2: Verify the page renders**

Navigate to `http://localhost:3001/docs/extensions` — should render with sidebar active state on "Extensions".

- [ ] **Step 3: Commit**

```bash
git add docs-site/src/app/docs/extensions/page.md
git commit -m "feat(docs): add Extensions documentation page"
```

---

### Task 4: Fix Font Theme Variables

**Files:**
- Modify: `docs-site/src/styles/tailwind.css:39-42` (first `@theme` block)
- Modify: `docs-site/src/styles/tailwind.css:97-99` (`@theme inline` block)

The first `@theme` block (lines 39-42) has two bugs:
- `--font-sans: var(--font-inter)` — `--font-inter` doesn't exist. Geist uses `--font-sans` as its next/font variable.
- `--font-display: var(--font-mono)` — CLAUDE.md says Syne is the display font, but this maps display to mono.

The next/font system in `layout.tsx` creates CSS variables via class names on `<html>`:
- Geist → `--font-sans`
- Syne → `--font-display`
- JetBrains Mono → `--font-mono`

**Important:** In Tailwind v4, the `@theme` block registers design tokens. If `--font-display` and `--font-mono` aren't registered, the `font-display` and `font-mono` utility classes won't work. The correct approach is to use `@theme inline` (which inherits values from the cascade — i.e., from next/font) for all three font tokens.

- [ ] **Step 1: Remove broken overrides from first `@theme` block**

In `tailwind.css`, replace lines 39-42:

```css
  --font-sans: var(--font-inter);
  --font-display: var(--font-mono);
  --font-display--font-feature-settings: 'ss01';
  --font-mono: var(--font-mono);
```

With just:

```css
  --font-display--font-feature-settings: 'ss01';
```

- [ ] **Step 2: Register font tokens in `@theme inline` block**

In the `@theme inline` block (line 97), the existing `--font-sans` and `--font-heading` are already there. Add `--font-display` and `--font-mono` so Tailwind registers these tokens while inheriting values from next/font:

Replace lines 97-99:

```css
@theme inline {
  --font-heading: var(--font-sans);
  --font-sans: var(--font-sans);
```

With:

```css
@theme inline {
  --font-heading: var(--font-sans);
  --font-sans: var(--font-sans);
  --font-display: var(--font-display);
  --font-mono: var(--font-mono);
```

This ensures `font-display` and `font-mono` utility classes resolve to Syne and JetBrains Mono respectively (set by next/font on `<html>`).

- [ ] **Step 3: Verify fonts render correctly**

Run dev server. Check:
- Hero headline should use Syne (the display font) — large, bold, uppercase
- Body text should use Geist (sans)
- Nav labels, code, buttons should use JetBrains Mono
- Run `pnpm build` to confirm no Tailwind compilation errors

- [ ] **Step 4: Commit**

```bash
git add docs-site/src/styles/tailwind.css
git commit -m "fix(docs): correct font theme variables to use Geist/Syne/JetBrains Mono"
```

---

### Task 5: Remove Dead Code

**Files:**
- Delete: `docs-site/src/components/hero-background.tsx`

`hero-background.tsx` is not imported anywhere. The hero now uses `FlickeringGrid` instead.

- [ ] **Step 1: Verify it's unused**

```bash
cd docs-site && grep -r "hero-background\|HeroBackground" src/ --include="*.tsx" --include="*.ts"
```

Should only match the file itself.

- [ ] **Step 2: Delete it**

```bash
rm docs-site/src/components/hero-background.tsx
```

- [ ] **Step 3: Verify build still works**

```bash
cd docs-site && pnpm build
```

- [ ] **Step 4: Commit**

```bash
git add docs-site/src/components/hero-background.tsx
git commit -m "chore(docs): remove unused hero-background component"
```

---

### Task 6: Final Verification

- [ ] **Step 1: Run lint**

```bash
cd docs-site && pnpm lint
```

- [ ] **Step 2: Run build**

```bash
cd docs-site && pnpm build
```

- [ ] **Step 3: Manual smoke test**

Check all pages in the sidebar nav render without 404s. Verify:
- Get Started → navigates to `/docs/quickstart`
- Stars counter appears in hero
- Extensions page renders
- Fonts look correct (Syne headlines, Geist body, JetBrains Mono nav/code)
