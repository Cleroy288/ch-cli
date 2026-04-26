use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
	Function,
	Method,
	Struct,
	Enum,
	Trait,
	Impl,
	Constant,
	Static,
	TypeAlias,
	Module,
	Macro,
	EnumVariant,
	Field,
	/// Markdown section
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

