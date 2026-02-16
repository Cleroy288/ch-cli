//! Parse functions for converting string representations back to types.

use crate::indexer::symbols::{SymbolKind, Visibility};

/// Parse a SymbolKind from its string representation
pub fn parse_symbol_kind(text: &str) -> Option<SymbolKind> {
	match text {
		"fn" => Some(SymbolKind::Function),
		"method" => Some(SymbolKind::Method),
		"struct" => Some(SymbolKind::Struct),
		"enum" => Some(SymbolKind::Enum),
		"trait" => Some(SymbolKind::Trait),
		"impl" => Some(SymbolKind::Impl),
		"const" => Some(SymbolKind::Constant),
		"static" => Some(SymbolKind::Static),
		"type" => Some(SymbolKind::TypeAlias),
		"mod" => Some(SymbolKind::Module),
		"macro" => Some(SymbolKind::Macro),
		"variant" => Some(SymbolKind::EnumVariant),
		"field" => Some(SymbolKind::Field),
		"doc" => Some(SymbolKind::DocumentChunk),
		_ => None,
	}
}

/// Parse a Visibility from its string representation
pub fn parse_visibility(text: &str) -> Visibility {
	match text {
		"pub" => Visibility::Public,
		"pub(crate)" => Visibility::PublicCrate,
		"pub(super)" => Visibility::PublicSuper,
		_ => Visibility::Private,
	}
}
