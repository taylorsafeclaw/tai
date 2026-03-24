use anyhow::{Context, Result};
use std::fs;
use std::os::unix::fs as unix_fs;
use std::path::Path;

use crate::types::LinkStatus;

/// Create a symlink, removing existing symlink if present.
/// Returns false if a non-symlink file exists (conflict).
pub fn create(source: &Path, dest: &Path) -> Result<bool> {
    if dest.is_symlink() {
        fs::remove_file(dest)
            .with_context(|| format!("Could not remove existing symlink: {}", dest.display()))?;
    } else if dest.exists() {
        return Ok(false); // conflict — non-symlink exists
    }

    // Ensure parent directory exists
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    unix_fs::symlink(source, dest)
        .with_context(|| format!("Could not create symlink: {} → {}", dest.display(), source.display()))?;

    Ok(true)
}

/// Remove a symlink if it exists and is a symlink.
pub fn remove(path: &Path) -> Result<bool> {
    if path.is_symlink() {
        fs::remove_file(path)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Check the link status of a destination path relative to an expected source.
pub fn check(source: &Path, dest: &Path) -> LinkStatus {
    if !dest.is_symlink() && !dest.exists() {
        return LinkStatus::Missing;
    }

    if dest.is_symlink() {
        match fs::read_link(dest) {
            Ok(target) => {
                if target == source {
                    if source.exists() {
                        LinkStatus::Linked
                    } else {
                        LinkStatus::Broken
                    }
                } else {
                    LinkStatus::Conflict(format!("points to {}", target.display()))
                }
            }
            Err(_) => LinkStatus::Broken,
        }
    } else {
        LinkStatus::Conflict("non-symlink file exists".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs as unix_fs;
    use tempfile::TempDir;

    // ── create() ──────────────────────────────────────────────

    #[test]
    fn create_symlink_successfully() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let dest = tmp.path().join("dest.md");

        let result = create(&source, &dest).unwrap();

        assert!(result);
        assert!(dest.is_symlink());
        assert_eq!(fs::read_link(&dest).unwrap(), source);
    }

    #[test]
    fn create_replaces_existing_symlink() {
        let tmp = TempDir::new().unwrap();
        let old_source = tmp.path().join("old.md");
        let new_source = tmp.path().join("new.md");
        fs::write(&old_source, "old").unwrap();
        fs::write(&new_source, "new").unwrap();
        let dest = tmp.path().join("dest.md");
        unix_fs::symlink(&old_source, &dest).unwrap();

        let result = create(&new_source, &dest).unwrap();

        assert!(result);
        assert_eq!(fs::read_link(&dest).unwrap(), new_source);
    }

    #[test]
    fn create_conflict_regular_file() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let dest = tmp.path().join("dest.md");
        fs::write(&dest, "existing").unwrap();

        let result = create(&source, &dest).unwrap();

        assert!(!result);
        // Original file should be untouched
        assert_eq!(fs::read_to_string(&dest).unwrap(), "existing");
    }

    #[test]
    fn create_makes_parent_directories() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let dest = tmp.path().join("deep").join("nested").join("dest.md");

        let result = create(&source, &dest).unwrap();

        assert!(result);
        assert!(dest.is_symlink());
    }

    // ── remove() ──────────────────────────────────────────────

    #[test]
    fn remove_existing_symlink() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let link = tmp.path().join("link.md");
        unix_fs::symlink(&source, &link).unwrap();

        let result = remove(&link).unwrap();

        assert!(result);
        assert!(!link.exists());
        assert!(!link.is_symlink());
    }

    #[test]
    fn remove_regular_file_returns_false() {
        let tmp = TempDir::new().unwrap();
        let file = tmp.path().join("file.md");
        fs::write(&file, "hello").unwrap();

        let result = remove(&file).unwrap();

        assert!(!result);
        assert!(file.exists()); // file untouched
    }

    #[test]
    fn remove_nonexistent_returns_false() {
        let tmp = TempDir::new().unwrap();
        let path = tmp.path().join("nope.md");

        let result = remove(&path).unwrap();

        assert!(!result);
    }

    // ── check() ───────────────────────────────────────────────

    #[test]
    fn check_missing() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        let dest = tmp.path().join("dest.md");

        assert!(matches!(check(&source, &dest), LinkStatus::Missing));
    }

    #[test]
    fn check_linked() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let dest = tmp.path().join("dest.md");
        unix_fs::symlink(&source, &dest).unwrap();

        assert!(matches!(check(&source, &dest), LinkStatus::Linked));
    }

    #[test]
    fn check_broken_source_deleted() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        fs::write(&source, "hello").unwrap();
        let dest = tmp.path().join("dest.md");
        unix_fs::symlink(&source, &dest).unwrap();
        fs::remove_file(&source).unwrap();

        assert!(matches!(check(&source, &dest), LinkStatus::Broken));
    }

    #[test]
    fn check_conflict_wrong_target() {
        let tmp = TempDir::new().unwrap();
        let expected_source = tmp.path().join("expected.md");
        let actual_source = tmp.path().join("actual.md");
        fs::write(&actual_source, "hello").unwrap();
        let dest = tmp.path().join("dest.md");
        unix_fs::symlink(&actual_source, &dest).unwrap();

        let status = check(&expected_source, &dest);
        assert!(matches!(status, LinkStatus::Conflict(ref msg) if msg.contains("points to")));
    }

    #[test]
    fn check_conflict_regular_file() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("source.md");
        let dest = tmp.path().join("dest.md");
        fs::write(&dest, "I am a regular file").unwrap();

        let status = check(&source, &dest);
        assert!(matches!(status, LinkStatus::Conflict(ref msg) if msg == "non-symlink file exists"));
    }
}

