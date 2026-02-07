//! Structure Search and Module Parsing
//!
//! Finds module structure on disk and parses mod.rs files
//! to extract submodule declarations and re-exports.

use std::fs;
use std::path::{Path, PathBuf};

/// Result of module structure search
#[derive(Debug, Clone)]
pub struct ModuleInfo {
	/// path to the mod.rs or lib.rs file
	pub path: PathBuf,
	/// list of submodule declarations
	pub submodules: Vec<SubmoduleDecl>,
	/// re-exports (pub use statements)
	pub reexports: Vec<String>,
}

/// A submodule declaration
#[derive(Debug, Clone)]
pub struct SubmoduleDecl {
	/// module name
	pub name: String,
	/// visibility (pub or private)
	pub is_public: bool,
	/// doc comment if any
	pub doc: Option<String>,
}

/// Find module structure for a target directory/module
pub fn find_module_structure(
	base_path: &Path,
	target: &str,
) -> Vec<ModuleInfo> {
	let mut results = Vec::new();

	let paths_to_check = vec![
		base_path.join("src").join(target).join("mod.rs"),
		base_path.join("src").join(format!("{}.rs", target)),
		base_path.join(target).join("mod.rs"),
	];

	for mod_path in paths_to_check {
		if !mod_path.exists() {
			continue;
		}
		if let Some(info) = parse_module_file(&mod_path) {
			results.push(info);
			let is_mod_rs = mod_path
				.file_name()
				.map_or(false, |n| n == "mod.rs");
			if is_mod_rs {
				if let Some(parent) = mod_path.parent() {
					results
						.extend(find_nested_modules(parent));
				}
			}
		}
		break;
	}

	results
}

/// List available source directories for suggestions
pub fn list_source_directories(
	base_path: &Path,
) -> Vec<String> {
	let mut dirs = Vec::new();
	let src_path = base_path.join("src");

	if let Ok(entries) = fs::read_dir(&src_path) {
		for entry in entries.filter_map(|e| e.ok()) {
			if entry.path().is_dir() {
				if let Some(name) = entry.file_name().to_str()
				{
					dirs.push(name.to_string());
				}
			}
		}
	}

	dirs.sort();
	dirs
}

/// Find nested mod.rs files in subdirectories
fn find_nested_modules(dir: &Path) -> Vec<ModuleInfo> {
	let mut results = Vec::new();

	let entries = match fs::read_dir(dir) {
		Ok(e) => e,
		Err(_) => return results,
	};

	for entry in entries.filter_map(|e| e.ok()) {
		let path = entry.path();
		if path.is_dir() {
			let mod_path = path.join("mod.rs");
			if mod_path.exists() {
				if let Some(info) =
					parse_module_file(&mod_path)
				{
					results.push(info);
				}
			}
		}
	}

	results
}

/// Parse a mod.rs file to extract submodule declarations
fn parse_module_file(path: &Path) -> Option<ModuleInfo> {
	let content = fs::read_to_string(path).ok()?;

	let mut submodules = Vec::new();
	let mut reexports = Vec::new();
	let mut doc_lines: Vec<String> = Vec::new();

	for line in content.lines() {
		let trimmed = line.trim();

		if trimmed.starts_with("///")
			|| trimmed.starts_with("//!")
		{
			let doc_text = trimmed
				.trim_start_matches("///")
				.trim_start_matches("//!")
				.trim();
			doc_lines.push(doc_text.to_string());
			continue;
		}

		if trimmed.starts_with("pub mod ")
			|| trimmed.starts_with("mod ")
		{
			parse_mod_decl(
				trimmed,
				&mut doc_lines,
				&mut submodules,
			);
		}

		if trimmed.starts_with("pub use ") {
			let reexport = trimmed
				.trim_start_matches("pub use ")
				.trim_end_matches(';')
				.to_string();
			reexports.push(reexport);
		}

		if !trimmed.starts_with("///")
			&& !trimmed.starts_with("//!")
			&& !trimmed.is_empty()
		{
			doc_lines.clear();
		}
	}

	Some(ModuleInfo {
		path: path.to_path_buf(),
		submodules,
		reexports,
	})
}

/// Parse a module declaration line into SubmoduleDecl
fn parse_mod_decl(
	trimmed: &str,
	doc_lines: &mut Vec<String>,
	submodules: &mut Vec<SubmoduleDecl>,
) {
	let is_public = trimmed.starts_with("pub ");
	let name = trimmed
		.trim_start_matches("pub mod ")
		.trim_start_matches("mod ")
		.trim_end_matches(';')
		.trim()
		.to_string();

	if name.is_empty() || name.contains('{') {
		return;
	}

	let doc = if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join(" "))
	};
	submodules.push(SubmoduleDecl {
		name,
		is_public,
		doc,
	});
	doc_lines.clear();
}

