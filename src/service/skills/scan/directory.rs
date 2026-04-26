use std::path::Path;

use crate::domain::skill::{SkillEntry, SkillSource};
use crate::service::skills::parse::extract_description;

use super::paths::{
	build_namespace, file_stem, is_markdown,
};

/// Recursively scan one directory for .md skill files
/// and push each discovered SkillEntry into `out`.
pub(super) fn scan_directory(
	dir: &Path,
	source: SkillSource,
	namespace: &str,
	out: &mut Vec<SkillEntry>,
) {
	let entries = match std::fs::read_dir(dir) {
		Ok(entries) => entries,
		Err(_) => return,
	};
	for entry in entries.filter_map(Result::ok) {
		let path = entry.path();
		if path.is_dir() {
			let dir_name = file_stem(&path);
			let ns =
				build_namespace(namespace, &dir_name);
			scan_directory(
				&path, source.clone(), &ns, out,
			);
		} else if is_markdown(&path) {
			if let Some(skill) =
				build_entry(&path, &source, namespace)
			{
				out.push(skill);
			}
		}
	}
}

/// Build a SkillEntry from a .md file on disk.
fn build_entry(
	path: &Path,
	source: &SkillSource,
	namespace: &str,
) -> Option<SkillEntry> {
	let stem = file_stem(path);
	let name = build_namespace(namespace, &stem);
	let content = std::fs::read_to_string(path).ok()?;
	let description = extract_description(&content)
		.unwrap_or_else(|| name.clone());
	Some(SkillEntry {
		name,
		description,
		path: path.to_path_buf(),
		source: source.clone(),
	})
}
