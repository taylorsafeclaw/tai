# Installation

## Requirements

- macOS or Linux
- Claude Code CLI
- `pnpm` (for quality pipeline)
- `gh` CLI (for PR creation)

## Install (Plugin)

```bash
git clone https://github.com/taylorsafeclaw/tstack.git ~/tstack
claude plugin add ~/tstack
```

The plugin system discovers all commands, agents, skills, and hooks from the `plugin.json` manifest automatically.

After install, all tstack commands (e.g., `/commit`, `/task`, `/ship`) are available in every Claude Code session, namespaced as `tstack:<category>`.

## Verify

```bash
# In a Claude Code session:
/help
```

You should see tstack commands grouped by category (git, lifecycle, planning, quality, testing, general, utility).

## Uninstall

```bash
claude plugin remove tstack
```

Does **not** touch any project-level `.claude/` files or runtime state.

## Update

```bash
cd ~/tstack && git pull
```

Plugin changes take effect on the next Claude Code session. No re-install needed.

---

## Building the CLI (optional)

```bash
cd ~/tstack/cli && cargo install --path .
```

The CLI provides: `tstack doctor`, `tstack add`, `tstack boost/eco/normal`, and delegates AI commands to Claude Code via `claude -p`.

---

## Project Templates

Templates install project-specific agents, commands, and skills into your project's `.claude/` directory. These override plugin-level components when they share a name.

### Using a template

```bash
~/tstack/templates/<template-name>/install
```

### Creating a template

```
~/tstack/templates/<my-project>/
├── install                ← copies components to project .claude/
├── agents/
│   └── <name>.md          ← project-specific agent overrides
├── commands/
│   └── <name>.md          ← project-specific commands
└── skills/
    └── <name>/
        └── SKILL.md       ← project-specific skills
```

See `templates/example/install` for a reference implementation.

---

## Runtime Directory

Many tstack commands write to `.tstack/` in your project root:
- `.tstack/state.json` — mission progress state
- `.tstack/.quality-passed` — quality gate marker
- `.tstack/.agent-log` — agent coordination log

Add `.tstack/` to your project's `.gitignore`.

---

## Boost Mode

Configure model selection via `tstack.local.md` in your project root:

```yaml
---
mode: boost    # normal | boost | economy
---
```

| Type | Normal | Boost | Economy |
|------|--------|-------|---------|
| Gates (validate, status, help) | haiku | sonnet | haiku |
| Work (commit, ship, review) | sonnet | opus | sonnet |
| Reasoning (plan, scope, debug) | opus | opus | sonnet |

Copy the template from `~/tstack/templates/tstack.local.md`.
