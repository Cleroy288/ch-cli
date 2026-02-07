//! Related Type Discovery for Graph Walker
//!
//! Extends GraphWalker with methods to find types related
//! to a symbol, and parses function signatures to extract
//! type names.

use crate::indexer::{Symbol, SymbolKind};

use super::graph_walker::GraphWalker;
use super::{RelatedType, TypeRelationship};

impl<'a> GraphWalker<'a> {
	/// Find types related to the symbol
	pub fn find_related_types(
		&self,
		symbol: &Symbol,
	) -> Vec<RelatedType> {
		if !self.config().include_related_types {
			return Vec::new();
		}

		let mut related = Vec::new();

		// parse signature to extract types
		if let Some(ref sig) = symbol.signature {
			let types = extract_types_from_signature(sig);
			for type_name in types {
				let rt = self.resolve_type(&type_name, sig);
				related.push(rt);
			}
		}

		// check for trait implementations
		self.check_trait_impl(symbol, &mut related);

		related
	}

	/// Resolve a type name to a RelatedType with location
	fn resolve_type(
		&self,
		type_name: &str,
		sig: &str,
	) -> RelatedType {
		let type_defs = self.graph().find_definitions(type_name);

		let (file, line) = type_defs
			.first()
			.map(|d| {
				let f = d.symbol.location.file.clone();
				let l = d.symbol.location.line;
				(Some(f), Some(l))
			})
			.unwrap_or((None, None));

		// determine relationship from signature position
		let relationship = determine_relationship(
			type_name, sig,
		);

		RelatedType {
			name: type_name.to_string(),
			relationship,
			file,
			line,
		}
	}

	/// Check if symbol is a trait implementation
	fn check_trait_impl(
		&self,
		symbol: &Symbol,
		related: &mut Vec<RelatedType>,
	) {
		if symbol.kind != SymbolKind::Impl {
			return;
		}

		let parent = match symbol.parent {
			Some(ref p) => p,
			None => return,
		};

		// parent might be "TraitName for TypeName"
		if !parent.contains(" for ") {
			return;
		}

		let parts: Vec<&str> = parent.split(" for ").collect();
		if parts.len() == 2 {
			related.push(RelatedType {
				name: parts[0].to_string(),
				relationship: TypeRelationship::Implements,
				file: None,
				line: None,
			});
		}
	}
}

/// Determine relationship based on position in signature
fn determine_relationship(
	type_name: &str,
	sig: &str,
) -> TypeRelationship {
	let is_return = sig.contains(&format!("-> {}", type_name))
		|| sig.contains(&format!("-> Result<{}", type_name));

	if is_return {
		return TypeRelationship::ReturnType;
	}

	if sig.contains(&format!("{}: {}", "", type_name)) {
		return TypeRelationship::ParameterType;
	}

	TypeRelationship::UsedInSignature
}

/// Extract type names from a function signature
pub fn extract_types_from_signature(
	sig: &str,
) -> Vec<String> {
	let mut types = Vec::new();

	// common Rust types to filter out
	let builtin_types = [
		"bool", "char", "str", "String",
		"i8", "i16", "i32", "i64", "i128",
		"u8", "u16", "u32", "u64", "u128",
		"f32", "f64", "isize", "usize",
		"Self", "self", "Option", "Result",
		"Vec", "Box", "Rc", "Arc",
		"HashMap", "HashSet", "BTreeMap", "BTreeSet",
		"Path", "PathBuf",
	];

	// extract words that look like type names
	for word in sig.split(
		|c: char| !c.is_alphanumeric() && c != '_',
	) {
		if word.is_empty() {
			continue;
		}

		let first = word.chars().next().unwrap();
		let is_type = first.is_uppercase()
			&& !builtin_types.contains(&word);

		if is_type && !types.contains(&word.to_string()) {
			types.push(word.to_string());
		}
	}

	types
}

