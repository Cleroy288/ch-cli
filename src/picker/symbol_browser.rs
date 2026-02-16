use std::collections::HashSet;
use std::path::PathBuf;

use crate::indexer::symbols::{Symbol, SymbolKind};

/// Browse symbols within a parsed file.
///
/// Supports drilling into container symbols
/// (struct, enum, trait, impl) and filtering by query.
pub struct SymbolBrowser {
    /// All symbols from the file
    symbols: Vec<Symbol>,
    /// Source file path
    file_path: PathBuf,
    /// Current parent (None = top-level)
    parent_stack: Vec<String>,
    /// Symbol names that have documentation
    doc_names: HashSet<String>,
}

impl SymbolBrowser {
    /// Create a new SymbolBrowser
    pub fn new(
        file_path: PathBuf,
        symbols: Vec<Symbol>,
    ) -> Self {
        Self {
            symbols,
            file_path,
            parent_stack: Vec::new(),
            doc_names: HashSet::new(),
        }
    }

    /// Get symbols matching current parent + query.
    ///
    /// Returns top-level symbols if parent is None,
    /// or children of current parent filtered by query.
    pub fn current_items(
        &self,
        query: &str,
    ) -> Vec<&Symbol> {
        let parent = self.current_parent();
        let query_lower = query.to_lowercase();

        self.symbols
            .iter()
            .filter(|sym| sym.parent.as_deref() == parent)
            .filter(|sym| {
                query_lower.is_empty()
                    || sym
                        .name
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .collect()
    }

    /// Drill into a container symbol's children
    pub fn drill_into(&mut self, parent_name: String) {
        self.parent_stack.push(parent_name);
    }

    /// Go up to parent. Returns false if already at top.
    pub fn go_up(&mut self) -> bool {
        self.parent_stack.pop().is_some()
    }

    /// Check if a symbol kind is a drillable container
    pub fn is_container(kind: SymbolKind) -> bool {
        matches!(
            kind,
            SymbolKind::Struct
                | SymbolKind::Enum
                | SymbolKind::Trait
                | SymbolKind::Impl
        )
    }
}

/// Getters for SymbolBrowser.
impl SymbolBrowser {
    /// Get the file path
    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    /// Get the current parent name
    pub fn current_parent(&self) -> Option<&str> {
        self.parent_stack.last().map(|name| name.as_str())
    }

    /// Get the full parent stack for building paths
    pub fn parent_stack(&self) -> &[String] {
        &self.parent_stack
    }

    /// Set the documented symbol names
    pub fn set_doc_names(
        &mut self,
        names: HashSet<String>,
    ) {
        self.doc_names = names;
    }

    /// Check if a symbol has documentation
    pub fn has_doc(&self, name: &str) -> bool {
        self.doc_names.contains(name)
    }
}
