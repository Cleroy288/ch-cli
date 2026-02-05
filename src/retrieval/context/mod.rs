//! Context Expansion Module
//!
//! Expands matched symbols into contextual blocks by:
//! - Finding parent scope (class, module, impl block)
//! - Finding related types (traits, type parameters)
//! - Finding callers and callees
//! - Extracting source code snippets

pub mod block_builder;
pub mod graph_walker;

use std::path::PathBuf;

pub use block_builder::{BlockBuilder, ContextExpander};
pub use graph_walker::{GraphWalker, UsageCollection, UsageInfo};

use crate::indexer::{Symbol, SymbolKind};

/// A contextual block containing a symbol with surrounding context
#[derive(Debug, Clone)]
pub struct ContextualBlock {
	/// the primary symbol
	pub symbol: Symbol,
	/// extracted source code for the symbol
	pub code_snippet: String,
	/// parent scope (module, impl, class)
	pub parent: Option<ParentContext>,
	/// related types (traits, type params)
	pub related_types: Vec<RelatedType>,
	/// symbols that call this symbol
	pub callers: Vec<CallerInfo>,
	/// symbols that this symbol calls
	pub callees: Vec<CalleeInfo>,
	/// documentation comment if available
	pub doc_comment: Option<String>,
	/// number of files that reference this symbol
	pub usage_count: usize,
	/// detailed usage locations across files
	pub usages: Vec<UsageInfo>,
}

/// Parent scope context
#[derive(Debug, Clone)]
pub struct ParentContext {
	/// name of the parent
	pub name: String,
	/// kind of parent (Module, Struct, Impl, etc.)
	pub kind: SymbolKind,
	/// file path
	pub file: PathBuf,
	/// line number
	pub line: usize,
	/// code snippet of parent declaration
	pub snippet: Option<String>,
}

/// Related type information
#[derive(Debug, Clone)]
pub struct RelatedType {
	/// name of the related type
	pub name: String,
	/// relationship kind
	pub relationship: TypeRelationship,
	/// file where type is defined
	pub file: Option<PathBuf>,
	/// line number
	pub line: Option<usize>,
}

/// How a type is related to the primary symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeRelationship {
	/// trait implemented by symbol
	Implements,
	/// type used in signature
	UsedInSignature,
	/// type used in body
	UsedInBody,
	/// return type
	ReturnType,
	/// parameter type
	ParameterType,
	/// field type
	FieldType,
}

/// Information about a caller
#[derive(Debug, Clone)]
pub struct CallerInfo {
	/// name of the calling symbol
	pub name: String,
	/// kind of the caller
	pub kind: SymbolKind,
	/// file where the call occurs
	pub file: PathBuf,
	/// line of the call
	pub line: usize,
}

/// Information about a callee
#[derive(Debug, Clone)]
pub struct CalleeInfo {
	/// name of the called symbol
	pub name: String,
	/// kind of the callee (if known)
	pub kind: Option<SymbolKind>,
	/// file where callee is defined
	pub file: Option<PathBuf>,
	/// line of the callee definition
	pub line: Option<usize>,
}

/// Configuration for context expansion
#[derive(Debug, Clone)]
pub struct ContextConfig {
	/// max number of callers to include
	pub max_callers: usize,
	/// max number of callees to include
	pub max_callees: usize,
	/// max number of usages per symbol to include
	pub max_usages_per_symbol: usize,
	/// context lines before symbol
	pub context_lines_before: usize,
	/// context lines after symbol
	pub context_lines_after: usize,
	/// include parent context
	pub include_parent: bool,
	/// include related types
	pub include_related_types: bool,
}

impl Default for ContextConfig {
	fn default() -> Self {
		Self {
			max_callers: 15,
			max_callees: 15,
			max_usages_per_symbol: 30,
			context_lines_before: 3,
			context_lines_after: 15,
			include_parent: true,
			include_related_types: true,
		}
	}
}

impl ContextualBlock {
	/// Format the block as XML for LLM consumption
	pub fn to_xml(&self) -> String {
		let mut xml = String::new();

		xml.push_str("<context-block>\n");

		// symbol info with usage count
		xml.push_str(&format!(
			"  <symbol kind=\"{}\" name=\"{}\" file=\"{}\" line=\"{}\" usages=\"{}\">\n",
			self.symbol.kind,
			self.symbol.name,
			self.symbol.location.file.display(),
			self.symbol.location.line,
			self.usage_count
		));

		// doc comment
		if let Some(ref doc) = self.doc_comment {
			xml.push_str(&format!("    <doc>{}</doc>\n", escape_xml(doc)));
		}

		// code
		xml.push_str("    <code>\n");
		xml.push_str(&escape_xml(&self.code_snippet));
		xml.push_str("\n    </code>\n");

		xml.push_str("  </symbol>\n");

		// parent
		if let Some(ref parent) = self.parent {
			xml.push_str(&format!(
				"  <parent kind=\"{}\" name=\"{}\" file=\"{}\" line=\"{}\"/>\n",
				parent.kind, parent.name, parent.file.display(), parent.line
			));
		}

		// related types
		if !self.related_types.is_empty() {
			xml.push_str("  <related-types>\n");
			for rt in &self.related_types {
				xml.push_str(&format!(
					"    <type name=\"{}\" relationship=\"{:?}\"/>\n",
					rt.name, rt.relationship
				));
			}
			xml.push_str("  </related-types>\n");
		}

		// callers
		if !self.callers.is_empty() {
			xml.push_str("  <callers>\n");
			for caller in &self.callers {
				xml.push_str(&format!(
					"    <caller name=\"{}\" kind=\"{}\" file=\"{}\" line=\"{}\"/>\n",
					caller.name, caller.kind, caller.file.display(), caller.line
				));
			}
			xml.push_str("  </callers>\n");
		}

		// callees
		if !self.callees.is_empty() {
			xml.push_str("  <callees>\n");
			for callee in &self.callees {
				xml.push_str(&format!(
					"    <callee name=\"{}\"/>\n",
					callee.name
				));
			}
			xml.push_str("  </callees>\n");
		}

		// usages across files - grouped by file for readability
		if !self.usages.is_empty() {
			xml.push_str(&format!("  <usages total=\"{}\">\n", self.usages.len()));

			// group usages by file path for organized output
			let mut by_file: std::collections::HashMap<&std::path::Path, Vec<&UsageInfo>> =
				std::collections::HashMap::new();
			for usage in &self.usages {
				by_file.entry(usage.file.as_path()).or_default().push(usage);
			}

			// sort file paths for consistent output order
			let mut files: Vec<_> = by_file.keys().collect();
			files.sort();

			// emit usages grouped by file
			for file in files {
				let file_usages = &by_file[file]; // usages in this file
				xml.push_str(&format!("    <file path=\"{}\">\n", file.display()));

				for usage in file_usages {
					let context_str = format!("{:?}", usage.context); // context type as string

					// build opening tag with attributes
					xml.push_str(&format!(
						"      <usage line=\"{}\" context=\"{}\"",
						usage.line,
						context_str
					));
					if let Some(ref containing) = usage.containing_symbol {
						xml.push_str(&format!(" in=\"{}\"", containing));
					}
					xml.push_str(">\n");

					// add snippet if present (with XML escaping)
					if let Some(ref snippet) = usage.snippet {
						let escaped = escape_xml(snippet); // escape <, >, & characters
						xml.push_str(&format!("        {}\n", escaped));
					}

					xml.push_str("      </usage>\n");
				}

				xml.push_str("    </file>\n");
			}

			xml.push_str("  </usages>\n");
		}

		xml.push_str("</context-block>");

		xml
	}

	/// Get approximate token count for the block
	pub fn token_count(&self) -> usize {
		// rough estimate: 4 chars per token
		let usage_chars: usize = self.usages.iter()
			.map(|u| u.snippet.as_ref().map(|s| s.len()).unwrap_or(50))
			.sum();

		let total_chars = self.code_snippet.len()
			+ self.doc_comment.as_ref().map(|d| d.len()).unwrap_or(0)
			+ self.symbol.name.len() * 2
			+ self.callers.len() * 20
			+ self.callees.len() * 20
			+ usage_chars;

		total_chars / 4
	}
}

/// Escape XML special characters
fn escape_xml(s: &str) -> String {
	s.replace('&', "&amp;")
		.replace('<', "&lt;")
		.replace('>', "&gt;")
		.replace('"', "&quot;")
		.replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::{CodeLocation, ReferenceContext, SymbolKind};

	#[test]
	fn test_context_config_defaults() {
		let config = ContextConfig::default();
		assert_eq!(config.max_callers, 15);
		assert_eq!(config.max_callees, 15);
		assert_eq!(config.max_usages_per_symbol, 30);
		assert_eq!(config.context_lines_after, 15);
	}

	#[test]
	fn test_context_config_custom() {
		let config = ContextConfig {
			max_callers: 20,
			max_callees: 20,
			max_usages_per_symbol: 50,
			..ContextConfig::default()
		};
		assert_eq!(config.max_callers, 20);
		assert_eq!(config.max_usages_per_symbol, 50);
	}

	/// Helper to create a test ContextualBlock with given usages
	fn create_test_block(usages: Vec<UsageInfo>) -> ContextualBlock {
		let symbol = crate::indexer::Symbol::new(
			"test_fn".to_string(),
			SymbolKind::Function,
			CodeLocation::new(PathBuf::from("/test/main.rs"), 10, 1, 0, 0),
		);
		let usage_count = usages.len(); // store count before moving usages

		ContextualBlock {
			symbol,
			code_snippet: "fn test_fn() {}".to_string(),
			parent: None,
			related_types: Vec::new(),
			callers: Vec::new(),
			callees: Vec::new(),
			doc_comment: None,
			usage_count,
			usages,
		}
	}

	#[test]
	fn test_usages_grouped_by_file_in_xml() {
		// create usages from multiple files
		let usages = vec![
			UsageInfo {
				file: PathBuf::from("/project/src/b_file.rs"),
				line: 20,
				context: ReferenceContext::Call,
				snippet: Some("test_fn()".to_string()),
				containing_symbol: Some("caller_b".to_string()),
			},
			UsageInfo {
				file: PathBuf::from("/project/src/a_file.rs"),
				line: 10,
				context: ReferenceContext::Call,
				snippet: Some("test_fn()".to_string()),
				containing_symbol: Some("caller_a1".to_string()),
			},
			UsageInfo {
				file: PathBuf::from("/project/src/a_file.rs"),
				line: 30,
				context: ReferenceContext::Type,
				snippet: None,
				containing_symbol: Some("caller_a2".to_string()),
			},
		];

		let block = create_test_block(usages); // block with usages from 2 files
		let xml = block.to_xml(); // generate XML output

		// verify usages are grouped by file
		assert!(xml.contains("<usages total=\"3\">"));
		assert!(xml.contains("<file path=\"/project/src/a_file.rs\">"));
		assert!(xml.contains("<file path=\"/project/src/b_file.rs\">"));

		// verify file a_file.rs appears before b_file.rs (sorted order)
		let a_pos = xml.find("a_file.rs").unwrap();
		let b_pos = xml.find("b_file.rs").unwrap();
		assert!(a_pos < b_pos, "Files should be sorted alphabetically");

		// verify both usages from a_file.rs are within the same <file> block
		let a_file_section = &xml[a_pos..b_pos];
		assert!(a_file_section.contains("line=\"10\""));
		assert!(a_file_section.contains("line=\"30\""));
	}

	#[test]
	fn test_usage_xml_contains_snippet() {
		// create usage with a code snippet
		let usages = vec![UsageInfo {
			file: PathBuf::from("/test/file.rs"),
			line: 42,
			context: ReferenceContext::Call,
			snippet: Some("let result = test_fn();".to_string()),
			containing_symbol: Some("main".to_string()),
		}];

		let block = create_test_block(usages); // block with snippet
		let xml = block.to_xml();

		// verify snippet appears in XML output
		assert!(xml.contains("let result = test_fn();"));
		assert!(xml.contains("line=\"42\""));
		assert!(xml.contains("in=\"main\""));
	}

	#[test]
	fn test_usage_xml_escapes_special_chars() {
		// create usage with XML special characters in snippet
		let usages = vec![UsageInfo {
			file: PathBuf::from("/test/file.rs"),
			line: 5,
			context: ReferenceContext::Type,
			snippet: Some("if x < 10 && y > 5 { fn_call() }".to_string()),
			containing_symbol: None,
		}];

		let block = create_test_block(usages); // block with special chars
		let xml = block.to_xml();

		// verify special characters are escaped
		assert!(xml.contains("&lt;")); // < escaped
		assert!(xml.contains("&gt;")); // > escaped
		assert!(xml.contains("&amp;")); // & escaped

		// verify original characters are NOT present unescaped
		assert!(!xml.contains(" < ")); // raw < should not appear
		assert!(!xml.contains(" && ")); // raw && should not appear
	}

	#[test]
	fn test_empty_usages_no_xml_section() {
		// create block with no usages
		let block = create_test_block(Vec::new());
		let xml = block.to_xml();

		// verify no usages section when empty
		assert!(!xml.contains("<usages"));
	}
}
