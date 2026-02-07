//! Type definitions for semantic analysis.

use serde::{Deserialize, Serialize};

use crate::indexer::symbols::{CodeLocation, Symbol};

/// A reference to a symbol (where it's used)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolReference {
	/// The name being referenced
	pub name: String,
	/// Location of the reference
	pub location: CodeLocation,
	/// The kind of reference (if known)
	pub context: ReferenceContext,
}

/// Context in which a reference appears
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferenceContext {
	/// Function/method call
	Call,
	/// Type annotation
	Type,
	/// Field access
	FieldAccess,
	/// Import/use statement
	Import,
	/// Variable/identifier usage
	Identifier,
	/// Unknown context
	Unknown,

	// More specific type contexts
	/// Used as field type: `field: MyType`
	FieldType,
	/// Used as return type: `-> MyType`
	ReturnType,
	/// Used as parameter type: `fn foo(x: MyType)`
	ParameterType,
	/// Used as generic argument: `Vec<MyType>`
	GenericArg,
	/// Used as trait bound: `T: MyTrait`
	TraitBound,
	/// Target of impl: `impl MyTrait for X`
	ImplTarget,
}

impl ReferenceContext {
	/// Check if this is a type-related context
	pub fn is_type_usage(&self) -> bool {
		matches!(
			self,
			Self::Type
				| Self::FieldType
				| Self::ReturnType
				| Self::ParameterType
				| Self::GenericArg
				| Self::TraitBound
				| Self::ImplTarget
		)
	}
}

/// A definition with its location and metadata
#[derive(Debug, Clone)]
pub struct Definition {
	/// The symbol that was defined
	pub symbol: Symbol,
	/// Fully qualified name (e.g., "module::struct::method")
	pub fqn: String,
	/// The scope this definition belongs to
	pub scope: Option<String>,
}

/// Result of resolving a reference
#[derive(Debug, Clone)]
pub struct ResolutionResult {
	/// The reference that was resolved
	pub reference: SymbolReference,
	/// Possible definitions this reference could refer to
	pub definitions: Vec<Definition>,
	/// Confidence score (0.0 - 1.0)
	pub confidence: f32,
}

/// All usages of a symbol
#[derive(Debug, Clone)]
pub struct AllUsages {
	/// The symbol name
	pub name: String,
	/// Where the symbol is defined
	pub definitions: Vec<CodeLocation>,
	/// Where the symbol is referenced
	pub references: Vec<CodeLocation>,
}

impl AllUsages {
	/// Total number of usages (definitions + references)
	pub fn total(&self) -> usize {
		self.definitions.len() + self.references.len()
	}
}

/// Statistics about the semantic graph
#[derive(Debug, Clone)]
pub struct SemanticStats {
	/// Total number of definitions
	pub total_definitions: usize,
	/// Total number of references
	pub total_references: usize,
	/// Number of unique symbol names
	pub unique_symbols: usize,
	/// Number of files analyzed
	pub files_analyzed: usize,
}
