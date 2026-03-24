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
