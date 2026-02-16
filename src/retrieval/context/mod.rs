//! Context Expansion Module
//!
//! Expands matched symbols into contextual blocks by:
//! - Finding parent scope (class, module, impl block)
//! - Finding related types (traits, type parameters)
//! - Finding callers and callees
//! - Extracting source code snippets

pub mod block_builder;
#[doc(hidden)]
pub mod code_extractor;
pub mod context_config;
pub mod file_reader;
pub mod context_xml;
pub mod context_xml_usages;
pub mod graph_walker;
pub mod graph_walker_callers;
pub mod graph_walker_types;
pub mod graph_walker_usage;

use std::path::PathBuf;

pub use block_builder::{BlockBuilder, ContextExpander};
pub use context_config::ContextConfig;
pub use graph_walker::GraphWalker;
pub use graph_walker_usage::{UsageCollection, UsageInfo};

use crate::indexer::{Symbol, SymbolKind};

/// A contextual block containing a symbol with context
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
