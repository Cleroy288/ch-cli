use crate::domain::skill::{SkillEntry, SkillSource};

use super::directory::scan_directory;
use super::paths::{
	project_commands_dir, user_commands_dir,
};

/// Scan user and project command directories for
/// skills and return all discovered entries.
pub fn scan_skills() -> Vec<SkillEntry> {
	let mut skills = Vec::new();
	if let Some(user_dir) = user_commands_dir() {
		scan_directory(
			&user_dir,
			SkillSource::User,
			"",
			&mut skills,
		);
	}
	let project_dir = project_commands_dir();
	scan_directory(
		&project_dir,
		SkillSource::Project,
		"",
		&mut skills,
	);
	skills
}
