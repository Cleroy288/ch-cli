//! Helper functions for the parser module.
//!
//! Contains utility functions for visibility parsing and keyword detection.

use crate::indexer::symbols::Visibility;

/// Parse a visibility modifier string into a Visibility enum.
/// Handles pub, pub(crate), pub(super), and private (default).
pub fn parse_visibility(text: &str) -> Visibility {
	match text.trim() {
		"pub" => Visibility::Public,
		vis if vis.starts_with("pub(crate)") => Visibility::PublicCrate,
		vis if vis.starts_with("pub(super)") => Visibility::PublicSuper,
		_ => Visibility::Private,
	}
}

/// Check if a string is a Rust keyword.
/// Used to filter out keywords from reference extraction.
pub fn is_rust_keyword(word: &str) -> bool {
	matches!(
		word,
		"as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
			| "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match"
			| "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
			| "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe"
			| "use" | "where" | "while" | "async" | "await" | "dyn" | "abstract"
			| "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
			| "typeof" | "unsized" | "virtual" | "yield" | "try"
	)
}
