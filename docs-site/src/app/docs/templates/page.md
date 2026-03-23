---
title: Templates
---

Project templates install domain-specific agents, commands, and skills into your project's `.claude/` directory. {% .lead %}

---

## Using a template

```shell
~/tstack/templates/<template-name>/install
```

Or via the CLI:

```shell
tstack template install <name>
```

---

## Template structure

```
~/tstack/templates/<project>/
├── install                ← copies components to project .claude/
├── agents/
│   └── <name>.md          ← project-specific agent overrides
├── commands/
│   └── <name>.md          ← project-specific commands
└── skills/
    └── <name>/
        └── SKILL.md       ← project-specific skills
```

---

## How install works

The `install` script should:
1. Find the project root (directory with `.claude/`)
2. `mkdir -p .claude/agents .claude/commands .claude/skills`
3. Copy template files into the project

See `templates/example/install` for a reference implementation.

---

## Template overrides

Project-level components override global ones when they share a name. This is how templates customize behavior — they install project-specific agents that know your schema, auth patterns, and conventions.

---

## Available templates

| Template | Description |
|----------|-------------|
| `example` | Minimal stub showing template structure |
| `safeclaw` | Full agent team for Convex + Next.js apps |

---

## Creating a template

1. Create `templates/<name>/`
2. Add an `install` script
3. Add agents, commands, or skills as needed
4. Test with `tstack template install <name>`

Templates are the recommended way to distribute project-specific tstack configurations.
