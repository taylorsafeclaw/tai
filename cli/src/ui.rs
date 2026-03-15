use owo_colors::OwoColorize;

pub const LOGO: &str = r#"
  ▄▄▄▄▄  ▄▄▄▄  ▄▄▄▄
    █   █▄▄▄█   █
    █   █   █  ▄█▄
"#;

/// Print the logo in cyan
pub fn print_logo(version: &str) {
    let lines: Vec<&str> = LOGO.trim_end().lines().collect();
    for (i, line) in lines.iter().enumerate() {
        if i == lines.len() - 1 {
            // Last line: logo + version
            print!("{}", line.cyan());
            println!("     {}", format!("v{version}").dimmed());
        } else {
            println!("{}", line.cyan());
        }
    }
    println!();
}

/// Print a section heading: "  ◆ tstack install"
pub fn heading(text: &str) {
    println!("  {} {}", "◆".cyan(), text.bold());
    println!();
}

/// Print a status line: "  ◆ 23 commands    ● all linked"
pub fn status_line(count: usize, label: &str, healthy: usize) {
    let count_str = format!("{count:>2} {label}");
    if healthy == count {
        println!(
            "  {} {:<18} {} {}",
            "◆".cyan(),
            count_str.bold(),
            "●".green(),
            "all linked".dimmed()
        );
    } else {
        let issues = count - healthy;
        println!(
            "  {} {:<18} {} {}",
            "◆".cyan(),
            count_str.bold(),
            "●".yellow(),
            format!("{issues} issue(s)").yellow()
        );
    }
}

/// Print a hook status line (hooks aren't symlinked)
pub fn hook_status_line(count: usize, configured: usize) {
    let count_str = format!("{count:>2} hooks");
    if configured > 0 {
        println!(
            "  {} {:<18} {} {}",
            "◆".cyan(),
            count_str.bold(),
            "●".green(),
            format!("{configured} configured").dimmed()
        );
    } else {
        println!(
            "  {} {:<18} {} {}",
            "◆".cyan(),
            count_str.bold(),
            "○".dimmed(),
            "not configured".dimmed()
        );
    }
}

/// Print a key-value line for the dashboard
pub fn info_line(key: &str, value: &str) {
    println!("  {}  {}", key.dimmed(), value);
}

/// Print a success item: "  ✓ commands/tstack-task.md"
pub fn success(text: &str) {
    println!("  {} {}", "✓".green(), text);
}

/// Print a skip/warning item
pub fn warn(text: &str) {
    println!("  {} {}", "⚠".yellow(), text);
}

/// Print an error item
pub fn error(text: &str) {
    println!("  {} {}", "✗".red(), text);
}

/// Print a dimmed info item
pub fn info(text: &str) {
    println!("  {}", text.dimmed());
}

/// Section separator
pub fn separator(label: &str) {
    println!(
        "  {} {}",
        label.bold(),
        "─".repeat(45 - label.len()).dimmed()
    );
}

