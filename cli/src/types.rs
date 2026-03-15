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

/// Scan a directory for tstack-*.md files and return TstackItems
pub fn scan_md_items(
    source_dir: &PathBuf,
    target_dir: &PathBuf,
    item_type: ItemType,
) -> Vec<TstackItem> {
    let mut items = Vec::new();

    let entries = match std::fs::read_dir(source_dir) {
        Ok(entries) => entries,
        Err(_) => return items,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let name = path.file_name().unwrap_or_default().to_string_lossy().to_string();

        if !name.starts_with("tstack-") || !name.ends_with(".md") {
            continue;
        }

        let fm = Frontmatter::from_file(&path).unwrap_or_default();
        let dest = target_dir.join(&name);
        let status = crate::symlink::check(&path, &dest);

        items.push(TstackItem {
            name: fm.name.unwrap_or_else(|| name.trim_end_matches(".md").to_string()),
            description: fm.description.unwrap_or_default(),
            model: fm.model,
            item_type: item_type.clone(),
            source_path: path,
            symlink_path: Some(dest),
            status,
        });
    }

    items.sort_by(|a, b| a.name.cmp(&b.name));
    items
}

/// Scan skills directory (directory-level symlinks for tstack-* dirs)
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
        if !name.starts_with("tstack-") {
            continue;
        }

        // Read SKILL.md frontmatter
        let skill_md = path.join("SKILL.md");
        let fm = if skill_md.exists() {
            Frontmatter::from_file(&skill_md).unwrap_or_default()
        } else {
            Frontmatter::default()
        };

        let dest = target_dir.join(&name);
        let status = crate::symlink::check(&path, &dest);

        items.push(TstackItem {
            name: fm.name.unwrap_or_else(|| name.clone()),
            description: fm.description.unwrap_or_default(),
            model: fm.model,
            item_type: ItemType::Skill,
            source_path: path,
            symlink_path: Some(dest),
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
