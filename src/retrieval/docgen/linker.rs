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
		let entry_ids: Vec<String> = store
			.all_entries()
			.map(|e| e.id.clone())
			.collect();

		for id in entry_ids {
			self.link_single_entry(
				store, graph, symbols, &id,
			);
		}
	}

	/// Extract external crate names from code.
	pub fn extract_crates(
		&self,
		code: &str,
	) -> Vec<String> {
		let mut crates = HashSet::new();

		for cap in self.use_regex.captures_iter(code) {
			if let Some(m) = cap.get(1) {
				let name = m.as_str();
				if !is_std_module(name) {
					crates.insert(name.to_string());
				}
			}
		}

		for cap in self.extern_regex.captures_iter(code) {
			if let Some(m) = cap.get(1) {
				crates.insert(m.as_str().to_string());
			}
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
		graph: &SemanticGraph,
		symbols: &[Symbol],
		id: &str,
	) {
		let entry_name = {
			let entry = store.get(id).unwrap();
			entry.name.clone()
		};

		let (ref_locs, depended_by) =
			collect_references(graph, symbols, &entry_name);

		let depends_on =
			find_dependencies(graph, symbols, &entry_name);

		let external_deps = {
			let entry = store.get(id).unwrap();
			extract_external_deps(&entry.code_snippet)
		};

		let parent = symbols
			.iter()
			.find(|s| s.name == entry_name)
			.and_then(|s| s.parent.clone());

		let children = find_children(symbols, &entry_name);

		if let Some(entry) = store.get_mut(id) {
			entry.references = ref_locs;
			entry.links.depended_by =
				depended_by.into_iter().collect();
			entry.links.depends_on = depends_on;
			entry.links.external_deps = external_deps;
			entry.links.parent = parent;
			entry.links.children = children;
		}
	}
}

/// Collect reference locations and dependents.
fn collect_references(
	graph: &SemanticGraph,
	symbols: &[Symbol],
	entry_name: &str,
) -> (Vec<ReferenceLocation>, HashSet<String>) {
	let refs = graph.find_references(entry_name);
	let mut ref_locs = Vec::new();
	let mut depended_by = HashSet::new();

	for sym_ref in refs {
		let ref_kind = convert_reference_context(
			sym_ref.context,
		);
		let context = extract_context_lines(
			&sym_ref.location.file,
			sym_ref.location.line,
		);
		let module_path =
			get_module_path(&sym_ref.location.file);
		let containing = find_containing_symbol(
			symbols,
			&sym_ref.location,
		);

		let ref_loc = ReferenceLocation::new(
			sym_ref.location.file.clone(),
			sym_ref.location.line,
			ref_kind,
		)
		.with_context(context)
		.with_module_path(module_path);

		let ref_loc =
			if let Some(ref container) = containing {
				depended_by.insert(container.clone());
				ref_loc.with_containing_symbol(
					container.clone(),
				)
			} else {
				ref_loc
			};

		ref_locs.push(ref_loc);
	}

	(ref_locs, depended_by)
}

impl Default for DocLinker {
	fn default() -> Self {
		Self::new()
	}
}

