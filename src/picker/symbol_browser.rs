use std::collections::HashSet;
use std::path::PathBuf;

use crate::indexer::symbols::{Symbol, SymbolKind};

/// Browse symbols within a parsed file.
/// Supports drilling into containers (struct,
/// enum, trait, impl) and filtering by query.
pub struct SymbolBrowser {
    symbols: Vec<Symbol>,
    file_path: PathBuf,
    parent_stack: Vec<String>,
    doc_names: HashSet<String>,
}

impl SymbolBrowser {
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

    pub fn file_path(&self) -> &PathBuf {
        &self.file_path
    }

    pub fn current_parent(&self) -> Option<&str> {
        self.parent_stack.last().map(String::as_str)
    }

    pub fn parent_stack(&self) -> &[String] {
        &self.parent_stack
    }

    pub fn set_doc_names(
        &mut self,
        names: HashSet<String>,
    ) {
        self.doc_names = names;
    }

    pub fn has_doc(&self, name: &str) -> bool {
        self.doc_names.contains(name)
    }

    /// Symbols matching current parent + query
    pub fn current_items(
        &self,
        query: &str,
    ) -> Vec<&Symbol> {
        let parent = self.current_parent();
        let q = query.to_lowercase();
        self.symbols
            .iter()
            .filter(|s| s.parent.as_deref() == parent)
            .filter(|s| {
                q.is_empty()
                    || s.name
                        .to_lowercase()
                        .contains(&q)
            })
            .collect()
    }

    pub fn drill_into(&mut self, name: String) {
        self.parent_stack.push(name);
    }

    /// Go up to parent. Returns false at top.
    pub fn go_up(&mut self) -> bool {
        self.parent_stack.pop().is_some()
    }

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
