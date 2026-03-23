---
title: CLI Reference
---

A standalone Rust binary for managing tstack outside of Claude Code sessions. {% .lead %}

---

## Install

```shell
cd cli && cargo install --path .
```

Or build without installing:

```shell
cd cli && cargo build --release
```

---

## Commands

### `tstack` (no args)

Status dashboard — shows ASCII logo, item counts with link health, and config paths.

### `tstack list [commands|agents|skills|hooks|all]`

Pretty-prints installed items with link status, description, and model. Defaults to `all`.

Status icons:
- `●` green — linked and healthy
- `●` red — broken symlink (target missing)
- `●` yellow — conflict (non-symlink file exists)
- `○` dim — not linked

### `tstack add <command|agent|skill> <name>`

Scaffolds a new item with proper frontmatter template.

```shell
tstack add command my-thing
```

```shell
tstack add agent reviewer
```

```shell
tstack add skill linter
```

### `tstack doctor`

Full diagnostic: symlinks, frontmatter validation, hooks, and templates.

### `tstack template list`

Shows available project templates with content counts.

### `tstack template install <name>`

Runs a template's install script.

### `tstack version`

Prints version, tstack root path, and claude home path.

---

## Configuration

The CLI auto-detects the tstack root by searching for a `VERSION` file:

1. `TSTACK_ROOT` environment variable
2. Parent of the binary's location
3. Current working directory
4. `~/tstack` (fallback)
