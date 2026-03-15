use anyhow::Result;
use owo_colors::OwoColorize;
use crate::config::TstackConfig;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;
    let version = config.version();

    println!();
    println!("  {} {}",
        "◆ tstack".cyan().bold(),
        format!("v{version}").bold()
    );
    println!("  {}  {}", "root".dimmed(), config.tstack_root.display());
    println!("  {}  {}", "home".dimmed(), config.claude_dir.display());
    println!();

    Ok(())
}
