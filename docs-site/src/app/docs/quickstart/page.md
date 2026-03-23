---
title: Quickstart
---

Get up and running with tstack in under 5 minutes. {% .lead %}

---

## Install

```shell
git clone https://github.com/taylorsafeclaw/tstack.git ~/tstack
claude plugin add ~/tstack
```

After install, all tstack commands are available in every Claude Code session.

---

## Verify

In a Claude Code session:

```shell
/help
```

You should see tstack commands grouped by category: git, lifecycle, planning, quality, testing, general, utility.

---

## Your first task

```shell
/task "fix the submit button color"
```

That's it. tstack handles context gathering, implementation, quality checks, and committing.

---

## Your first feature

```shell
/feature "add dark mode toggle"
```

tstack will plan, create a branch, dispatch agents, run the quality pipeline, and open a PR.

---

## Uninstall

```shell
claude plugin remove tstack
```

---

## Next steps

- Read the [tier system](/docs/tiers-overview) to understand task vs feature vs mission
- Browse the [command reference](/docs/commands) for all 24 commands
- Check the [quality pipeline](/docs/quality-pipeline) to understand the guardrails
