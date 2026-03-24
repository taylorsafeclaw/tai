use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
#[allow(dead_code)]
pub struct Frontmatter {
    pub name: Option<String>,
    pub description: Option<String>,
    pub model: Option<String>,
    pub tools: Option<String>,
    #[serde(rename = "maxTurns")]
    pub max_turns: Option<u32>,
    #[serde(rename = "argument-hint")]
    pub argument_hint: Option<String>,
    #[serde(rename = "user-invocable")]
    pub user_invocable: Option<bool>,
}

impl Frontmatter {
    pub fn parse(content: &str) -> Result<Self> {
        let content = content.trim();
        if !content.starts_with("---") {
            return Ok(Self::default());
        }

        let rest = &content[3..];
        let end = rest.find("---").context("No closing --- in frontmatter")?;
        let yaml = &rest[..end];

        serde_yaml::from_str(yaml).context("Invalid YAML in frontmatter")
    }

    pub fn from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read {}", path.display()))?;
        Self::parse(&content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn parse_all_fields() {
        let input = r#"---
name: my-command
description: A test command
model: opus
tools: Read, Grep, Glob
maxTurns: 30
argument-hint: "<feature name>"
user-invocable: true
---
Body content here.
"#;
        let fm = Frontmatter::parse(input).unwrap();
        assert_eq!(fm.name.as_deref(), Some("my-command"));
        assert_eq!(fm.description.as_deref(), Some("A test command"));
        assert_eq!(fm.model.as_deref(), Some("opus"));
        assert_eq!(fm.tools.as_deref(), Some("Read, Grep, Glob"));
        assert_eq!(fm.max_turns, Some(30));
        assert_eq!(fm.argument_hint.as_deref(), Some("<feature name>"));
        assert_eq!(fm.user_invocable, Some(true));
    }

    #[test]
    fn parse_partial_fields() {
        let input = "---\nname: partial\nmodel: sonnet\n---\n";
        let fm = Frontmatter::parse(input).unwrap();
        assert_eq!(fm.name.as_deref(), Some("partial"));
        assert_eq!(fm.model.as_deref(), Some("sonnet"));
        assert!(fm.description.is_none());
        assert!(fm.tools.is_none());
        assert!(fm.max_turns.is_none());
        assert!(fm.argument_hint.is_none());
        assert!(fm.user_invocable.is_none());
    }

    #[test]
    fn parse_no_frontmatter_returns_default() {
        let input = "Just some markdown content without frontmatter.";
        let fm = Frontmatter::parse(input).unwrap();
        assert!(fm.name.is_none());
        assert!(fm.description.is_none());
    }

    #[test]
    fn parse_missing_closing_delimiter_returns_error() {
        let input = "---\nname: broken\nno closing delimiter";
        let result = Frontmatter::parse(input);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("No closing ---"));
    }

    #[test]
    fn parse_invalid_yaml_returns_error() {
        let input = "---\n: : : not valid yaml [[\n---\n";
        let result = Frontmatter::parse(input);
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Invalid YAML"));
    }

    #[test]
    fn parse_empty_string_returns_default() {
        let fm = Frontmatter::parse("").unwrap();
        assert!(fm.name.is_none());
        assert!(fm.description.is_none());
        assert!(fm.model.is_none());
    }

    #[test]
    fn parse_extra_unknown_fields_are_ignored() {
        let input = "---\nname: flexible\nunknown_field: surprise\ncustom: 42\n---\n";
        let fm = Frontmatter::parse(input).unwrap();
        assert_eq!(fm.name.as_deref(), Some("flexible"));
    }

    #[test]
    fn from_file_reads_and_parses() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "---\nname: from-file\ndescription: temp test\n---\nBody.").unwrap();
        let fm = Frontmatter::from_file(tmp.path()).unwrap();
        assert_eq!(fm.name.as_deref(), Some("from-file"));
        assert_eq!(fm.description.as_deref(), Some("temp test"));
    }

    #[test]
    fn from_file_nonexistent_path_returns_error() {
        let result = Frontmatter::from_file(Path::new("/tmp/does_not_exist_tstack_test.md"));
        assert!(result.is_err());
    }

    #[test]
    fn parse_whitespace_before_opening_delimiter() {
        let input = "  \n\n---\nname: trimmed\n---\nContent.";
        let fm = Frontmatter::parse(input).unwrap();
        assert_eq!(fm.name.as_deref(), Some("trimmed"));
    }
}
