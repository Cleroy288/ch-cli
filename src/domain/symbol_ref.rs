use crate::domain::file_name::FileName;
use crate::domain::file_path::FilePath;
use crate::domain::file_ref::InputSpan;

#[derive(Debug, Clone)]
pub struct SymbolSelector {
    pub start: usize,
    pub end: usize,
    pub file_path: FilePath,
    pub file_name: FileName,
    pub symbol_path: String,
}

impl SymbolSelector {
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

    pub fn display_text(&self) -> String {
        format!(
            "{}({})",
            self.file_name,
            self.symbol_path,
        )
    }
}

pub fn extract_leaf_name(
    symbol_path: &str,
) -> &str {
    symbol_path
        .rsplit("::")
        .next()
        .unwrap_or(symbol_path)
}

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
