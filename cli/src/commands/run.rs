use anyhow::{bail, Result};
use crate::ui;
use std::process::Command;

/// Invoke a tstack slash command via the `claude` CLI.
/// e.g. `tstack run task "fix the bug"` → `claude -p "/tstack-task fix the bug"`
pub fn run(command: String, args: Vec<String>, print_mode: bool) -> Result<()> {
    let base = command.trim_start_matches("tstack-");

    let slash_cmd = if args.is_empty() {
        format!("/tstack-{base}")
    } else {
        format!("/tstack-{base} {}", args.join(" "))
    };

    ui::info_line("invoke", &slash_cmd);

    let mut cmd = Command::new("claude");
    if print_mode {
        cmd.arg("-p");
    }
    cmd.arg(&slash_cmd);

    let status = cmd.status().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow::anyhow!(
                "claude CLI not found on PATH — install Claude Code from https://claude.ai/code"
            )
        } else {
            anyhow::anyhow!("failed to spawn claude: {e}")
        }
    })?;

    if !status.success() {
        bail!("claude exited with status {}", status);
    }

    Ok(())
}
