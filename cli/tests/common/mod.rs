//! Shared test harness for tstack CLI tests.
//!
//! `TstackFixture` builds a temporary directory tree that mirrors a real tstack
//! installation. Tests describe what they need (commands, agents, skills, hooks,
//! symlinks) and the fixture creates it. When the fixture is dropped the temp
//! directory is cleaned up automatically.
//!
//! This design means tests survive structural changes to the repo — if a new
//! frontmatter field is added or a directory is renamed, you update the fixture
//! builder once and all tests adapt.

use std::fs;
#[cfg(unix)]
use std::os::unix::fs as unix_fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

/// A self-contained tstack directory tree for testing.
pub struct TstackFixture {
    /// Root temp directory — dropped when fixture goes out of scope.
    pub _dir: TempDir,
    /// Path to the fake tstack root (contains commands/, agents/, skills/, etc.)
    pub tstack_root: PathBuf,
    /// Path to the fake ~/.claude/ directory
    pub claude_dir: PathBuf,
}

impl TstackFixture {
    /// Create a new empty fixture with the standard directory structure.
    pub fn new() -> Self {
        let dir = TempDir::new().expect("failed to create temp dir");
        let root = dir.path().join("tstack");
        let claude = dir.path().join(".claude");

        // tstack source dirs
        fs::create_dir_all(root.join("commands")).unwrap();
        fs::create_dir_all(root.join("agents")).unwrap();
        fs::create_dir_all(root.join("skills")).unwrap();
        fs::create_dir_all(root.join("hooks")).unwrap();
        fs::create_dir_all(root.join("templates")).unwrap();

        // VERSION file (makes config detection work)
        fs::write(root.join("VERSION"), "0.0.0-test\n").unwrap();

        // claude target dirs
        fs::create_dir_all(claude.join("commands")).unwrap();
        fs::create_dir_all(claude.join("agents")).unwrap();
        fs::create_dir_all(claude.join("skills")).unwrap();

        Self {
            _dir: dir,
            tstack_root: root,
            claude_dir: claude,
        }
    }

    // ── Command helpers ─────────────────────────────────────────────────

    /// Add a command .md file under commands/<category>/<name>.md
    pub fn add_command(&self, category: &str, name: &str, frontmatter: &str, body: &str) -> PathBuf {
        let dir = self.tstack_root.join("commands").join(category);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{name}.md"));
        fs::write(&path, format!("---\n{frontmatter}\n---\n\n{body}")).unwrap();
        path
    }

    /// Add a command with standard frontmatter fields.
    pub fn add_command_simple(&self, category: &str, name: &str, desc: &str, model: &str) -> PathBuf {
        self.add_command(
            category,
            name,
            &format!("name: {name}\ndescription: {desc}\nmodel: {model}"),
            &format!("# {name}\n\nCommand body."),
        )
    }

    // ── Agent helpers ───────────────────────────────────────────────────

    /// Add an agent .md file under agents/<category>/<name>.md
    pub fn add_agent(&self, category: &str, name: &str, frontmatter: &str, body: &str) -> PathBuf {
        let dir = self.tstack_root.join("agents").join(category);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(format!("{name}.md"));
        fs::write(&path, format!("---\n{frontmatter}\n---\n\n{body}")).unwrap();
        path
    }

    pub fn add_agent_simple(&self, category: &str, name: &str, desc: &str, model: &str) -> PathBuf {
        self.add_agent(
            category,
            name,
            &format!("name: {name}\ndescription: {desc}\nmodel: {model}\ntools: Read, Grep, Glob"),
            &format!("# {name}\n\nAgent body."),
        )
    }

    // ── Skill helpers ───────────────────────────────────────────────────

    /// Add a skill directory with SKILL.md
    pub fn add_skill(&self, name: &str, frontmatter: &str, body: &str) -> PathBuf {
        let dir = self.tstack_root.join("skills").join(name);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("SKILL.md");
        fs::write(&path, format!("---\n{frontmatter}\n---\n\n{body}")).unwrap();
        dir
    }

    pub fn add_skill_simple(&self, name: &str, desc: &str) -> PathBuf {
        self.add_skill(
            name,
            &format!("name: {name}\ndescription: {desc}\nuser-invocable: true"),
            &format!("# {name}\n\nSkill body."),
        )
    }

    // ── Hook helpers ────────────────────────────────────────────────────

    /// Add a .js hook file.
    pub fn add_hook(&self, name: &str, content: &str) -> PathBuf {
        let path = self.tstack_root.join("hooks").join(format!("{name}.js"));
        fs::write(&path, content).unwrap();
        path
    }

    // ── Symlink helpers ─────────────────────────────────────────────────

    /// Create a symlink in the claude target dir pointing to a source file.
    #[cfg(unix)]
    pub fn link_file(&self, target_subdir: &str, filename: &str, source: &Path) -> PathBuf {
        let dest = self.claude_dir.join(target_subdir).join(filename);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        unix_fs::symlink(source, &dest).unwrap();
        dest
    }

    /// Create a regular (non-symlink) file in the claude target dir — for conflict tests.
    pub fn place_regular_file(&self, target_subdir: &str, filename: &str, content: &str) -> PathBuf {
        let dest = self.claude_dir.join(target_subdir).join(filename);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&dest, content).unwrap();
        dest
    }

    /// Create a broken symlink (points to nonexistent target).
    #[cfg(unix)]
    pub fn place_broken_symlink(&self, target_subdir: &str, filename: &str) -> PathBuf {
        let dest = self.claude_dir.join(target_subdir).join(filename);
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        unix_fs::symlink("/nonexistent/path/that/does/not/exist", &dest).unwrap();
        dest
    }

    // ── Raw file helpers ────────────────────────────────────────────────

    /// Write an arbitrary file relative to tstack_root.
    pub fn write_file(&self, relative_path: &str, content: &str) -> PathBuf {
        let path = self.tstack_root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(&path, content).unwrap();
        path
    }

    /// Write a raw .md file (no frontmatter wrapping) at an arbitrary path.
    pub fn write_raw_md(&self, relative_path: &str, content: &str) -> PathBuf {
        self.write_file(relative_path, content)
    }
}
