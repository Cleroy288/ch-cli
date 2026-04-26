use std::path::PathBuf;

/// Where the skill was discovered
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillSource {
	/// ~/.claude/commands/
	User,
	/// .claude/commands/
	Project,
}

/// A Claude skill parsed from a .md file
#[derive(Debug, Clone)]
pub struct SkillEntry {
	/// Display name (kebab-case, e.g. "clean-code")
	pub name: String,
	/// One-line description from frontmatter
	pub description: String,
	/// Absolute path to the .md file
	pub path: PathBuf,
	/// Where it was found
	pub source: SkillSource,
}
