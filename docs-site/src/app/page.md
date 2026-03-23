---
title: Install
---

Get tstack running in under 2 minutes. {% .lead %}

{% quick-links %}

{% quick-link title="Quickstart" icon="installation" href="/docs/quickstart" description="Step-by-step guide to your first task and feature." /%}

{% quick-link title="Tiers" icon="presets" href="/docs/tiers-overview" description="Understand the task, feature, and mission tiers." /%}

{% quick-link title="Commands" icon="plugins" href="/docs/commands" description="All 24 slash commands with arguments and models." /%}

{% quick-link title="Quality Pipeline" icon="theming" href="/docs/quality-pipeline" description="The lint → build → test guardrails." /%}

{% /quick-links %}

---

## Requirements

- macOS or Linux
- Claude Code CLI
- `pnpm` (for quality pipeline)
- `gh` CLI (for PR creation)

---

## Install

```shell
git clone https://github.com/taylorsafeclaw/tstack.git ~/tstack
```

```shell
claude plugin add ~/tstack
```

The plugin system discovers all commands, agents, skills, and hooks automatically.

---

## Verify

```shell
/help
```

You should see tstack commands grouped by category.

---

## Uninstall

```shell
claude plugin remove tstack
```

---

## Update

```shell
cd ~/tstack && git pull
```

Plugin changes take effect on the next Claude Code session. No re-install needed.

---

## Building the CLI (optional)

```shell
cd ~/tstack/cli && cargo install --path .
```

The CLI provides `tstack doctor`, `tstack add`, and more.

---

## Runtime directory

Many tstack commands write to `.tstack/` in your project root. Add `.tstack/` to your `.gitignore`.

---

## Boost mode

Configure model selection via `tstack.local.md`:

```yaml
---
mode: boost
---
```

| Type | Normal | Boost | Economy |
|------|--------|-------|---------|
| Gates | haiku | sonnet | haiku |
| Work | sonnet | opus | sonnet |
| Reasoning | opus | opus | sonnet |
