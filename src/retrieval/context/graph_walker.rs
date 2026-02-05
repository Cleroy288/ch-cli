//! Graph Walker for Context Expansion
//!
//! Uses SemanticGraph to traverse definitions and references,
//! finding parent scope, related types, and call relationships.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::indexer::{ReferenceContext, SemanticGraph, Symbol, SymbolKind};

use super::{
	CallerInfo, CalleeInfo, ContextConfig, ParentContext, RelatedType, TypeRelationship,
};

/// Walks the semantic graph to find context for symbols
pub struct GraphWalker<'a> {
	/// the semantic graph
	graph: &'a SemanticGraph,
	/// configuration
	config: ContextConfig,
}

impl<'a> GraphWalker<'a> {
	/// Create a new graph walker
	pub fn new(graph: &'a SemanticGraph, config: ContextConfig) -> Self {
		Self { graph, config }
	}

	/// Find the parent context for a symbol
	pub fn find_parent(&self, symbol: &Symbol) -> Option<ParentContext> {
		if !self.config.include_parent {
			return None;
		}

		// check if symbol has a parent field
		if let Some(ref parent_name) = symbol.parent {
			// look up parent definition
			let parent_defs = self.graph.find_definitions(parent_name);
			if let Some(parent_def) = parent_defs.first() {
				return Some(ParentContext {
					name: parent_name.clone(),
					kind: parent_def.symbol.kind,
					file: parent_def.symbol.location.file.clone(),
					line: parent_def.symbol.location.line,
					snippet: None, // filled in by block builder
				});
			}

			// parent not found in graph, create basic info
			return Some(ParentContext {
				name: parent_name.clone(),
				kind: SymbolKind::Struct, // assume struct/impl
				file: symbol.location.file.clone(),
				line: 0,
				snippet: None,
			});
		}

		None
	}

	/// Find symbols that call the given symbol
	pub fn find_callers(&self, symbol: &Symbol) -> Vec<CallerInfo> {
		let mut callers = Vec::new();

		// find references to this symbol that are calls
		let refs = self.graph.find_references(&symbol.name);

		for reference in refs {
			if reference.context == ReferenceContext::Call {
				// find what symbol contains this call
				if let Some(caller) = self.find_containing_symbol(&reference.location.file, reference.location.line) {
					callers.push(CallerInfo {
						name: caller.name.clone(),
						kind: caller.kind,
						file: reference.location.file.clone(),
						line: reference.location.line,
					});
				} else {
					// couldn't find containing symbol, use file-level info
					callers.push(CallerInfo {
						name: "<unknown>".to_string(),
						kind: SymbolKind::Function,
						file: reference.location.file.clone(),
						line: reference.location.line,
					});
				}
			}

			if callers.len() >= self.config.max_callers {
				break;
			}
		}

		callers
	}

	/// Find symbols that the given symbol calls
	pub fn find_callees(&self, symbol: &Symbol) -> Vec<CalleeInfo> {
		let mut callees = Vec::new();

		// find references in the same file near this symbol
		let refs = self.graph.references_in_file(&symbol.location.file);

		// filter to calls within the symbol's scope
		for reference in refs {
			if reference.context == ReferenceContext::Call {
				// check if this reference is within the symbol's line range
				// (rough heuristic: within 50 lines after the symbol start)
				let ref_line = reference.location.line;
				let sym_line = symbol.location.line;

				if ref_line >= sym_line && ref_line <= sym_line + 50 {
					// look up the callee
					let callee_defs = self.graph.find_definitions(&reference.name);
					let (file, line) = callee_defs
						.first()
						.map(|d| (Some(d.symbol.location.file.clone()), Some(d.symbol.location.line)))
						.unwrap_or((None, None));

					let kind = callee_defs.first().map(|d| d.symbol.kind);

					callees.push(CalleeInfo {
						name: reference.name.clone(),
						kind,
						file,
						line,
					});

					if callees.len() >= self.config.max_callees {
						break;
					}
				}
			}
		}

		callees
	}

	/// Find types related to the symbol
	pub fn find_related_types(&self, symbol: &Symbol) -> Vec<RelatedType> {
		if !self.config.include_related_types {
			return Vec::new();
		}

		let mut related = Vec::new();

		// parse signature to extract types
		if let Some(ref sig) = symbol.signature {
			// extract types from signature (simplified parsing)
			let types = extract_types_from_signature(sig);

			for type_name in types {
				// look up the type
				let type_defs = self.graph.find_definitions(&type_name);
				let (file, line) = type_defs
					.first()
					.map(|d| (Some(d.symbol.location.file.clone()), Some(d.symbol.location.line)))
					.unwrap_or((None, None));

				// determine relationship based on position in signature
				let relationship = if sig.contains(&format!("-> {}", type_name))
					|| sig.contains(&format!("-> Result<{}", type_name))
				{
					TypeRelationship::ReturnType
				} else if sig.contains(&format!("{}: {}", "", type_name)) {
					TypeRelationship::ParameterType
				} else {
					TypeRelationship::UsedInSignature
				};

				related.push(RelatedType {
					name: type_name,
					relationship,
					file,
					line,
				});
			}
		}

		// check for trait implementations (look for impl X for Y patterns)
		if symbol.kind == SymbolKind::Impl {
			if let Some(ref parent) = symbol.parent {
				// parent might be "TraitName for TypeName"
				if parent.contains(" for ") {
					let parts: Vec<&str> = parent.split(" for ").collect();
					if parts.len() == 2 {
						related.push(RelatedType {
							name: parts[0].to_string(),
							relationship: TypeRelationship::Implements,
							file: None,
							line: None,
						});
					}
				}
			}
		}

		related
	}

	/// Find the symbol that contains a given location
	fn find_containing_symbol(&self, file: &PathBuf, line: usize) -> Option<&Symbol> {
		let defs = self.graph.definitions_in_file(file);

		// find the definition that contains this line
		// (the one with the largest start line that's still <= the target line)
		let mut best: Option<&Symbol> = None;
		let mut best_line = 0;

		for def in defs {
			let def_line = def.symbol.location.line;
			if def_line <= line && def_line > best_line {
				best = Some(&def.symbol);
				best_line = def_line;
			}
		}

		best
	}

	/// Extract a code snippet with context lines around a location
	pub fn extract_snippet(&self, file: &std::path::Path, line: usize, context_lines: usize) -> Option<String> {
		let file_content = std::fs::read_to_string(file).ok()?; // file content as string
		let lines: Vec<&str> = file_content.lines().collect(); // lines of the file

		// calculate start and end with bounds checking
		let start = line.saturating_sub(context_lines + 1); // line is 1-indexed
		let end = (line + context_lines).min(lines.len());

		if start >= lines.len() {
			return None;
		}

		// build snippet with line numbers
		let snippet: String = lines[start..end]
			.iter()
			.enumerate()
			.map(|(i, line_content)| format!("{:>4} | {}", start + i + 1, line_content))
			.collect::<Vec<_>>()
			.join("\n");

		Some(snippet)
	}

	/// Find all usages of a symbol with full context
	/// Returns a UsageCollection with deduplication
	pub fn find_all_usages(&self, symbol: &Symbol) -> UsageCollection {
		let mut usages = UsageCollection::new(); // collection to store usages

		// find references to this symbol
		let refs = self.graph.find_references(&symbol.name);

		for reference in refs {
			// extract snippet with 2 lines of context
			let snippet = self.extract_snippet(
				&reference.location.file,
				reference.location.line,
				2,
			);

			// find containing symbol
			let containing = self.find_containing_symbol(
				&reference.location.file,
				reference.location.line,
			);

			usages.add(UsageInfo {
				file: reference.location.file.clone(),
				line: reference.location.line,
				context: reference.context,
				snippet,
				containing_symbol: containing.map(|s| s.name.clone()),
			});
		}

		// sort by file for grouped display
		usages.sort_by_file();

		// apply limit from config
		usages.truncate(self.config.max_usages_per_symbol);

		usages
	}
}

/// Information about a single usage of a symbol
#[derive(Debug, Clone)]
pub struct UsageInfo {
	/// File where the usage occurs
	pub file: std::path::PathBuf,
	/// Line number of the usage
	pub line: usize,
	/// Context of the usage (Call, Type, etc.)
	pub context: ReferenceContext,
	/// Code snippet with context lines
	pub snippet: Option<String>,
	/// Name of the symbol containing this usage
	pub containing_symbol: Option<String>,
}

/// Collection of usages with deduplication
/// Ensures no duplicate (file, line) pairs are added
pub struct UsageCollection {
	/// List of unique usages
	usages: Vec<UsageInfo>,
	/// Set of (file, line) pairs for deduplication
	seen: HashSet<(std::path::PathBuf, usize)>,
}

impl UsageCollection {
	/// Create a new empty usage collection
	pub fn new() -> Self {
		Self {
			usages: Vec::new(),
			seen: HashSet::new(),
		}
	}

	/// Add a usage if not already present (deduplication by file+line)
	pub fn add(&mut self, usage: UsageInfo) {
		let key = (usage.file.clone(), usage.line); // key for deduplication
		if !self.seen.contains(&key) {
			self.seen.insert(key);
			self.usages.push(usage);
		}
	}

	/// Check if a file+line pair is already in the collection
	pub fn contains(&self, file: &std::path::Path, line: usize) -> bool {
		self.seen.contains(&(file.to_path_buf(), line))
	}

	/// Sort usages by file path, then by line number
	pub fn sort_by_file(&mut self) {
		self.usages.sort_by(|a, b| {
			a.file.cmp(&b.file).then(a.line.cmp(&b.line))
		});
	}

	/// Truncate to a maximum number of usages
	pub fn truncate(&mut self, limit: usize) {
		self.usages.truncate(limit);
	}

	/// Convert to a vector of UsageInfo
	pub fn into_vec(self) -> Vec<UsageInfo> {
		self.usages
	}

	/// Get the number of usages
	pub fn len(&self) -> usize {
		self.usages.len()
	}

	/// Check if the collection is empty
	pub fn is_empty(&self) -> bool {
		self.usages.is_empty()
	}
}

impl Default for UsageCollection {
	fn default() -> Self {
		Self::new()
	}
}

/// Extract type names from a function signature (simplified)
fn extract_types_from_signature(sig: &str) -> Vec<String> {
	let mut types = Vec::new();

	// common Rust types to filter out
	let builtin_types = [
		"bool", "char", "str", "String", "i8", "i16", "i32", "i64", "i128",
		"u8", "u16", "u32", "u64", "u128", "f32", "f64", "isize", "usize",
		"Self", "self", "Option", "Result", "Vec", "Box", "Rc", "Arc",
		"HashMap", "HashSet", "BTreeMap", "BTreeSet", "Path", "PathBuf",
	];

	// extract words that look like type names (start with uppercase)
	for word in sig.split(|c: char| !c.is_alphanumeric() && c != '_') {
		if !word.is_empty() {
			let first_char = word.chars().next().unwrap();
			if first_char.is_uppercase() && !builtin_types.contains(&word) {
				if !types.contains(&word.to_string()) {
					types.push(word.to_string());
				}
			}
		}
	}

	types
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_types() {
		let sig = "fn parse(input: &str) -> Result<Config, ParseError>";
		let types = extract_types_from_signature(sig);

		assert!(types.contains(&"Config".to_string()));
		assert!(types.contains(&"ParseError".to_string()));
		assert!(!types.contains(&"Result".to_string())); // filtered
	}

	#[test]
	fn test_usage_collection_deduplication() {
		let mut collection = UsageCollection::new(); // collection to test
		let file = std::path::PathBuf::from("/test/file.rs"); // test file path

		// add first usage
		collection.add(UsageInfo {
			file: file.clone(),
			line: 10,
			context: ReferenceContext::Call,
			snippet: Some("test snippet".to_string()),
			containing_symbol: Some("test_fn".to_string()),
		});

		// add duplicate (same file+line)
		collection.add(UsageInfo {
			file: file.clone(),
			line: 10,
			context: ReferenceContext::Type, // different context, same location
			snippet: Some("different snippet".to_string()),
			containing_symbol: Some("other_fn".to_string()),
		});

		// should only have one entry
		assert_eq!(collection.len(), 1);
	}

	#[test]
	fn test_usage_collection_sort_by_file() {
		let mut collection = UsageCollection::new(); // collection to test

		// add usages from different files in random order
		collection.add(UsageInfo {
			file: std::path::PathBuf::from("/z/file.rs"),
			line: 5,
			context: ReferenceContext::Call,
			snippet: None,
			containing_symbol: None,
		});
		collection.add(UsageInfo {
			file: std::path::PathBuf::from("/a/file.rs"),
			line: 10,
			context: ReferenceContext::Call,
			snippet: None,
			containing_symbol: None,
		});
		collection.add(UsageInfo {
			file: std::path::PathBuf::from("/a/file.rs"),
			line: 5,
			context: ReferenceContext::Call,
			snippet: None,
			containing_symbol: None,
		});

		// sort by file
		collection.sort_by_file();

		// convert to vec and check order
		let usages = collection.into_vec();
		assert_eq!(usages[0].file, std::path::PathBuf::from("/a/file.rs"));
		assert_eq!(usages[0].line, 5);
		assert_eq!(usages[1].file, std::path::PathBuf::from("/a/file.rs"));
		assert_eq!(usages[1].line, 10);
		assert_eq!(usages[2].file, std::path::PathBuf::from("/z/file.rs"));
	}

	#[test]
	fn test_usage_collection_truncate() {
		let mut collection = UsageCollection::new(); // collection to test

		// add 5 usages
		for i in 1..=5 {
			collection.add(UsageInfo {
				file: std::path::PathBuf::from(format!("/test/file{}.rs", i)),
				line: i,
				context: ReferenceContext::Call,
				snippet: None,
				containing_symbol: None,
			});
		}

		assert_eq!(collection.len(), 5);

		// truncate to 3
		collection.truncate(3);

		assert_eq!(collection.len(), 3);
	}

	#[test]
	fn test_usage_collection_contains() {
		let mut collection = UsageCollection::new(); // collection to test
		let file = std::path::PathBuf::from("/test/file.rs"); // test file path

		// add a usage
		collection.add(UsageInfo {
			file: file.clone(),
			line: 42,
			context: ReferenceContext::Call,
			snippet: None,
			containing_symbol: None,
		});

		// test contains
		assert!(collection.contains(&file, 42));
		assert!(!collection.contains(&file, 43));
		assert!(!collection.contains(&std::path::PathBuf::from("/other/file.rs"), 42));
	}

	#[test]
	fn test_extract_snippet() {
		use std::io::Write;
		use tempfile::NamedTempFile;

		// create a temporary file with test content
		let mut temp_file = NamedTempFile::new().unwrap();
		writeln!(temp_file, "line 1").unwrap();
		writeln!(temp_file, "line 2").unwrap();
		writeln!(temp_file, "line 3").unwrap();
		writeln!(temp_file, "line 4").unwrap();
		writeln!(temp_file, "line 5").unwrap();

		// create a minimal graph and walker for testing
		let graph = SemanticGraph::new();
		let config = ContextConfig::default();
		let walker = GraphWalker::new(&graph, config);

		// extract snippet around line 3 with 1 line of context
		let snippet = walker.extract_snippet(temp_file.path(), 3, 1);

		assert!(snippet.is_some());
		let snippet_text = snippet.unwrap();

		// should contain lines 2, 3, 4 (line 3 with 1 line context before and after)
		assert!(snippet_text.contains("line 2"));
		assert!(snippet_text.contains("line 3"));
		assert!(snippet_text.contains("line 4"));
	}
}
