//! Symbol kind enumeration and display formatting.

use std::fmt;

use serde::{Deserialize, Serialize};

/// The kind of symbol (function, struct, trait, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
	/// Function definition
	Function,
	/// Method (function inside impl block)
	Method,
	/// Struct definition
	Struct,
	/// Enum definition
	Enum,
	/// Trait definition
	Trait,
	/// Impl block
	Impl,
	/// Constant definition
	Constant,
	/// Static variable
	Static,
	/// Type alias
	TypeAlias,
	/// Module definition
	Module,
	/// Macro definition
	Macro,
	/// Enum variant
	EnumVariant,
	/// Struct field
	Field,
	/// Documentation chunk (markdown section)
	DocumentChunk,
}

impl fmt::Display for SymbolKind {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let label = match self {
			SymbolKind::Function => "fn",
			SymbolKind::Method => "method",
			SymbolKind::Struct => "struct",
			SymbolKind::Enum => "enum",
			SymbolKind::Trait => "trait",
			SymbolKind::Impl => "impl",
			SymbolKind::Constant => "const",
			SymbolKind::Static => "static",
			SymbolKind::TypeAlias => "type",
			SymbolKind::Module => "mod",
			SymbolKind::Macro => "macro",
			SymbolKind::EnumVariant => "variant",
			SymbolKind::Field => "field",
			SymbolKind::DocumentChunk => "doc",
		};
		write!(fmt, "{}", label)
	}
}

