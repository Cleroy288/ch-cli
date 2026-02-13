# Module Structure Queries Implementation Plan

**Date:** 2026-02-04
**Status:** Planning
**Priority:** 4 (MEDIUM impact, MEDIUM effort)
**Goal:** Detect and handle queries about module structure like "modules in retrieval"

---

## Summary

When users ask structural questions like:
- "modules in retrieval"
- "what's in the retrieval directory"
- "structure of indexer module"

rustean should return the relevant `mod.rs` files showing the module hierarchy, not keyword search results.

---

## Current Behavior

**Query:** "what modules are in retrieval"

**rustean returns:**
```
1. doc Query 6: "What modules are in retrieval directory" (rustean-vs-augment-gaps.md:83)
2. doc Expected Result: (TEST_SCENARIOS.md:177)
3. doc Retrieval Quality (Unchanged) (rustean-vs-augment-mcp-metal.md:94)
...
7. mod retrieval (lib.rs:9)
```

**Augment MCP returns:**
- `src/retrieval/mod.rs` with full content showing all submodules
- Related mod.rs files in subdirectories

---

## Target Behavior

**Query:** "modules in retrieval"

**Expected output:**
```
Module structure for 'retrieval':

📁 src/retrieval/mod.rs
   pub mod agent;
   pub mod context;
   pub mod daemon;
   pub mod docgen;
   pub mod hybrid;
   pub mod models;
   pub mod query;
   pub mod rerank;

📁 src/retrieval/agent/mod.rs
   pub mod feedback;
   pub mod output;
   pub mod pipeline;

📁 src/retrieval/daemon/mod.rs
   pub mod async_client;
   pub mod client;
   pub mod lifecycle;
   pub mod protocol;
   pub mod server;
...
```

---

## Implementation Plan

### Phase 1: Add Structure Query Detection

**File:** `src/retrieval/query/structure.rs` (NEW)

```rust
//! Structure Query Detection
//!
//! Detects queries about module/directory structure and extracts
//! the target directory or module name.

use regex::Regex;

/// Result of structure query detection
#[derive(Debug, Clone)]
pub struct StructureQuery {
	/// the target directory/module name
	pub target: String,
	/// whether this is a module query (vs directory)
	pub is_module: bool,
}

/// Detect if a query is asking about structure
/// Returns Some(StructureQuery) if detected, None otherwise
pub fn detect_structure_query(query: &str) -> Option<StructureQuery> {
	let query_lower = query.to_lowercase();

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
		let words: Vec<&str> = query_lower.split_whitespace().collect();
		for (i, word) in words.iter().enumerate() {
			if *word == "structure" && i > 0 {
				let target = words[i - 1].to_string();
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

/// Extract target name after a keyword
fn extract_target_after(query: &str, keyword: &str) -> Option<String> {
	if let Some(idx) = query.find(keyword) {
		let rest = &query[idx + keyword.len()..];
		// Get first word, strip common suffixes
		let target = rest
			.split_whitespace()
			.next()?
			.trim_end_matches("directory")
			.trim_end_matches("module")
			.trim_end_matches("folder")
			.trim_end_matches('?')
			.to_string();

		if target.len() > 1 {
			return Some(target);
		}
	}
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_modules_in_pattern() {
		let result = detect_structure_query("modules in retrieval");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "retrieval");
	}

	#[test]
	fn test_what_modules_are_in() {
		let result = detect_structure_query("what modules are in the retrieval directory");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "retrieval");
	}

	#[test]
	fn test_whats_in() {
		let result = detect_structure_query("what's in the indexer directory?");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "indexer");
	}

	#[test]
	fn test_structure_of() {
		let result = detect_structure_query("structure of daemon module");
		assert!(result.is_some());
		assert_eq!(result.unwrap().target, "daemon");
	}

	#[test]
	fn test_non_structure_query() {
		let result = detect_structure_query("how does BgeEmbedder work");
		assert!(result.is_none());
	}
}
```

### Phase 2: Add Module Structure Search

**File:** `src/retrieval/query/structure.rs` (append)

```rust
use std::path::{Path, PathBuf};
use std::fs;

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
pub fn find_module_structure(base_path: &Path, target: &str) -> Vec<ModuleInfo> {
	let mut results = Vec::new();

	// Try common paths
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
		}
	}

	results
}

/// Find nested mod.rs files in subdirectories
fn find_nested_modules(dir: &Path) -> Vec<ModuleInfo> {
	let mut results = Vec::new();

	if let Ok(entries) = fs::read_dir(dir) {
		for entry in entries.filter_map(|e| e.ok()) {
			let path = entry.path();
			if path.is_dir() {
				let mod_path = path.join("mod.rs");
				if mod_path.exists() {
					if let Some(info) = parse_module_file(&mod_path) {
						results.push(info);
					}
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
	let mut current_doc: Option<String> = None;

	for line in content.lines() {
		let trimmed = line.trim();

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
			let is_public = trimmed.starts_with("pub ");
			let name = trimmed
				.trim_start_matches("pub mod ")
				.trim_start_matches("mod ")
				.trim_end_matches(';')
				.trim()
				.to_string();

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
```

### Phase 3: Update Query Module Exports

**File:** `src/retrieval/query/mod.rs`

Add to exports:
```rust
pub mod structure;
pub use structure::{detect_structure_query, find_module_structure, ModuleInfo, StructureQuery};
```

### Phase 4: Integrate into Search Command

**File:** `src/cli/commands.rs`

Modify `search_command` function:

```rust
use crate::retrieval::query::{detect_structure_query, find_module_structure};

/// Execute the `search` command
pub fn search_command(
	query: &str,
	limit: usize,
	fuzzy: bool,
	kind: Option<&str>,
	semantic: bool,
	context: bool,
	rerank: bool,
) -> CommandResult {
	// NEW: Check for structure query first
	if let Some(structure_query) = detect_structure_query(query) {
		return handle_structure_query(&structure_query);
	}

	// ... rest of existing search logic
}

/// Handle a module structure query
fn handle_structure_query(query: &StructureQuery) -> CommandResult {
	let base_path = std::path::Path::new(".");
	let modules = find_module_structure(base_path, &query.target);

	if modules.is_empty() {
		println!("No module structure found for '{}'", query.target);
		println!("\nTry one of these directories:");

		// List available directories under src/
		if let Ok(entries) = std::fs::read_dir("src") {
			for entry in entries.filter_map(|e| e.ok()) {
				if entry.path().is_dir() {
					if let Some(name) = entry.file_name().to_str() {
						println!("  - {}", name);
					}
				}
			}
		}
		return Ok(());
	}

	println!("Module structure for '{}':\n", query.target);

	for module in modules {
		// Show file path
		println!("📁 {}", module.path.display());

		// Show submodules
		if !module.submodules.is_empty() {
			for sub in &module.submodules {
				let vis = if sub.is_public { "pub mod" } else { "mod" };
				if let Some(ref doc) = sub.doc {
					println!("   {} {}; // {}", vis, sub.name, doc);
				} else {
					println!("   {} {};", vis, sub.name);
				}
			}
		}

		// Show re-exports (summarized)
		if !module.reexports.is_empty() {
			println!("   // {} re-exports", module.reexports.len());
		}

		println!();
	}

	Ok(())
}
```

### Phase 5: Add Tests

**File:** `tests/structure_query_tests.rs` (NEW)

```rust
//! Tests for module structure query detection and search

use rustean::retrieval::query::{detect_structure_query, find_module_structure};
use std::path::Path;

#[test]
fn test_detect_modules_in() {
	let result = detect_structure_query("modules in retrieval");
	assert!(result.is_some());
	let query = result.unwrap();
	assert_eq!(query.target, "retrieval");
	assert!(query.is_module);
}

#[test]
fn test_detect_whats_in() {
	let result = detect_structure_query("what's in indexer");
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "indexer");
}

#[test]
fn test_detect_structure_of() {
	let result = detect_structure_query("structure of daemon");
	assert!(result.is_some());
	assert_eq!(result.unwrap().target, "daemon");
}

#[test]
fn test_no_false_positive() {
	// Should NOT match
	assert!(detect_structure_query("how does retrieval work").is_none());
	assert!(detect_structure_query("find function in file").is_none());
	assert!(detect_structure_query("BgeEmbedder struct").is_none());
}

#[test]
fn test_find_retrieval_modules() {
	let base = Path::new(".");
	let modules = find_module_structure(base, "retrieval");
	assert!(!modules.is_empty());

	// Should find src/retrieval/mod.rs
	let main_mod = modules.iter().find(|m| m.path.ends_with("retrieval/mod.rs"));
	assert!(main_mod.is_some());

	// Should have submodules
	let main = main_mod.unwrap();
	assert!(main.submodules.iter().any(|s| s.name == "agent"));
	assert!(main.submodules.iter().any(|s| s.name == "daemon"));
	assert!(main.submodules.iter().any(|s| s.name == "hybrid"));
}
```

---

## File Changes Summary

| File | Action | Changes |
|------|--------|---------|
| `src/retrieval/query/structure.rs` | CREATE | Structure query detection and module parsing |
| `src/retrieval/query/mod.rs` | MODIFY | Add exports for structure module |
| `src/cli/commands.rs` | MODIFY | Add structure query handling to search_command |
| `tests/structure_query_tests.rs` | CREATE | Integration tests |

---

## Implementation Order

1. **Create `structure.rs`** with detection logic and tests
2. **Add module parsing** functions
3. **Update `query/mod.rs`** exports
4. **Integrate into `commands.rs`**
5. **Add integration tests**
6. **Test manually** with benchmark queries

---

## Testing Plan

### Unit Tests
```bash
cargo test structure --lib
```

### Manual Tests
```bash
# Should return module structure
rustean search "modules in retrieval"
rustean search "what's in daemon directory"
rustean search "structure of indexer"

# Should NOT trigger structure query
rustean search "how does retrieval work"
rustean search "RetrievalError definition"
```

### Expected Output
```
$ rustean search "modules in retrieval"

Module structure for 'retrieval':

📁 src/retrieval/mod.rs
   pub mod agent;
   pub mod context;
   pub mod daemon;
   pub mod docgen;
   pub mod hybrid;
   pub mod models;
   pub mod query;
   pub mod rerank;
   // 12 re-exports

📁 src/retrieval/agent/mod.rs
   pub mod feedback;
   pub mod output;
   pub mod pipeline;
   // 6 re-exports

📁 src/retrieval/daemon/mod.rs
   pub mod async_client;
   pub mod client;
   pub mod lifecycle;
   pub mod protocol;
   pub mod server;
   // 8 re-exports
```

---

## Success Criteria

1. Structure queries are detected with >95% accuracy
2. No false positives on regular search queries
3. Returns correct mod.rs files for valid targets
4. Graceful fallback when target not found
5. All tests pass

---

## Future Enhancements

- Show full file content with `--full` flag
- Support for non-Rust module systems (Python `__init__.py`, JS `index.js`)
- Graphical tree view of module hierarchy
- Integration with semantic graph for cross-references
