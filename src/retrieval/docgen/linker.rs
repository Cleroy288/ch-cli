//! Cross-reference linker for documentation entries.
//!
//! Builds comprehensive symbol graphs with dependencies,
//! dependents, and external crate tracking.

use std::collections::HashSet;

use regex::Regex;

use crate::indexer::semantic::SemanticGraph;
use crate::indexer::Symbol;
use crate::retrieval::docgen::linker_analysis::{
	convert_reference_context, extract_context_lines,
	is_std_module,
};
use crate::retrieval::docgen::linker_paths::get_module_path;
use crate::retrieval::docgen::linker_symbols::{
	extract_external_deps, find_children,
	find_containing_symbol, find_dependencies,
};
use crate::retrieval::docgen::reference_location::ReferenceLocation;
use crate::retrieval::docgen::DocStore;

/// Collection of reference locations and dependent names
type RefCollection =
	(Vec<ReferenceLocation>, HashSet<String>);

/// Context for linking: graph + symbols
pub struct LinkContext<'ctx> {
	/// semantic graph with call references
	pub graph: &'ctx SemanticGraph,
	/// all symbols in the project
	pub symbols: &'ctx [Symbol],
}

/// Builds cross-reference links between doc entries.
pub struct DocLinker {
	/// regex for extracting external crate names
	use_regex: Regex,
	/// regex for extracting crate names from extern crate
	extern_regex: Regex,
}

impl DocLinker {
	/// Create a new linker.
	pub fn new() -> Self {
		Self {
			use_regex: Regex::new(
				r"use\s+([a-z_][a-z0-9_]*)::",
			)
			.unwrap(),
			extern_regex: Regex::new(
				r"extern\s+crate\s+([a-z_][a-z0-9_]*)",
			)
			.unwrap(),
		}
	}

	/// Build all cross-references for entries in store.
	pub fn build_links(
		&self,
		store: &mut DocStore,
		graph: &SemanticGraph,
		symbols: &[Symbol],
	) {
		let ctx = LinkContext { graph, symbols };
		let entry_ids: Vec<String> = store
			.all_entries()
			.map(|entry| entry.id.clone())
			.collect();

		for entry_id in entry_ids {
			self.link_single_entry(
				store, &ctx, &entry_id,
			);
		}
	}

	/// Extract external crate names from code.
	pub fn extract_crates(
		&self,
		code: &str,
	) -> Vec<String> {
		let mut crates = HashSet::new();

		let use_names = self
			.use_regex
			.captures_iter(code)
			.filter_map(|cap| cap.get(1))
			.map(|mat| mat.as_str())
			.filter(|name| !is_std_module(name));

		for name in use_names {
			crates.insert(name.to_string());
		}

		let extern_names = self
			.extern_regex
			.captures_iter(code)
			.filter_map(|cap| cap.get(1))
			.map(|mat| mat.as_str());

		for name in extern_names {
			crates.insert(name.to_string());
		}

		crates.into_iter().collect()
	}
}

/// Internal linking logic for a single entry.
impl DocLinker {
	/// Link a single entry with its references.
	fn link_single_entry(
		&self,
		store: &mut DocStore,
		ctx: &LinkContext<'_>,
		entry_id: &str,
	) {
		let entry_name = {
			let entry = store.get(entry_id).unwrap();
			entry.name.clone()
		};

		let (ref_locs, depended_by) = collect_references(
			ctx.graph, ctx.symbols, &entry_name,
		);

		let depends_on = find_dependencies(
			ctx.graph, ctx.symbols, &entry_name,
		);

		let external_deps = {
			let entry = store.get(entry_id).unwrap();
			extract_external_deps(&entry.code_snippet)
		};

		let parent = find_parent_name(
			ctx.symbols, &entry_name,
		);
		let children =
			find_children(ctx.symbols, &entry_name);

		apply_links(
			store, entry_id, ref_locs,
			depended_by, depends_on,
			external_deps, parent, children,
		);
	}
}

/// Find parent name for a symbol.
fn find_parent_name(
	symbols: &[Symbol],
	entry_name: &str,
) -> Option<String> {
	symbols
		.iter()
		.find(|sym| sym.name == entry_name)
		.and_then(|sym| sym.parent.clone())
}

/// Apply collected links to a store entry.
#[allow(clippy::too_many_arguments)]
fn apply_links(
	store: &mut DocStore,
	entry_id: &str,
	ref_locs: Vec<ReferenceLocation>,
	depended_by: HashSet<String>,
	depends_on: Vec<String>,
	external_deps: Vec<String>,
	parent: Option<String>,
	children: Vec<String>,
) {
	if let Some(entry) = store.get_mut(entry_id) {
		entry.references = ref_locs;
		entry.links.depended_by =
			depended_by.into_iter().collect();
		entry.links.depends_on = depends_on;
		entry.links.external_deps = external_deps;
		entry.links.parent = parent;
		entry.links.children = children;
	}
}

/// Collect reference locations and dependents.
fn collect_references(
	graph: &SemanticGraph,
	symbols: &[Symbol],
	entry_name: &str,
) -> RefCollection {
	let refs = graph.find_references(entry_name);
	let mut ref_locs = Vec::new();
	let mut depended_by = HashSet::new();

	for sym_ref in refs {
		let ref_loc = build_ref_location(
			sym_ref, symbols, &mut depended_by,
		);
		ref_locs.push(ref_loc);
	}

	(ref_locs, depended_by)
}

/// Build base reference location from a sym ref.
fn build_ref_location(
	sym_ref: &crate::indexer::semantic::SymbolReference,
	symbols: &[Symbol],
	depended_by: &mut HashSet<String>,
) -> ReferenceLocation {
	let base = make_base_ref_loc(sym_ref);
	let containing = find_containing_symbol(
		symbols,
		&sym_ref.location,
	);

	if let Some(ref container) = containing {
		depended_by.insert(container.clone());
		base.with_containing_symbol(
			container.clone(),
		)
	} else {
		base
	}
}

/// Create base ReferenceLocation with context/module.
fn make_base_ref_loc(
	sym_ref: &crate::indexer::semantic::SymbolReference,
) -> ReferenceLocation {
	let ref_kind = convert_reference_context(
		sym_ref.context,
	);
	let context = extract_context_lines(
		&sym_ref.location.file,
		sym_ref.location.line,
	);
	let module_path =
		get_module_path(&sym_ref.location.file);

	ReferenceLocation::new(
		sym_ref.location.file.clone(),
		sym_ref.location.line,
		ref_kind,
	)
	.with_context(context)
	.with_module_path(module_path)
}

impl Default for DocLinker {
	fn default() -> Self {
		Self::new()
	}
}
