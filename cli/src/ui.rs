use crossterm::{
    cursor, execute,
    style::{Color, SetForegroundColor, ResetColor},
    terminal,
};
use std::io::{self, Write};
use std::time::Duration;
use std::thread;

// ── Color palette ──────────────────────────────────────────────────
pub const LOGO_BASE: (u8, u8, u8) = (45, 65, 130);
pub const LOGO_HIGHLIGHT: (u8, u8, u8) = (140, 170, 240);
pub const ACCENT: (u8, u8, u8) = (90, 120, 200);
pub const GREEN: (u8, u8, u8) = (80, 180, 100);
pub const YELLOW: (u8, u8, u8) = (220, 180, 50);
pub const RED: (u8, u8, u8) = (220, 70, 70);
pub const DIM: (u8, u8, u8) = (100, 105, 115);
pub const WHITE: (u8, u8, u8) = (220, 225, 235);
pub const VERSION_COLOR: (u8, u8, u8) = (70, 80, 110);

// ── Block-art letters (5 rows × 4 cols each) ──────────────────────
const LETTER_T: [[u8; 4]; 5] = [
    [1, 1, 1, 1],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
    [0, 1, 1, 0],
];
const LETTER_S: [[u8; 4]; 5] = [
    [1, 1, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 1, 1],
    [0, 0, 1, 1],
    [1, 1, 1, 1],
];
const LETTER_A: [[u8; 4]; 5] = [
    [1, 1, 1, 1],
    [1, 1, 1, 1],
    [1, 0, 0, 1],
    [1, 1, 1, 1],
    [1, 0, 0, 1],
];
const LETTER_C: [[u8; 4]; 5] = [
    [1, 1, 1, 1],
    [1, 1, 0, 0],
    [1, 1, 0, 0],
    [1, 1, 0, 0],
    [1, 1, 1, 1],
];
const LETTER_K: [[u8; 4]; 5] = [
    [1, 0, 0, 1],
    [1, 0, 1, 0],
    [1, 1, 0, 0],
    [1, 0, 1, 0],
    [1, 0, 0, 1],
];

fn build_logo_grid() -> Vec<Vec<u8>> {
    let letters: Vec<&[[u8; 4]; 5]> = vec![
        &LETTER_T, &LETTER_S, &LETTER_T, &LETTER_A, &LETTER_C, &LETTER_K,
    ];
    let mut rows: Vec<Vec<u8>> = vec![vec![]; 5];
    for (li, letter) in letters.iter().enumerate() {
        for row in 0..5 {
            if li > 0 {
                rows[row].push(0);
            }
            for &col in &letter[row] {
                rows[row].push(col);
            }
        }
    }
    rows
}

fn lerp_color(from: (u8, u8, u8), to: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        (from.0 as f32 + (to.0 as f32 - from.0 as f32) * t) as u8,
        (from.1 as f32 + (to.1 as f32 - from.1 as f32) * t) as u8,
        (from.2 as f32 + (to.2 as f32 - from.2 as f32) * t) as u8,
    )
}

fn rgb(c: (u8, u8, u8)) -> Color {
    Color::Rgb { r: c.0, g: c.1, b: c.2 }
}

/// Write a colored string to stdout
pub fn write_rgb(stdout: &mut io::Stdout, text: &str, color: (u8, u8, u8)) {
    execute!(stdout, SetForegroundColor(rgb(color))).ok();
    write!(stdout, "{}", text).ok();
    execute!(stdout, ResetColor).ok();
}


// ── Logo ───────────────────────────────────────────────────────────

/// Print the logo with shimmer animation + version
pub fn print_logo(version: &str) {
    let grid = build_logo_grid();
    let total_cols = grid[0].len();

    let can_animate = terminal::is_raw_mode_enabled().is_ok()
        && std::env::var("NO_COLOR").is_err()
        && std::env::var("CI").is_err()
        && atty_stdout();

    if !can_animate {
        print_logo_static(version);
        return;
    }

    let mut stdout = io::stdout();
    let shimmer_width: f32 = 6.0;
    let total_frames = 20;
    let frame_duration = Duration::from_millis(35);
    let logo_height = grid.len();

    println!();
    for _ in 0..logo_height {
        println!();
    }
    println!();

    for frame in 0..total_frames {
        let progress = frame as f32 / (total_frames as f32 - 1.0);
        let shimmer_center =
            -shimmer_width + progress * (total_cols as f32 + shimmer_width * 2.0);

        execute!(
            stdout,
            cursor::MoveUp((logo_height + 1) as u16),
            cursor::MoveToColumn(0)
        )
        .ok();

        for row in &grid {
            write!(stdout, "  ").ok();
            for (col_idx, &cell) in row.iter().enumerate() {
                if cell == 1 {
                    let dist = (col_idx as f32 - shimmer_center).abs();
                    let intensity = (1.0 - dist / shimmer_width).max(0.0);
                    let intensity = if intensity > 0.0 {
                        (intensity * std::f32::consts::FRAC_PI_2).sin()
                    } else {
                        0.0
                    };
                    let c = lerp_color(LOGO_BASE, LOGO_HIGHLIGHT, intensity);
                    execute!(stdout, SetForegroundColor(rgb(c))).ok();
                    write!(stdout, "██").ok();
                    execute!(stdout, ResetColor).ok();
                } else {
                    write!(stdout, "  ").ok();
                }
            }
            writeln!(stdout).ok();
        }

        let ver_text = format!("v{version}");
        let logo_width = total_cols * 2 + 2;
        let ver_padding = logo_width.saturating_sub(ver_text.len());
        write_rgb(
            &mut stdout,
            &format!("{:>width$}", ver_text, width = ver_padding),
            VERSION_COLOR,
        );
        writeln!(stdout).ok();
        stdout.flush().ok();
        thread::sleep(frame_duration);
    }

    // Final rest frame
    execute!(
        stdout,
        cursor::MoveUp((logo_height + 1) as u16),
        cursor::MoveToColumn(0)
    )
    .ok();
    render_logo_frame(&mut stdout, &grid, LOGO_BASE);

    let ver_text = format!("v{version}");
    let logo_width = total_cols * 2 + 2;
    let ver_padding = logo_width.saturating_sub(ver_text.len());
    write_rgb(
        &mut stdout,
        &format!("{:>width$}", ver_text, width = ver_padding),
        VERSION_COLOR,
    );
    writeln!(stdout).ok();
    stdout.flush().ok();
    println!();
}

fn render_logo_frame(stdout: &mut io::Stdout, grid: &[Vec<u8>], color: (u8, u8, u8)) {
    for row in grid {
        write!(stdout, "  ").ok();
        for &cell in row.iter() {
            if cell == 1 {
                execute!(stdout, SetForegroundColor(rgb(color))).ok();
                write!(stdout, "██").ok();
                execute!(stdout, ResetColor).ok();
            } else {
                write!(stdout, "  ").ok();
            }
        }
        writeln!(stdout).ok();
    }
}

fn print_logo_static(version: &str) {
    let grid = build_logo_grid();
    let total_cols = grid[0].len();
    let mut stdout = io::stdout();

    println!();
    for row in &grid {
        write!(stdout, "  ").ok();
        for &cell in row.iter() {
            if cell == 1 {
                write_rgb(&mut stdout, "██", LOGO_BASE);
            } else {
                write!(stdout, "  ").ok();
            }
        }
        writeln!(stdout).ok();
    }

    let ver_text = format!("v{version}");
    let logo_width = total_cols * 2 + 2;
    let ver_padding = logo_width.saturating_sub(ver_text.len());
    write_rgb(
        &mut stdout,
        &format!("{:>width$}", ver_text, width = ver_padding),
        VERSION_COLOR,
    );
    writeln!(stdout).ok();
    stdout.flush().ok();
    println!();
}

fn atty_stdout() -> bool {
    use std::os::unix::io::AsRawFd;
    extern "C" {
        fn isatty(fd: i32) -> i32;
    }
    unsafe { isatty(io::stdout().as_raw_fd()) != 0 }
}

// ── Dashboard components ───────────────────────────────────────────

/// Print a section heading
pub fn heading(text: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "◆ ", ACCENT);
    write_rgb(&mut stdout, text, WHITE);
    writeln!(stdout).ok();
    println!();
}

/// Status line with count, label, and health indicator
pub fn status_line(count: usize, label: &str, healthy: usize) {
    let mut stdout = io::stdout();
    let count_str = format!("{count:>2} {label}");

    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "◆ ", ACCENT);
    write_rgb(&mut stdout, &format!("{:<16}", count_str), WHITE);

    if healthy == count {
        write_rgb(&mut stdout, " ● ", GREEN);
        write_rgb(&mut stdout, "all linked", DIM);
    } else {
        let issues = count - healthy;
        write_rgb(&mut stdout, " ● ", YELLOW);
        write_rgb(&mut stdout, &format!("{issues} issue(s)"), YELLOW);
    }
    writeln!(stdout).ok();
}

/// Hook status line
pub fn hook_status_line(count: usize, configured: usize) {
    let mut stdout = io::stdout();
    let count_str = format!("{count:>2} hooks");

    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "◆ ", ACCENT);
    write_rgb(&mut stdout, &format!("{:<16}", count_str), WHITE);

    if configured > 0 {
        write_rgb(&mut stdout, " ● ", GREEN);
        write_rgb(&mut stdout, &format!("{configured} configured"), DIM);
    } else {
        write_rgb(&mut stdout, " ○ ", DIM);
        write_rgb(&mut stdout, "not configured", DIM);
    }
    writeln!(stdout).ok();
}

/// Key-value info line
pub fn info_line(key: &str, value: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, &format!("{:<6}", key), DIM);
    write_rgb(&mut stdout, value, WHITE);
    writeln!(stdout).ok();
}

/// Success item
pub fn success(text: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "✓ ", GREEN);
    write_rgb(&mut stdout, text, WHITE);
    writeln!(stdout).ok();
}

/// Warning item
pub fn warn(text: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "⚠ ", YELLOW);
    write_rgb(&mut stdout, text, YELLOW);
    writeln!(stdout).ok();
}

/// Error item
pub fn error(text: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "✗ ", RED);
    write_rgb(&mut stdout, text, RED);
    writeln!(stdout).ok();
}

/// Dimmed info
pub fn info(text: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, text, DIM);
    writeln!(stdout).ok();
}

/// Section separator with label
pub fn separator(label: &str) {
    let mut stdout = io::stdout();
    let line_len = 44usize.saturating_sub(label.len());
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, label, WHITE);
    write!(stdout, " ").ok();
    write_rgb(&mut stdout, &"─".repeat(line_len), (55, 60, 70));
    writeln!(stdout).ok();
}

/// List item with status dot, name, description, and optional tag
pub fn list_item(status_color: (u8, u8, u8), name: &str, desc: &str, tag: &str) {
    let mut stdout = io::stdout();
    write!(stdout, "  ").ok();
    write_rgb(&mut stdout, "● ", status_color);
    write_rgb(&mut stdout, &format!("{:<24}", name), WHITE);
    write_rgb(&mut stdout, &format!(" {:<42}", truncate_str(desc, 40)), DIM);
    if !tag.is_empty() {
        write_rgb(&mut stdout, tag, DIM);
    }
    writeln!(stdout).ok();
}

/// Truncate a string with ellipsis
fn truncate_str(s: &str, max_chars: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() > max_chars {
        let truncated: String = chars[..max_chars].iter().collect();
        format!("{truncated}…")
    } else {
        s.to_string()
    }
}
