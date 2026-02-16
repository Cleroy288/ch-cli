//! Structure Search and Module Parsing
//!
//! Finds module structure on disk and parses mod.rs
//! files to extract submodule declarations.

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
		let Some(info) = parse_module_file(&mod_path)
		else {
			break;
		};
		results.push(info);
		add_nested_if_mod(&mod_path, &mut results);
		break;
	}

	results
}

/// Generate candidate paths for a target module
fn candidate_paths(
	base_path: &Path,
	target: &str,
) -> Vec<PathBuf> {
	vec![
		base_path
			.join("src")
			.join(target)
			.join("mod.rs"),
		base_path.join("src").join(
			format!("{}.rs", target),
		),
		base_path.join(target).join("mod.rs"),
	]
}

/// Add nested modules if path is a mod.rs
fn add_nested_if_mod(
	mod_path: &Path,
	results: &mut Vec<ModuleInfo>,
) {
	let is_mod = mod_path
		.file_name()
		.is_some_and(|name| name == "mod.rs");
	if let (true, Some(parent)) =
		(is_mod, mod_path.parent())
	{
		results.extend(find_nested_modules(parent));
	}
}

/// List available source directories
pub fn list_source_directories(
	base_path: &Path,
) -> Vec<String> {
	let src_path = base_path.join("src");
	let Ok(entries) = std::fs::read_dir(&src_path)
	else {
		return Vec::new();
	};
	let mut dirs: Vec<String> = entries
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.path().is_dir())
		.filter_map(|entry| {
			entry.file_name().to_str().map(String::from)
		})
		.collect();
	dirs.sort();
	dirs
}

/// Find nested mod.rs files in subdirectories
fn find_nested_modules(
	dir: &Path,
) -> Vec<ModuleInfo> {
	let Ok(entries) = std::fs::read_dir(dir) else {
		return Vec::new();
	};

	entries
		.filter_map(|entry| entry.ok())
		.filter(|entry| entry.path().is_dir())
		.filter_map(|entry| {
			let mod_path = entry.path().join("mod.rs");
			parse_module_file(&mod_path)
		})
		.collect()
}

/// Parse a mod.rs file for submodules and reexports
fn parse_module_file(
	path: &Path,
) -> Option<ModuleInfo> {
	let content =
		std::fs::read_to_string(path).ok()?;

	let mut submodules = Vec::new();
	let mut reexports = Vec::new();
	let mut doc_lines: Vec<String> = Vec::new();

	for line in content.lines() {
		let trimmed = line.trim();
		process_line(
			trimmed,
			&mut doc_lines,
			&mut submodules,
			&mut reexports,
		);
	}

	Some(ModuleInfo {
		path: path.to_path_buf(),
		submodules,
		reexports,
	})
}

/// Process a single line from a module file
fn process_line(
	trimmed: &str,
	doc_lines: &mut Vec<String>,
	submodules: &mut Vec<SubmoduleDecl>,
	reexports: &mut Vec<String>,
) {
	if is_doc_comment(trimmed) {
		let doc_text = trimmed
			.trim_start_matches("///")
			.trim_start_matches("//!")
			.trim();
		doc_lines.push(doc_text.to_string());
		return;
	}

	if is_mod_decl(trimmed) {
		parse_mod_decl(trimmed, doc_lines, submodules);
	}

	if trimmed.starts_with("pub use ") {
		let reexport = trimmed
			.trim_start_matches("pub use ")
			.trim_end_matches(';')
			.to_string();
		reexports.push(reexport);
	}

	if !trimmed.is_empty() && !is_doc_comment(trimmed)
	{
		doc_lines.clear();
	}
}

/// Check if line is a doc comment
fn is_doc_comment(trimmed: &str) -> bool {
	trimmed.starts_with("///")
		|| trimmed.starts_with("//!")
}

/// Check if line is a module declaration
fn is_mod_decl(trimmed: &str) -> bool {
	trimmed.starts_with("pub mod ")
		|| trimmed.starts_with("mod ")
}

/// Parse a module declaration line
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
