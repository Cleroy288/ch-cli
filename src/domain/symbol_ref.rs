use crate::domain::file_name::FileName;
use crate::domain::file_path::FilePath;
use crate::domain::file_ref::InputSpan;

/// Reference to a symbol within a file.
///
/// Tracks the input span and symbol path for display
/// like `memory.rs(AiResponse::CodeChange)`.
#[derive(Debug, Clone)]
pub struct SymbolSelector {
    /// Start position in input string
    pub start: usize,
    /// End position in input string
    pub end: usize,
    /// Full path to the file
    pub file_path: FilePath,
    /// File display name (e.g., "memory.rs")
    pub file_name: FileName,
    /// Symbol path (e.g., "AiResponse::CodeChange")
    pub symbol_path: String,
}

impl SymbolSelector {
    /// Create a new SymbolSelector
    pub fn new(
        span: InputSpan,
        file_path: FilePath,
        file_name: FileName,
        symbol_path: String,
    ) -> Self {
        Self {
            start: span.start,
            end: span.end,
            file_path,
            file_name,
            symbol_path,
        }
    }

    /// Display text: "file.rs(Symbol::Path)"
    pub fn display_text(&self) -> String {
        format!(
            "{}({})",
            self.file_name.as_string(),
            self.symbol_path,
        )
    }
}

/// Extract the leaf name from a symbol path.
///
/// "Calculator::add" -> "add"
/// "add" -> "add"
pub fn extract_leaf_name(
    symbol_path: &str,
) -> &str {
    symbol_path
        .rsplit("::")
        .next()
        .unwrap_or(symbol_path)
}

/// Build a symbol path by joining parents and leaf
/// with `::` separator.
pub fn build_symbol_path(
    parents: &[String],
    leaf: &str,
) -> String {
    if parents.is_empty() {
        return leaf.to_string();
    }
    let mut path = parents.join("::");
    path.push_str("::");
    path.push_str(leaf);
    path
}
