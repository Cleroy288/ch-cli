//! Embedding text builder for semantic search.
//!
//! Converts Symbol metadata into rich natural-language text
//! that produces discriminative embeddings. Transformer
//! attention front-weights tokens, so NL label comes first.

use std::path::Path;

use crate::indexer::symbols::SymbolKind;
use crate::indexer::symbols::Visibility;
use crate::indexer::Symbol;

/// Map SymbolKind to a natural-language label
/// for embedding text (front-weighted by attention).
pub fn kind_to_label(kind: &SymbolKind) -> &'static str {
	match kind {
		SymbolKind::Function => "function",
		SymbolKind::Method => "method",
		SymbolKind::Struct => "data structure",
		SymbolKind::Enum => "enumeration",
		SymbolKind::Trait => "trait interface",
		SymbolKind::Impl => "implementation block",
		SymbolKind::Constant => "constant value",
		SymbolKind::Static => "static variable",
		SymbolKind::TypeAlias => "type alias",
		SymbolKind::Module => "module",
		SymbolKind::Macro => "macro",
		SymbolKind::EnumVariant => "enum variant",
		SymbolKind::Field => "struct field",
		SymbolKind::DocumentChunk => "documentation",
	}
}

/// Split identifier into natural-language words.
/// `"embed_text"` -> `"embed text"`,
/// `"VectorStore"` -> `"vector store"`.
pub fn split_identifier(name: &str) -> String {
	let mut words: Vec<String> = Vec::new();
	let mut current = String::new();

	for chr in name.chars() {
		if chr == '_' || chr == '-' {
			push_word(&mut words, &mut current);
		} else if chr.is_uppercase()
			&& !current.is_empty()
		{
			push_word(&mut words, &mut current);
			current.push(chr);
		} else {
			current.push(chr);
		}
	}
	push_word(&mut words, &mut current);
	words.join(" ")
}

/// Flush current accumulator into words list
fn push_word(
	words: &mut Vec<String>,
	current: &mut String,
) {
	if !current.is_empty() {
		words.push(current.to_lowercase());
		current.clear();
	}
}

/// Extract file context from path as space-separated words.
/// `"src/retrieval/hybrid/vector_store.rs"` ->
/// `"retrieval hybrid vector store"`.
pub fn file_context(path: &Path) -> String {
	let raw = path.to_string_lossy().replace('\\', "/");
	let trimmed =
		raw.strip_prefix("src/").unwrap_or(&raw);
	let stripped = trimmed
		.strip_suffix(".rs")
		.or_else(|| trimmed.strip_suffix(".md"))
		.or_else(|| trimmed.strip_suffix(".py"))
		.or_else(|| trimmed.strip_suffix(".ts"))
		.unwrap_or(trimmed);
	let clean =
		stripped.strip_suffix("/mod").unwrap_or(stripped);
	clean
		.split('/')
		.flat_map(|seg| seg.split('_'))
		.collect::<Vec<_>>()
		.join(" ")
}

/// Build rich embedding text for a symbol.
///
/// Format: `"{vis} {label} {name} [of {parent}] in
///   {file_ctx}. {doc}. {sig}"`
/// Visibility and NL label FIRST for transformer
/// attention front-weighting.
pub fn symbol_to_embedding_text(
	symbol: &Symbol,
) -> String {
	let label = kind_to_label(&symbol.kind);
	let split_name = split_identifier(&symbol.name);
	let file_ctx = file_context(&symbol.location.file);
	let vis = visibility_label(&symbol.visibility);

	let prefix =
		format!("{} {} {}", vis, label, split_name);
	let mut text = build_base_text(
		&prefix, &file_ctx, &symbol.parent,
	);
	append_optional(&mut text, &symbol.doc_comment);
	append_optional(&mut text, &symbol.signature);
	text
}

/// Map visibility to a natural-language label
fn visibility_label(vis: &Visibility) -> &'static str {
	match vis {
		Visibility::Public => "public",
		Visibility::PublicCrate => "crate-visible",
		Visibility::PublicSuper => "module-visible",
		Visibility::Private => "private",
	}
}

/// Build base embedding text with optional parent
fn build_base_text(
	prefix: &str,
	file_ctx: &str,
	parent: &Option<String>,
) -> String {
	match parent {
		Some(parent_name) => {
			let parent_words =
				split_identifier(parent_name);
			format!(
				"{} of {} in {}",
				prefix, parent_words, file_ctx,
			)
		}
		None => format!(
			"{} in {}",
			prefix, file_ctx,
		),
	}
}

/// Append optional section with ". " separator
fn append_optional(
	text: &mut String,
	section: &Option<String>,
) {
	if let Some(ref content) = section {
		text.push_str(". ");
		text.push_str(content);
	}
}
