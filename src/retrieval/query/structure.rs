//! Structure Query Detection and Module Parsing
//!
//! Detects queries about module/directory structure and extracts
//! the target directory or module name. Returns parsed module
//! information showing the hierarchy.

use std::fs;
use std::path::{Path, PathBuf};

/// Result of structure query detection
#[derive(Debug, Clone)]
pub struct StructureQuery {
	/// the target directory/module name
	pub target: String,
	/// whether this is a module query (vs directory)
	pub is_module: bool,
}

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

/// Detect if a query is asking about structure
/// Returns Some(StructureQuery) if detected, None otherwise
pub fn detect_structure_query(query: &str) -> Option<StructureQuery> {
	let query_lower = query.to_lowercase(); // lowercase for matching

	// Pattern 1: "modules in X" or "what modules are in X"
	if query_lower.contains("modules in") || query_lower.contains("modules are in") {
		if let Some(target) = extract_target_after(&query_lower, "in ") {
			return Some(StructureQuery {
				target,
				is_module: true,
			});
		}
	}

	// Pattern 2: "what's in X directory" or "what is in X"
	if query_lower.contains("what's in") || query_lower.contains("what is in") {
		if let Some(target) = extract_target_after(&query_lower, "in ") {
			return Some(StructureQuery {
				target,
				is_module: false,
			});
		}
	}

	// Pattern 3: "structure of X" or "X structure"
	if query_lower.contains("structure of") {
		if let Some(target) = extract_target_after(&query_lower, "structure of ") {
			return Some(StructureQuery {
				target,
				is_module: true,
			});
		}
	}

	// Pattern 4: "X module structure" or "X directory structure"
	if query_lower.contains(" structure") {
		let words: Vec<&str> = query_lower.split_whitespace().collect(); // split into words
		for (i, word) in words.iter().enumerate() {
			if *word == "structure" && i > 0 {
				let target = words[i - 1].to_string(); // word before "structure"
				if target != "module" && target != "directory" && target.len() > 2 {
					return Some(StructureQuery {
						target,
						is_module: true,
					});
				}
			}
		}
	}

	None
}

/// Articles and common words to skip when extracting target
const SKIP_WORDS: &[&str] = &["the", "a", "an", "this", "that", "our", "my"];

/// Extract target name after a keyword
fn extract_target_after(query: &str, keyword: &str) -> Option<String> {
	let idx = query.find(keyword)?; // find keyword position
	let rest = &query[idx + keyword.len()..]; // get text after keyword

	// Iterate through words, skipping articles
	for word in rest.split_whitespace() {
		let cleaned = word
			.trim_end_matches("directory")
			.trim_end_matches("module")
			.trim_end_matches("folder")
			.trim_end_matches('?')
			.to_lowercase();

		// Skip articles and common words
		if SKIP_WORDS.contains(&cleaned.as_str()) {
			continue;
		}

		// Return first meaningful word
		if cleaned.len() > 1 {
			return Some(cleaned);
		}
	}

	None
}

/// Find module structure for a target directory/module
pub fn find_module_structure(base_path: &Path, target: &str) -> Vec<ModuleInfo> {
	let mut results = Vec::new(); // collected module info

	// Try common paths for the target module
	let paths_to_check = vec![
		base_path.join("src").join(target).join("mod.rs"),
		base_path.join("src").join(format!("{}.rs", target)),
		base_path.join(target).join("mod.rs"),
	];

	for mod_path in paths_to_check {
		if mod_path.exists() {
			if let Some(info) = parse_module_file(&mod_path) {
				results.push(info);

				// Also check subdirectories for nested mod.rs files
				if let Some(parent) = mod_path.parent() {
					results.extend(find_nested_modules(parent));
				}
			}
			break; // Found the main module, don't check other paths
		}
	}

	results
}

/// Find nested mod.rs files in subdirectories
fn find_nested_modules(dir: &Path) -> Vec<ModuleInfo> {
	let mut results = Vec::new(); // collected nested modules

	let entries = match fs::read_dir(dir) {
		Ok(e) => e,
		Err(_) => return results,
	};

	for entry in entries.filter_map(|e| e.ok()) {
		let path = entry.path(); // entry path
		if path.is_dir() {
			let mod_path = path.join("mod.rs"); // check for mod.rs
			if mod_path.exists() {
				if let Some(info) = parse_module_file(&mod_path) {
					results.push(info);
				}
			}
		}
	}

	results
}

/// Parse a mod.rs file to extract submodule declarations
fn parse_module_file(path: &Path) -> Option<ModuleInfo> {
	let content = fs::read_to_string(path).ok()?; // read file content

	let mut submodules = Vec::new(); // collected submodules
	let mut reexports = Vec::new(); // collected re-exports
	let mut current_doc: Option<String> = None; // current doc comment

	for line in content.lines() {
		let trimmed = line.trim(); // trimmed line

		// Track doc comments
		if trimmed.starts_with("///") || trimmed.starts_with("//!") {
			let doc_text = trimmed
				.trim_start_matches("///")
				.trim_start_matches("//!")
				.trim();
			current_doc = Some(doc_text.to_string());
			continue;
		}

		// Parse module declarations
		if trimmed.starts_with("pub mod ") || trimmed.starts_with("mod ") {
			let is_public = trimmed.starts_with("pub "); // check visibility
			let name = trimmed
				.trim_start_matches("pub mod ")
				.trim_start_matches("mod ")
				.trim_end_matches(';')
				.trim()
				.to_string();

			// Skip inline modules (with braces)
			if !name.is_empty() && !name.contains('{') {
				submodules.push(SubmoduleDecl {
					name,
					is_public,
					doc: current_doc.take(),
				});
			}
		}

		// Parse re-exports
		if trimmed.starts_with("pub use ") {
			let reexport = trimmed
				.trim_start_matches("pub use ")
				.trim_end_matches(';')
				.to_string();
			reexports.push(reexport);
		}

		// Reset doc comment if not followed by mod/use
		if !trimmed.starts_with("///") && !trimmed.starts_with("//!") {
			current_doc = None;
		}
	}

	Some(ModuleInfo {
		path: path.to_path_buf(),
		submodules,
		reexports,
	})
}

/// List available source directories for suggestions
pub fn list_source_directories(base_path: &Path) -> Vec<String> {
	let mut dirs = Vec::new(); // collected directories
	let src_path = base_path.join("src"); // src directory

	if let Ok(entries) = fs::read_dir(&src_path) {
		for entry in entries.filter_map(|e| e.ok()) {
			if entry.path().is_dir() {
				if let Some(name) = entry.file_name().to_str() {
					dirs.push(name.to_string());
				}
			}
		}
	}

	dirs.sort(); // sort alphabetically
	dirs
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_detect_modules_in_pattern() {
		let result = detect_structure_query("modules in retrieval");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "retrieval");
	}

	#[test]
	fn test_detect_what_modules_are_in() {
		let result = detect_structure_query("what modules are in the retrieval directory");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "retrieval");
	}

	#[test]
	fn test_detect_whats_in() {
		let result = detect_structure_query("what's in the indexer directory?");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "indexer");
	}

	#[test]
	fn test_detect_structure_of() {
		let result = detect_structure_query("structure of daemon module");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "daemon");
	}

	#[test]
	fn test_detect_x_structure() {
		let result = detect_structure_query("retrieval structure");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "retrieval");
	}

	#[test]
	fn test_no_false_positive_conceptual() {
		let result = detect_structure_query("how does retrieval work");
		assert!(result.is_none());
	}

	#[test]
	fn test_no_false_positive_search() {
		let result = detect_structure_query("find function in file");
		assert!(result.is_none());
	}

	#[test]
	fn test_no_false_positive_definition() {
		let result = detect_structure_query("BgeEmbedder struct");
		assert!(result.is_none());
	}
}
