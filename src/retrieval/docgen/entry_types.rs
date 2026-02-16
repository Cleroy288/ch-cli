//! Supporting types for documentation entries.
//!
//! Defines status, reference, and linking types
//! used by DocEntry.

use std::fmt;

use serde::{Deserialize, Serialize};

/// Status of documentation generation for an entry.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize, Default,
)]
pub enum DocStatus {
	/// not yet processed
	#[default]
	Pending,
	/// LLM is currently generating doc
	Generating,
	/// doc generation complete
	Ready,
	/// generation failed
	Failed,
}

impl fmt::Display for DocStatus {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let label = match self {
			DocStatus::Pending => "pending",
			DocStatus::Generating => "generating",
			DocStatus::Ready => "ready",
			DocStatus::Failed => "failed",
		};
		write!(fmt, "{}", label)
	}
}

/// Kind of reference between symbols.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub enum ReferenceKind {
	/// function or method call
	Call,
	/// type usage in signature, field, variable
	TypeUsage,
	/// import or use statement
	Import,
	/// trait implementation
	TraitImpl,
	/// derive macro usage
	Derive,
	/// field access on struct/enum
	FieldAccess,
	/// external crate dependency
	ExternalCrate,
	/// generic type parameter
	GenericParam,
	/// return type usage
	ReturnType,
	/// parameter type usage
	ParamType,
}

impl fmt::Display for ReferenceKind {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let label = match self {
			ReferenceKind::Call => "call",
			ReferenceKind::TypeUsage => "type_usage",
			ReferenceKind::Import => "import",
			ReferenceKind::TraitImpl => "trait_impl",
			ReferenceKind::Derive => "derive",
			ReferenceKind::FieldAccess => "field_access",
			ReferenceKind::ExternalCrate => "external_crate",
			ReferenceKind::GenericParam => "generic_param",
			ReferenceKind::ReturnType => "return_type",
			ReferenceKind::ParamType => "param_type",
		};
		write!(fmt, "{}", label)
	}
}
