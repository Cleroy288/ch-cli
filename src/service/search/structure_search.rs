use std::path::{Path, PathBuf};

use super::structure_search_parse::parse_module_file;

/// Parsed module file contents
#[derive(Debug, Clone)]
pub struct ModuleInfo {
	pub path: PathBuf,
	pub submodules: Vec<SubmoduleDecl>,
	pub reexports: Vec<String>,
}

/// A `mod` or `pub mod` declaration
#[derive(Debug, Clone)]
pub struct SubmoduleDecl {
	pub name: String,
	pub is_public: bool,
	pub doc: Option<String>,
}

/// Find module structure for a target
pub fn find_module_structure(
	base_path: &Path,
	target: &str,
) -> Vec<ModuleInfo> {
	let mut results = Vec::new();
	let paths = candidate_paths(base_path, target);

	for mod_path in paths {
		if !mod_path.exists() {
			continue;
		}
		let Some(info) =
			parse_module_file(&mod_path)
		else {
			continue;
		};
		results.push(info);
		add_nested(&mod_path, &mut results);
		break;
	}
	results
}

fn candidate_paths(
	base: &Path,
	target: &str,
) -> Vec<PathBuf> {
	vec![
		base.join("src")
			.join(target)
			.join("mod.rs"),
		base.join("src")
			.join(format!("{}.rs", target)),
		base.join(target).join("mod.rs"),
	]
}

fn add_nested(
	mod_path: &Path,
	results: &mut Vec<ModuleInfo>,
) {
	let is_mod = mod_path
		.file_name()
		.is_some_and(|n| n == "mod.rs");
	if let (true, Some(parent)) =
		(is_mod, mod_path.parent())
	{
		results.extend(find_nested(parent));
	}
}

/// List available source directories
pub fn list_source_directories(
	base_path: &Path,
) -> Vec<String> {
	let src = base_path.join("src");
	let Ok(entries) = std::fs::read_dir(&src) else {
		return Vec::new();
	};
	let mut dirs: Vec<String> = entries
		.filter_map(|ent| ent.ok())
		.filter(|ent| ent.path().is_dir())
		.filter_map(|ent| {
			ent.file_name().to_str().map(String::from)
		})
		.collect();
	dirs.sort();
	dirs
}

/// Find nested mod.rs files in subdirectories
fn find_nested(dir: &Path) -> Vec<ModuleInfo> {
	let Ok(entries) = std::fs::read_dir(dir) else {
		return Vec::new();
	};
	entries
		.filter_map(|ent| ent.ok())
		.filter(|ent| ent.path().is_dir())
		.filter_map(|ent| {
			parse_module_file(
				&ent.path().join("mod.rs"),
			)
		})
		.collect()
}
