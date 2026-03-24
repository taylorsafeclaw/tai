use std::path::PathBuf;
use crate::config::TstackConfig;
use crate::frontmatter::Frontmatter;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TstackItem {
    pub name: String,
    pub description: String,
    pub model: Option<String>,
    pub item_type: ItemType,
    pub source_path: PathBuf,
    pub symlink_path: Option<PathBuf>,
    pub status: LinkStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ItemType {
    Command,
    Agent,
    Skill,
    Hook,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemType::Command => write!(f, "command"),
            ItemType::Agent => write!(f, "agent"),
            ItemType::Skill => write!(f, "skill"),
            ItemType::Hook => write!(f, "hook"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum LinkStatus {
    Linked,
    Broken,
    Missing,
    Conflict(String),
}

impl LinkStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, LinkStatus::Linked)
    }
}

/// Scan a directory recursively for .md files and return TstackItems.
/// When plugin_active is true, all items are marked as Linked (plugin handles discovery).
pub fn scan_md_items(
    source_dir: &std::path::Path,
    target_dir: &std::path::Path,
    item_type: ItemType,
    plugin_active: bool,
) -> Vec<TstackItem> {
    let mut items = Vec::new();
    walk_md_files(source_dir, target_dir, &item_type, plugin_active, &mut items);
    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

fn walk_md_files(
    current_dir: &std::path::Path,
    target_dir: &std::path::Path,
    item_type: &ItemType,
    plugin_active: bool,
    items: &mut Vec<TstackItem>,
) {
    let entries = match std::fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();

        // Recurse into real subdirectories only (skip symlinks to prevent cycles)
        if path.is_dir() && !path.is_symlink() {
            walk_md_files(&path, target_dir, item_type, plugin_active, items);
            continue;
        }

        let filename = path.file_name().unwrap_or_default().to_string_lossy().to_string();
        if !filename.ends_with(".md") {
            continue;
        }

        let fm = Frontmatter::from_file(&path).unwrap_or_default();

        // Use frontmatter name, or derive from filename
        let display_name = fm.name.unwrap_or_else(|| {
            filename.trim_end_matches(".md").to_string()
        });

        let status = if plugin_active {
            LinkStatus::Linked
        } else {
            let dest = target_dir.join(&filename);
            crate::symlink::check(&path, &dest)
        };

        items.push(TstackItem {
            name: display_name,
            description: fm.description.unwrap_or_default(),
            model: fm.model,
            item_type: item_type.clone(),
            source_path: path,
            symlink_path: None,
            status,
        });
    }
}

/// Scan skills directory — each subdirectory with a SKILL.md is a skill.
pub fn scan_skills(config: &TstackConfig) -> Vec<TstackItem> {
    let source_dir = config.skills_dir();
    let target_dir = config.claude_skills_dir();
    let mut items = Vec::new();

    let entries = match std::fs::read_dir(&source_dir) {
        Ok(entries) => entries,
        Err(_) => return items,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        // Read SKILL.md frontmatter
        let skill_md = path.join("SKILL.md");
        if !skill_md.exists() {
            continue;
        }

        let fm = Frontmatter::from_file(&skill_md).unwrap_or_default();

        let status = if config.plugin_active {
            LinkStatus::Linked
        } else {
            let dest = target_dir.join(&name);
            crate::symlink::check(&path, &dest)
        };

        items.push(TstackItem {
            name: fm.name.unwrap_or_else(|| name.clone()),
            description: fm.description.unwrap_or_default(),
            model: fm.model,
            item_type: ItemType::Skill,
            source_path: path,
            symlink_path: None,
            status,
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

/// Scan hooks directory (not symlinked, just discovered)
pub fn scan_hooks(config: &TstackConfig) -> Vec<TstackItem> {
    let source_dir = config.hooks_dir();
    let mut items = Vec::new();

    let entries = match std::fs::read_dir(&source_dir) {
        Ok(entries) => entries,
        Err(_) => return items,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        if !name.ends_with(".js") {
            continue;
        }

        items.push(TstackItem {
            name: name.trim_end_matches(".js").to_string(),
            description: String::new(),
            model: None,
            item_type: ItemType::Hook,
            source_path: path,
            symlink_path: None,
            status: LinkStatus::Missing, // hooks aren't symlinked
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper: write a .md file with YAML frontmatter
    fn write_md(path: &std::path::Path, name: Option<&str>, desc: Option<&str>, model: Option<&str>) {
        let mut content = String::from("---\n");
        if let Some(n) = name {
            content.push_str(&format!("name: {}\n", n));
        }
        if let Some(d) = desc {
            content.push_str(&format!("description: {}\n", d));
        }
        if let Some(m) = model {
            content.push_str(&format!("model: {}\n", m));
        }
        content.push_str("---\n# Body\n");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn make_config(tmp: &TempDir) -> TstackConfig {
        TstackConfig {
            tstack_root: tmp.path().to_path_buf(),
            claude_dir: tmp.path().join(".claude"),
            plugin_active: true,
        }
    }

    // ── LinkStatus ───────────────────────────────────────────

    #[test]
    fn link_status_is_healthy_only_for_linked() {
        assert!(LinkStatus::Linked.is_healthy());
        assert!(!LinkStatus::Broken.is_healthy());
        assert!(!LinkStatus::Missing.is_healthy());
        assert!(!LinkStatus::Conflict("x".into()).is_healthy());
    }

    // ── scan_md_items ────────────────────────────────────────

    #[test]
    fn scan_md_items_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        fs::create_dir_all(&source).unwrap();
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert!(items.is_empty());
    }

    #[test]
    fn scan_md_items_single_file_with_frontmatter() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        write_md(&source.join("commit.md"), Some("commit"), Some("Git commit helper"), Some("sonnet"));
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "commit");
        assert_eq!(items[0].description, "Git commit helper");
        assert_eq!(items[0].model.as_deref(), Some("sonnet"));
        assert_eq!(items[0].item_type, ItemType::Command);
    }

    #[test]
    fn scan_md_items_recursive_subdirectories() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        write_md(&source.join("git").join("commit.md"), Some("commit"), Some("Commit"), None);
        write_md(&source.join("quality").join("review.md"), Some("review"), Some("Review"), None);
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert_eq!(items.len(), 2);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"commit"));
        assert!(names.contains(&"review"));
    }

    #[test]
    fn scan_md_items_ignores_non_md_files() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        fs::create_dir_all(&source).unwrap();
        write_md(&source.join("valid.md"), Some("valid"), None, None);
        fs::write(source.join("ignore.txt"), "not markdown").unwrap();
        fs::write(source.join("also.js"), "not markdown").unwrap();
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "valid");
    }

    #[test]
    fn scan_md_items_falls_back_to_filename_when_no_name() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("my-command.md"), "---\ndescription: A thing\n---\n# Body\n").unwrap();
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "my-command");
    }

    #[test]
    fn scan_md_items_plugin_active_marks_linked() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        write_md(&source.join("test.md"), Some("test"), None, None);
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        assert_eq!(items.len(), 1);
        assert!(items[0].status.is_healthy());
    }

    #[test]
    fn scan_md_items_plugin_inactive_no_symlink_marks_missing() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        write_md(&source.join("test.md"), Some("test"), None, None);
        let target = tmp.path().join("target");
        fs::create_dir_all(&target).unwrap();

        let items = scan_md_items(&source, &target, ItemType::Command, false);
        assert_eq!(items.len(), 1);
        assert!(matches!(items[0].status, LinkStatus::Missing));
    }

    #[test]
    fn scan_md_items_sorted_by_name() {
        let tmp = TempDir::new().unwrap();
        let source = tmp.path().join("commands");
        write_md(&source.join("zebra.md"), Some("zebra"), None, None);
        write_md(&source.join("alpha.md"), Some("alpha"), None, None);
        write_md(&source.join("middle.md"), Some("middle"), None, None);
        let target = tmp.path().join("target");

        let items = scan_md_items(&source, &target, ItemType::Command, true);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, vec!["alpha", "middle", "zebra"]);
    }

    // ── scan_skills ──────────────────────────────────────────

    #[test]
    fn scan_skills_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        fs::create_dir_all(config.skills_dir()).unwrap();

        let items = scan_skills(&config);
        assert!(items.is_empty());
    }

    #[test]
    fn scan_skills_discovers_skill_with_skill_md() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        let skill_dir = config.skills_dir().join("browse");
        fs::create_dir_all(&skill_dir).unwrap();
        fs::write(
            skill_dir.join("SKILL.md"),
            "---\nname: browse\ndescription: Fast browser\n---\n# Skill\n",
        )
        .unwrap();

        let items = scan_skills(&config);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "browse");
        assert_eq!(items[0].description, "Fast browser");
        assert_eq!(items[0].item_type, ItemType::Skill);
    }

    #[test]
    fn scan_skills_skips_dir_without_skill_md() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        let skills = config.skills_dir();
        let good = skills.join("good");
        fs::create_dir_all(&good).unwrap();
        fs::write(good.join("SKILL.md"), "---\nname: good\n---\n").unwrap();
        let bad = skills.join("bad");
        fs::create_dir_all(&bad).unwrap();
        fs::write(bad.join("README.md"), "not a skill").unwrap();

        let items = scan_skills(&config);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "good");
    }

    #[test]
    fn scan_skills_skips_regular_files() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        let skills = config.skills_dir();
        fs::create_dir_all(&skills).unwrap();
        fs::write(skills.join("stray-file.md"), "not a skill").unwrap();

        let items = scan_skills(&config);
        assert!(items.is_empty());
    }

    // ── scan_hooks ───────────────────────────────────────────

    #[test]
    fn scan_hooks_empty_dir() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        fs::create_dir_all(config.hooks_dir()).unwrap();

        let items = scan_hooks(&config);
        assert!(items.is_empty());
    }

    #[test]
    fn scan_hooks_discovers_js_ignores_json() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        let hooks = config.hooks_dir();
        fs::create_dir_all(&hooks).unwrap();
        fs::write(hooks.join("pre-commit.js"), "// hook").unwrap();
        fs::write(hooks.join("post-push.js"), "// hook").unwrap();
        fs::write(hooks.join("hooks.json"), "{}").unwrap();
        fs::write(hooks.join("README.md"), "# Hooks").unwrap();

        let items = scan_hooks(&config);
        assert_eq!(items.len(), 2);
        let names: Vec<&str> = items.iter().map(|i| i.name.as_str()).collect();
        assert!(names.contains(&"pre-commit"));
        assert!(names.contains(&"post-push"));
        assert!(items.iter().all(|i| i.item_type == ItemType::Hook));
    }

    #[test]
    fn scan_hooks_name_strips_js_extension() {
        let tmp = TempDir::new().unwrap();
        let config = make_config(&tmp);
        let hooks = config.hooks_dir();
        fs::create_dir_all(&hooks).unwrap();
        fs::write(hooks.join("quality-gate.js"), "// hook").unwrap();

        let items = scan_hooks(&config);
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "quality-gate");
    }
}
