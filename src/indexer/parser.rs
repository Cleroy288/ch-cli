//! Tree-sitter parser wrapper for Rust code.
//!
//! This module provides a high-level interface for parsing Rust source files
//! and extracting symbols and references using Tree-sitter queries.

use std::fs;
use std::path::Path;

use streaming_iterator::StreamingIterator;
use thiserror::Error;
use tree_sitter::{Language, Parser, Query, QueryCursor};

use crate::indexer::queries::{RUST_REFERENCES_QUERY, RUST_SYMBOLS_QUERY};
use crate::indexer::semantic::ReferenceContext;
use crate::indexer::symbols::{CodeLocation, Symbol, SymbolKind, Visibility};

/// A reference to a symbol extracted from the AST
#[derive(Debug, Clone)]
pub struct ExtractedReference {
    /// The name being referenced
    pub name: String,
    /// Location of the reference
    pub location: CodeLocation,
    /// The kind of reference
    pub context: ReferenceContext,
}

/// Errors that can occur during parsing
#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Failed to set parser language: {0}")]
    LanguageError(String),

    #[error("Failed to parse file: {0}")]
    ParseFailed(String),

    #[error("Failed to compile query: {0}")]
    QueryError(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;

/// Rust language parser using Tree-sitter.
///
/// Provides methods to parse Rust source files and extract symbols and references.
pub struct RustParser {
    parser: Parser,
    language: Language,
    symbol_query: Query,
    reference_query: Query,
}

impl RustParser {
    /// Create a new RustParser instance.
    pub fn new() -> Result<Self> {
        let mut parser = Parser::new();
        let language: Language = tree_sitter_rust::LANGUAGE.into();

        parser
            .set_language(&language)
            .map_err(|e| ParseError::LanguageError(e.to_string()))?;

        let symbol_query = Query::new(&language, RUST_SYMBOLS_QUERY)
            .map_err(|e| ParseError::QueryError(format!("{:?}", e)))?;

        let reference_query = Query::new(&language, RUST_REFERENCES_QUERY)
            .map_err(|e| ParseError::QueryError(format!("{:?}", e)))?;

        Ok(Self {
            parser,
            language,
            symbol_query,
            reference_query,
        })
    }

    /// Parse a Rust source file and extract all symbols.
    pub fn parse_file<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<Symbol>> {
        let path = path.as_ref();
        let source_code = fs::read_to_string(path)?;
        self.parse_source(&source_code, path)
    }

    /// Parse Rust source code from a string.
    pub fn parse_source(&mut self, source: &str, file_path: &Path) -> Result<Vec<Symbol>> {
        let tree = self
            .parser
            .parse(source, None)
            .ok_or_else(|| ParseError::ParseFailed(file_path.display().to_string()))?;

        let root_node = tree.root_node();
        let mut symbols = Vec::new();
        let mut cursor = QueryCursor::new();

        let mut matches = cursor.matches(&self.symbol_query, root_node, source.as_bytes());

        // Use StreamingIterator API - advance() + get() pattern
        while let Some(match_) = matches.next() {
            if let Some(symbol) = self.process_match(match_, source, file_path) {
                symbols.push(symbol);
            }
        }

        Ok(symbols)
    }

    /// Parse a Rust source file and extract all references (symbol usages).
    pub fn parse_file_references<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ExtractedReference>> {
        let path = path.as_ref();
        let source_code = fs::read_to_string(path)?;
        self.parse_references(&source_code, path)
    }

    /// Parse Rust source code and extract all references.
    pub fn parse_references(&mut self, source: &str, file_path: &Path) -> Result<Vec<ExtractedReference>> {
        let tree = self
            .parser
            .parse(source, None)
            .ok_or_else(|| ParseError::ParseFailed(file_path.display().to_string()))?;

        let root_node = tree.root_node();
        let mut references = Vec::new();
        let mut cursor = QueryCursor::new();

        let mut matches = cursor.matches(&self.reference_query, root_node, source.as_bytes());

        while let Some(match_) = matches.next() {
            if let Some(reference) = self.process_reference_match(match_, source, file_path) {
                references.push(reference);
            }
        }

        // Deduplicate references (same name and location)
        references.sort_by(|a, b| {
            a.location.line.cmp(&b.location.line)
                .then(a.location.column.cmp(&b.location.column))
                .then(a.name.cmp(&b.name))
        });
        references.dedup_by(|a, b| {
            a.name == b.name
                && a.location.line == b.location.line
                && a.location.column == b.location.column
        });

        Ok(references)
    }

    /// Process a reference match and extract reference info.
    fn process_reference_match(
        &self,
        match_: &tree_sitter::QueryMatch,
        source: &str,
        file_path: &Path,
    ) -> Option<ExtractedReference> {
        // Find the most specific capture
        let capture = match_.captures.first()?;
        let capture_name = self.reference_query.capture_names()[capture.index as usize];
        let node = capture.node;
        let text = &source[node.byte_range()];

        // Determine reference context from capture name
        let context = match capture_name {
            "call.name" => ReferenceContext::Call,
            "method_call.name" => ReferenceContext::Call,
            "type_ref.name" => ReferenceContext::Type,
            "use.name" | "use.type_name" | "use.simple" | "use.list_item" | "use.list_type_item" => {
                ReferenceContext::Import
            }
            "field_access.name" => ReferenceContext::FieldAccess,
            "ident.name" => ReferenceContext::Identifier,
            "scoped.name" => ReferenceContext::Identifier,
            _ => return None, // Skip expression/declaration captures, we want names
        };

        // Skip keywords and common identifiers that aren't real references
        if is_rust_keyword(text) || text == "self" || text == "Self" {
            return None;
        }

        let location = CodeLocation::new(
            file_path.to_path_buf(),
            node.start_position().row + 1,
            node.start_position().column + 1,
            node.start_byte(),
            node.end_byte() - node.start_byte(),
        );

        Some(ExtractedReference {
            name: text.to_string(),
            location,
            context,
        })
    }

    /// Process a query match and extract a symbol.
    fn process_match(
        &self,
        match_: &tree_sitter::QueryMatch,
        source: &str,
        file_path: &Path,
    ) -> Option<Symbol> {
        // Determine the symbol kind based on capture names
        let mut name: Option<&str> = None;
        let mut kind: Option<SymbolKind> = None;
        let mut visibility = Visibility::Private;
        let mut def_node: Option<tree_sitter::Node> = None;
        let mut parent_type: Option<&str> = None;

        for capture in match_.captures {
            let capture_name = self.symbol_query.capture_names()[capture.index as usize];
            let node = capture.node;
            let text = &source[node.byte_range()];

            match capture_name {
                // Function captures
                "function.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Function);
                }
                "function.def" => def_node = Some(node),
                "function.visibility" => visibility = parse_visibility(text),

                // Struct captures
                "struct.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Struct);
                }
                "struct.def" => def_node = Some(node),
                "struct.visibility" => visibility = parse_visibility(text),

                // Enum captures
                "enum.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Enum);
                }
                "enum.def" => def_node = Some(node),
                "enum.visibility" => visibility = parse_visibility(text),

                // Trait captures
                "trait.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Trait);
                }
                "trait.def" => def_node = Some(node),
                "trait.visibility" => visibility = parse_visibility(text),

                // Impl captures
                "impl.type" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Impl);
                }
                "impl.def" => def_node = Some(node),

                // Const captures
                "const.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Constant);
                }
                "const.def" => def_node = Some(node),
                "const.visibility" => visibility = parse_visibility(text),

                // Static captures
                "static.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Static);
                }
                "static.def" => def_node = Some(node),
                "static.visibility" => visibility = parse_visibility(text),

                // Type alias captures
                "type.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::TypeAlias);
                }
                "type.def" => def_node = Some(node),
                "type.visibility" => visibility = parse_visibility(text),

                // Module captures
                "mod.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Module);
                }
                "mod.def" => def_node = Some(node),
                "mod.visibility" => visibility = parse_visibility(text),

                // Macro captures
                "macro.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Macro);
                }
                "macro.def" => def_node = Some(node),

                // Method captures
                "method.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Method);
                }
                "method.def" => def_node = Some(node),
                "method.visibility" => visibility = parse_visibility(text),
                "method.parent_type" => parent_type = Some(text),

                // Enum variant captures
                "variant.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::EnumVariant);
                }
                "variant.def" => def_node = Some(node),
                "variant.parent" => parent_type = Some(text),

                // Field captures
                "field.name" => {
                    name = Some(text);
                    kind = Some(SymbolKind::Field);
                }
                "field.def" => def_node = Some(node),
                "field.visibility" => visibility = parse_visibility(text),
                "field.parent" => parent_type = Some(text),

                _ => {}
            }
        }

        // Build the symbol if we have the required information
        let name = name?;
        let kind = kind?;
        let node = def_node?;

        let location = CodeLocation::new(
            file_path.to_path_buf(),
            node.start_position().row + 1, // Convert to 1-indexed
            node.start_position().column + 1,
            node.start_byte(),
            node.end_byte() - node.start_byte(),
        );

        let mut symbol = Symbol::new(name.to_string(), kind, location).with_visibility(visibility);

        if let Some(parent) = parent_type {
            symbol = symbol.with_parent(parent.to_string());
        }

        Some(symbol)
    }

    /// Get the Tree-sitter language
    pub fn language(&self) -> &Language {
        &self.language
    }

    /// Extract module-level doc comments (//!) from source code.
    /// Returns the concatenated module documentation as a single string.
    pub fn extract_module_docs(source: &str) -> Option<String> {
        let mut doc_lines = Vec::new();
        let mut in_module_docs = true;

        for line in source.lines() {
            let trimmed = line.trim();

            // Module doc comment: //!
            if trimmed.starts_with("//!") {
                let doc = trimmed.strip_prefix("//!").unwrap_or("").trim();
                doc_lines.push(doc.to_string());
            }
            // Empty lines are OK at the start
            else if trimmed.is_empty() && doc_lines.is_empty() {
                continue;
            }
            // Non-comment line ends module docs section
            else if !trimmed.starts_with("//") && !trimmed.is_empty() {
                in_module_docs = false;
                break;
            }
            // Regular comment - might be part of module docs
            else if trimmed.starts_with("//") && in_module_docs && !doc_lines.is_empty() {
                // After module docs start, regular comments don't count
                break;
            }
        }

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }

    /// Extract doc comment (///) above a symbol at a given line.
    /// Returns the concatenated documentation.
    pub fn extract_item_doc(source: &str, symbol_line: usize) -> Option<String> {
        let lines: Vec<&str> = source.lines().collect();

        if symbol_line == 0 || symbol_line > lines.len() {
            return None;
        }

        let mut doc_lines = Vec::new();
        let mut current_line = symbol_line - 2; // 0-indexed, start one line above

        // Walk backwards collecting doc comments
        while current_line < lines.len() {
            let line = lines[current_line].trim();

            if line.starts_with("///") {
                let doc = line.strip_prefix("///").unwrap_or("").trim();
                doc_lines.insert(0, doc.to_string());
            } else if line.starts_with("#[") {
                // Attribute - skip but continue
            } else if line.is_empty() {
                // Empty line - might have more docs above
                if !doc_lines.is_empty() {
                    break;
                }
            } else {
                // Non-doc line - stop
                break;
            }

            if current_line == 0 {
                break;
            }
            current_line -= 1;
        }

        if doc_lines.is_empty() {
            None
        } else {
            Some(doc_lines.join("\n"))
        }
    }
}

/// Parse a visibility modifier string into a Visibility enum
fn parse_visibility(text: &str) -> Visibility {
    match text.trim() {
        "pub" => Visibility::Public,
        s if s.starts_with("pub(crate)") => Visibility::PublicCrate,
        s if s.starts_with("pub(super)") => Visibility::PublicSuper,
        _ => Visibility::Private,
    }
}

/// Check if a string is a Rust keyword (to filter out from references)
fn is_rust_keyword(s: &str) -> bool {
    matches!(
        s,
        "as" | "break" | "const" | "continue" | "crate" | "else" | "enum" | "extern"
            | "false" | "fn" | "for" | "if" | "impl" | "in" | "let" | "loop" | "match"
            | "mod" | "move" | "mut" | "pub" | "ref" | "return" | "self" | "Self"
            | "static" | "struct" | "super" | "trait" | "true" | "type" | "unsafe"
            | "use" | "where" | "while" | "async" | "await" | "dyn" | "abstract"
            | "become" | "box" | "do" | "final" | "macro" | "override" | "priv"
            | "typeof" | "unsized" | "virtual" | "yield" | "try"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_function() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
fn hello() {
    println!("Hello, world!");
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        assert!(!symbols.is_empty());
        let func = symbols.iter().find(|s| s.name == "hello").unwrap();
        assert_eq!(func.kind, SymbolKind::Function);
        assert_eq!(func.visibility, Visibility::Private);
    }

    #[test]
    fn test_parse_pub_struct() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub struct MyStruct {
    field: i32,
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let strukt = symbols.iter().find(|s| s.name == "MyStruct").unwrap();
        assert_eq!(strukt.kind, SymbolKind::Struct);
        assert_eq!(strukt.visibility, Visibility::Public);
    }

    #[test]
    fn test_parse_enum() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub enum Color {
    Red,
    Green,
    Blue,
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let enm = symbols.iter().find(|s| s.name == "Color").unwrap();
        assert_eq!(enm.kind, SymbolKind::Enum);
        assert_eq!(enm.visibility, Visibility::Public);
    }

    #[test]
    fn test_parse_trait() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub trait Drawable {
    fn draw(&self);
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let trt = symbols.iter().find(|s| s.name == "Drawable").unwrap();
        assert_eq!(trt.kind, SymbolKind::Trait);
    }

    #[test]
    fn test_parse_impl_block() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
struct Point { x: i32, y: i32 }

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        // Should find the struct
        assert!(symbols.iter().any(|s| s.name == "Point" && s.kind == SymbolKind::Struct));
        // Should find the impl
        assert!(symbols.iter().any(|s| s.name == "Point" && s.kind == SymbolKind::Impl));
    }

    #[test]
    fn test_parse_const_and_static() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub const MAX_SIZE: usize = 100;
static mut COUNTER: i32 = 0;
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let cnst = symbols.iter().find(|s| s.name == "MAX_SIZE").unwrap();
        assert_eq!(cnst.kind, SymbolKind::Constant);
        assert_eq!(cnst.visibility, Visibility::Public);

        let stat = symbols.iter().find(|s| s.name == "COUNTER").unwrap();
        assert_eq!(stat.kind, SymbolKind::Static);
    }

    #[test]
    fn test_parse_module() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub mod utils {
    pub fn helper() {}
}
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let module = symbols.iter().find(|s| s.name == "utils").unwrap();
        assert_eq!(module.kind, SymbolKind::Module);
        assert_eq!(module.visibility, Visibility::Public);
    }

    #[test]
    fn test_parse_type_alias() {
        let mut parser = RustParser::new().unwrap();
        let source = r#"
pub type Result<T> = std::result::Result<T, Error>;
"#;
        let symbols = parser
            .parse_source(source, Path::new("test.rs"))
            .unwrap();

        let alias = symbols.iter().find(|s| s.name == "Result").unwrap();
        assert_eq!(alias.kind, SymbolKind::TypeAlias);
    }

    #[test]
    fn test_parse_real_file() {
        let mut parser = RustParser::new().unwrap();
        // Parse the actual lib.rs file
        let result = parser.parse_file("src/lib.rs");
        assert!(result.is_ok());
        let symbols = result.unwrap();
        // Should find at least the App re-export
        assert!(!symbols.is_empty());
    }
}

