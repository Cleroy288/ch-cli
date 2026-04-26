use serde::{Deserialize, Serialize};

use crate::indexer::symbols::{CodeLocation, Symbol};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolReference {
	pub name: String,
	pub location: CodeLocation,
	pub context: ReferenceContext,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ReferenceContext {
	Call,
	Type,
	FieldAccess,
	Import,
	Identifier,
	Unknown,
	FieldType,
	ReturnType,
	ParameterType,
	GenericArg,
	TraitBound,
	ImplTarget,
}

impl ReferenceContext {
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

#[derive(Debug, Clone)]
pub struct Definition {
	pub symbol: Symbol,
	pub fqn: String,
	pub scope: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolutionResult {
	pub reference: SymbolReference,
	pub definitions: Vec<Definition>,
	pub confidence: f32,
}

#[derive(Debug, Clone)]
pub struct AllUsages {
	pub name: String,
	pub definitions: Vec<CodeLocation>,
	pub references: Vec<CodeLocation>,
}

impl AllUsages {
	pub fn total(&self) -> usize {
		self.definitions.len() + self.references.len()
	}
}

#[derive(Debug, Clone)]
pub struct SemanticStats {
	pub total_definitions: usize,
	pub total_references: usize,
	pub unique_symbols: usize,
	pub files_analyzed: usize,
}
