//! Symbol dependency linking.

use serde::{Deserialize, Serialize};

/// Cross-reference links between symbols.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolLinks {
	/// symbols this one depends on (calls, uses types from)
	pub depends_on: Vec<String>,
	/// symbols that depend on this one
	pub depended_by: Vec<String>,
	/// parent module/struct/impl
	pub parent: Option<String>,
	/// child symbols (for modules, structs, impls)
	pub children: Vec<String>,
	/// related external crates
	pub external_deps: Vec<String>,
}

impl SymbolLinks {
	/// Create empty links.
	pub fn new() -> Self {
		Self::default()
	}

	/// Add a dependency.
	pub fn add_depends_on(&mut self, symbol: String) {
		if !self.depends_on.contains(&symbol) {
			self.depends_on.push(symbol);
		}
	}

	/// Add a dependent.
	pub fn add_depended_by(&mut self, symbol: String) {
		if !self.depended_by.contains(&symbol) {
			self.depended_by.push(symbol);
		}
	}

	/// Add a child symbol.
	pub fn add_child(&mut self, symbol: String) {
		if !self.children.contains(&symbol) {
			self.children.push(symbol);
		}
	}

	/// Add external dependency.
	pub fn add_external_dep(&mut self, crate_name: String) {
		if !self.external_deps.contains(&crate_name) {
			self.external_deps.push(crate_name);
		}
	}
}

