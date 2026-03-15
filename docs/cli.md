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

Symlinks all commands, agents, and skills from the tstack repo to `~/.claude/`. Equivalent to running the `setup` bash script. Idempotent — safe to re-run.

### `tstack uninstall`

Removes all `tstack-*` symlinks from `~/.claude/commands/`, `~/.claude/agents/`, and `~/.claude/skills/`. Does not touch project-level `.claude/` files.

### `tstack list [commands|agents|skills|hooks|all]`

Pretty-prints installed items with link status, description, and model. Defaults to `all`.

Status icons:
- `●` green — linked and healthy
- `●` red — broken symlink (target missing)
- `●` yellow — conflict (non-symlink file exists)
- `○` dim — not linked

### `tstack add <command|agent|skill> <name>`

Scaffolds a new item with proper frontmatter template. The `tstack-` prefix is added automatically if not provided.

```bash
tstack add command my-thing    # → commands/tstack-my-thing.md
tstack add agent reviewer      # → agents/tstack-reviewer.md
tstack add skill linter        # → skills/tstack-linter/SKILL.md
```

Run `tstack install` after adding to create the symlink.

### `tstack doctor`

Full diagnostic:
- **Symlinks** — checks all items for healthy/broken/missing/conflict status
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
