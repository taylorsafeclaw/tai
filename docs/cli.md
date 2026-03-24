# tstack CLI

A standalone Rust binary for managing the tstack framework outside of Claude Code sessions.

## Install

```bash
cd cli && cargo install --path .
```

Or build without installing:

```bash
cd cli && cargo build --release
# Binary at cli/target/release/tstack
```

## Commands

### `tstack` (no args)

Status dashboard — shows ASCII logo, item counts with link health, and config paths.

### `tstack install`

Registers tstack components with Claude Code. Idempotent — safe to re-run.

### `tstack uninstall`

Removes tstack components from Claude Code. Does not touch project-level `.claude/` files.

### `tstack list [commands|agents|skills|hooks|all]`

Pretty-prints installed items with status, description, and model. Defaults to `all`.

Status icons:
- `●` green — healthy
- `●` red — error (missing or broken)
- `●` yellow — conflict
- `○` dim — not installed

### `tstack add <command|agent|skill> <name>`

Scaffolds a new item with proper frontmatter template. Filenames are bare — the plugin system handles namespacing.

```bash
tstack add command my-thing    # → commands/<category>/my-thing.md
tstack add agent reviewer      # → agents/<category>/reviewer.md
tstack add skill linter        # → skills/linter/SKILL.md
```

### `tstack doctor`

Full diagnostic:
- **Components** — checks all commands, agents, and skills for health status
- **Frontmatter** — validates YAML in all markdown files, warns on missing descriptions
- **Hooks** — lists available hooks and configuration status
- **Templates** — inventories available project templates

### `tstack template list`

Shows available project templates with content counts.

### `tstack template install <name>`

Runs a template's `install` script (e.g., `templates/example/install`).

### `tstack version`

Prints version (from `VERSION` file), tstack root path, and claude home path.

## Configuration

The CLI auto-detects the tstack root directory by searching for a `VERSION` file:

1. `TSTACK_ROOT` environment variable (if set)
2. Parent of the binary's location
3. Current working directory
4. `~/tstack` (fallback)

The Claude home is always `~/.claude/`.

## Color palette

| Color | Usage |
|-------|-------|
| Cyan | Headings, logo, diamond bullets |
| Bold white | Item names, primary text |
| Dim white | Descriptions, secondary text, paths |
| Green | Checkmarks, healthy status |
| Yellow | Warnings, conflicts |
| Red | Errors, broken links |
