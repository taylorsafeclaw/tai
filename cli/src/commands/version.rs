use anyhow::Result;
use crate::config::TstackConfig;
use crate::ui;
use std::io::Write;

pub fn run() -> Result<()> {
    let config = TstackConfig::detect()?;
    let version = config.version();

    let mut stdout = std::io::stdout();
    println!();
    write!(stdout, "  ").ok();
    ui::write_rgb(&mut stdout, "◆ tstack ", ui::ACCENT);
    ui::write_rgb(&mut stdout, &format!("v{version}"), ui::WHITE);
    writeln!(stdout).ok();
    write!(stdout, "  ").ok();
    ui::write_rgb(&mut stdout, "root  ", ui::DIM);
    ui::write_rgb(&mut stdout, &config.tstack_root.display().to_string(), ui::WHITE);
    writeln!(stdout).ok();
    write!(stdout, "  ").ok();
    ui::write_rgb(&mut stdout, "home  ", ui::DIM);
    ui::write_rgb(&mut stdout, &config.claude_dir.display().to_string(), ui::WHITE);
    writeln!(stdout).ok();
    println!();

    Ok(())
}
