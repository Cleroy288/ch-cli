//! Symbol types for the semantic indexer.
//!
//! This module defines the core data structures used to represent
//! code symbols extracted from source files.

use std::fmt;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::retrieval::daemon::protocol::QueryIntent;

/// Represents a location in source code.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeLocation {
    /// Path to the source file
    pub file: PathBuf,
    /// Line number (1-indexed for display)
    pub line: usize,
    /// Column number (1-indexed for display)
    pub column: usize,
    /// Byte offset from start of file
    pub byte_offset: usize,
    /// Byte length of the symbol
    pub byte_length: usize,
}

impl CodeLocation {
    /// Create a new CodeLocation
    pub fn new(file: PathBuf, line: usize, column: usize, byte_offset: usize, byte_length: usize) -> Self {
        Self {
            file,
            line,
            column,
            byte_offset,
            byte_length,
        }
    }
}

impl fmt::Display for CodeLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file.display(), self.line, self.column)
    }
}

/// The kind of symbol (function, struct, trait, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SymbolKind {
    /// Function definition
    Function,
    /// Method (function inside impl block)
    Method,
    /// Struct definition
    Struct,
    /// Enum definition
    Enum,
    /// Trait definition
    Trait,
    /// Impl block
    Impl,
    /// Constant definition
    Constant,
    /// Static variable
    Static,
    /// Type alias
    TypeAlias,
    /// Module definition
    Module,
    /// Macro definition
    Macro,
    /// Enum variant
    EnumVariant,
    /// Struct field
    Field,
    /// Documentation chunk (markdown section)
    DocumentChunk,
}

impl SymbolKind {
    /// Get boost factor for ranking
    /// Functions/methods ranked higher than fields/constants
    pub fn boost_factor(&self) -> f32 {
        match self {
            // High priority: actual code structures
            SymbolKind::Function => 1.4,
            SymbolKind::Method => 1.4,
            SymbolKind::Struct => 1.3,
            SymbolKind::Enum => 1.3,
            SymbolKind::Trait => 1.3,
            SymbolKind::Impl => 1.2,
            SymbolKind::Module => 1.1,

            // Medium priority
            SymbolKind::Constant => 0.9,
            SymbolKind::Static => 0.9,
            SymbolKind::TypeAlias => 0.9,
            SymbolKind::Macro => 1.0,
            SymbolKind::EnumVariant => 0.8,

            // Lower priority
            SymbolKind::Field => 0.7,

            // Documentation symbols
            SymbolKind::DocumentChunk => 0.6,
        }
    }

    /// Get boost factor adjusted for query intent
    /// Boosts functions for understanding queries, deprioritizes fields
    pub fn boost_factor_for_intent(&self, intent: &QueryIntent) -> f32 {
        let base_boost = self.boost_factor(); // base symbol kind boost

        match intent {
            QueryIntent::Understand => {
                // For "how does X work" queries, prioritize behavior over data
                self.apply_understand_boost(base_boost)
            }
            QueryIntent::FindDefinition => {
                // For definition queries, prioritize type definitions
                self.apply_find_definition_boost(base_boost)
            }
            QueryIntent::Debug => {
                // For debugging queries, prioritize functions with error handling
                self.apply_debug_boost(base_boost)
            }
            _ => base_boost, // other intents use base boost
        }
    }

    /// Apply boost multiplier for Understand intent
    /// Boosts functions/methods (1.5x), deprioritizes fields/docs (0.3x)
    fn apply_understand_boost(&self, base_boost: f32) -> f32 {
        match self {
            // Functions/methods explain behavior - heavily boost them
            SymbolKind::Function | SymbolKind::Method => base_boost * 1.5,
            // Structs/impls can show structure - slightly boost
            SymbolKind::Struct | SymbolKind::Impl => base_boost * 1.2,
            // Fields are data, not behavior - deprioritize
            SymbolKind::Field => base_boost * 0.3,
            // Documentation chunks are docs, not code - heavily deprioritize
            SymbolKind::DocumentChunk => base_boost * 0.2,
            _ => base_boost,
        }
    }

    /// Apply boost multiplier for Debug intent
    /// Boosts functions/methods for debugging, deprioritizes fields
    fn apply_debug_boost(&self, base_boost: f32) -> f32 {
        match self {
            // Functions/methods likely contain error handling - boost
            SymbolKind::Function | SymbolKind::Method => base_boost * 1.3,
            // Structs that might be error types - slight boost
            SymbolKind::Struct | SymbolKind::Enum => base_boost * 1.1,
            // Fields less relevant for debugging - deprioritize
            SymbolKind::Field => base_boost * 0.5,
            _ => base_boost,
        }
    }

    /// Apply boost multiplier for FindDefinition intent
    /// Boosts struct/enum/trait definitions (1.1x)
    fn apply_find_definition_boost(&self, base_boost: f32) -> f32 {
        match self {
            // Type definitions are what user is looking for
            SymbolKind::Struct | SymbolKind::Enum | SymbolKind::Trait => base_boost * 1.1,
            _ => base_boost,
        }
    }
}

/// Document type for scoring purposes
/// Used to boost source code results over notes/benchmarks
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentType {
    /// Source code (.rs, .py, .js, etc.)
    SourceCode,
    /// Official documentation (doc/*.md)
    Documentation,
    /// Implementation notes (notes/*.md)
    Notes,
    /// Benchmark/test notes (notes/benchmarks/*.md)
    Benchmark,
    /// Test files (*_test.rs, tests/*.rs)
    Test,
}

/// Check if a path string represents a test file
/// Detects: /tests/ directory, _test.rs suffix, test_ prefix, _tests.rs suffix
fn is_test_file(path_str: &str) -> bool {
    // tests directory pattern
    if path_str.contains("/tests/") {
        return true;
    }

    // _test.rs suffix pattern (e.g., parser_test.rs)
    if path_str.ends_with("_test.rs") {
        return true;
    }

    // _tests.rs suffix pattern (e.g., parser_tests.rs)
    if path_str.ends_with("_tests.rs") {
        return true;
    }

    // test_ prefix pattern (e.g., /src/test_parser.rs)
    // Check for /test_ to avoid matching paths like "latest_feature.rs"
    if path_str.contains("/test_") {
        return true;
    }

    false
}

impl DocumentType {
    /// Classify a file path into a document type
    pub fn from_path(path: &std::path::Path) -> Self {
        let path_str = path.to_string_lossy(); // path as string for pattern matching
        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or(""); // file extension

        // Check benchmarks first (most specific)
        if path_str.contains("/benchmarks/") {
            return Self::Benchmark;
        }

        // Check notes
        if path_str.contains("/notes/") {
            return Self::Notes;
        }

        // Check documentation
        if path_str.contains("/doc/") && extension == "md" {
            return Self::Documentation;
        }

        // Check test files - multiple patterns
        // Matches: /tests/ directory, _test.rs suffix, test_ prefix, _tests.rs suffix
        if is_test_file(&path_str) {
            return Self::Test;
        }

        // Check source code extensions
        match extension {
            "rs" | "py" | "js" | "ts" | "go" | "java" | "c" | "cpp" | "h" => Self::SourceCode,
            "md" | "txt" => Self::Documentation,
            _ => Self::SourceCode, // default to source
        }
    }

    /// Get the boost factor for this document type
    /// Higher values = higher priority in search results
    pub fn boost_factor(&self) -> f32 {
        match self {
            Self::SourceCode => 1.5,      // Highest priority
            Self::Documentation => 1.0,   // Normal priority
            Self::Test => 0.8,            // Lower than docs, tests are examples not primary code
            Self::Notes => 0.7,           // Lower priority
            Self::Benchmark => 0.3,       // Much lower priority
        }
    }

    /// Get boost factor adjusted for query context.
    /// Lowers documentation boost for implementation-focused queries.
    pub fn boost_factor_for_query(&self, query: &str) -> f32 {
        let base_boost = self.boost_factor(); // base boost for this document type
        let lower_query = query.to_lowercase(); // lowercase query for matching

        // Keywords indicating user wants code, not docs
        let impl_keywords = [
            "implementation", "algorithm", "logic", "code", "function", "method",
            "work", "works", "working", "implement", "source", "actual",
        ];
        let wants_code = impl_keywords.iter().any(|k| lower_query.contains(k)); // check if query wants code

        // If query wants code, reduce boost for non-code types
        if wants_code {
            match self {
                Self::Documentation | Self::Notes | Self::Benchmark => base_boost * 0.5,
                Self::SourceCode | Self::Test => base_boost, // unchanged for code
            }
        } else {
            base_boost // no adjustment
        }
    }

    /// Get boost factor adjusted for query intent.
    /// For Understand intent, heavily reduce doc boost since users want code behavior.
    pub fn boost_factor_for_intent(&self, intent: &QueryIntent) -> f32 {
        let base_boost = self.boost_factor(); // base boost for this document type

        match intent {
            QueryIntent::Understand => {
                // users asking "how does X work" want code, not docs
                match self {
                    Self::SourceCode => base_boost * 2.0, // heavily boost source code
                    Self::Documentation | Self::Notes => base_boost * 0.2, // heavily reduce docs
                    Self::Benchmark => base_boost * 0.1, // almost eliminate benchmarks
                    Self::Test => base_boost * 0.8, // slightly reduce tests
                }
            }
            _ => base_boost, // no adjustment for other intents
        }
    }
}

/// High-level content classification for triple pipeline indexing.
/// Used to route symbols to separate Code, Doc, or Notes indexes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ContentType {
    /// Code files (SourceCode + Test)
    Code,
    /// Documentation files (doc/*.md)
    Doc,
    /// Notes and benchmarks (notes/*.md, notes/benchmarks/*.md)
    Notes,
}

impl ContentType {
    /// Classify from DocumentType
    pub fn from_document_type(doc_type: &DocumentType) -> Self {
        match doc_type {
            DocumentType::SourceCode | DocumentType::Test => Self::Code,
            DocumentType::Documentation => Self::Doc,
            DocumentType::Notes | DocumentType::Benchmark => Self::Notes,
        }
    }

    /// Classify from file path
    pub fn from_path(path: &std::path::Path) -> Self {
        Self::from_document_type(&DocumentType::from_path(path))
    }

    /// Get all variants for iteration
    pub fn all() -> [Self; 3] {
        [Self::Code, Self::Doc, Self::Notes]
    }

    /// Get display name for this content type
    pub fn name(&self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Doc => "docs",
            Self::Notes => "notes",
        }
    }
}

impl fmt::Display for ContentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl fmt::Display for SymbolKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            SymbolKind::Function => "fn",
            SymbolKind::Method => "method",
            SymbolKind::Struct => "struct",
            SymbolKind::Enum => "enum",
            SymbolKind::Trait => "trait",
            SymbolKind::Impl => "impl",
            SymbolKind::Constant => "const",
            SymbolKind::Static => "static",
            SymbolKind::TypeAlias => "type",
            SymbolKind::Module => "mod",
            SymbolKind::Macro => "macro",
            SymbolKind::EnumVariant => "variant",
            SymbolKind::Field => "field",
            SymbolKind::DocumentChunk => "doc",
        };
        write!(f, "{}", s)
    }
}

/// Visibility of a symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// Public (pub)
    Public,
    /// Public within crate (pub(crate))
    PublicCrate,
    /// Public within super module (pub(super))
    PublicSuper,
    /// Private (default)
    #[default]
    Private,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Visibility::Public => "pub",
            Visibility::PublicCrate => "pub(crate)",
            Visibility::PublicSuper => "pub(super)",
            Visibility::Private => "",
        };
        write!(f, "{}", s)
    }
}

/// A code symbol extracted from source code.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Name of the symbol
    pub name: String,
    /// Kind of symbol
    pub kind: SymbolKind,
    /// Location in source code
    pub location: CodeLocation,
    /// Visibility modifier
    pub visibility: Visibility,
    /// Function/method signature (if applicable)
    pub signature: Option<String>,
    /// Documentation comment (if any)
    pub doc_comment: Option<String>,
    /// Fully qualified name (e.g., "module::struct::method")
    pub fqn: Option<String>,
    /// Parent symbol name (for methods, fields, variants)
    pub parent: Option<String>,
    /// Full content for documentation chunks
    pub content: Option<String>,
}

impl Symbol {
    /// Create a new Symbol with required fields
    pub fn new(name: String, kind: SymbolKind, location: CodeLocation) -> Self {
        Self {
            name,
            kind,
            location,
            visibility: Visibility::default(),
            signature: None,
            doc_comment: None,
            fqn: None,
            parent: None,
            content: None,
        }
    }

    /// Builder method to set content
    pub fn with_content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }

    /// Builder method to set visibility
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Builder method to set signature
    pub fn with_signature(mut self, signature: String) -> Self {
        self.signature = Some(signature);
        self
    }

    /// Builder method to set doc comment
    pub fn with_doc_comment(mut self, doc: String) -> Self {
        self.doc_comment = Some(doc);
        self
    }

    /// Builder method to set parent
    pub fn with_parent(mut self, parent: String) -> Self {
        self.parent = Some(parent);
        self
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let vis = if self.visibility == Visibility::Private {
            String::new()
        } else {
            format!("{} ", self.visibility)
        };
        write!(f, "{}{} {} @ {}", vis, self.kind, self.name, self.location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    /// Test DocumentType::from_path with benchmark paths
    #[test]
    fn test_document_type_from_path_benchmark() {
        let path = Path::new("/project/notes/benchmarks/perf.md");
        assert_eq!(DocumentType::from_path(path), DocumentType::Benchmark);

        let path = Path::new("/project/benchmarks/test.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Benchmark);
    }

    /// Test DocumentType::from_path with notes paths
    #[test]
    fn test_document_type_from_path_notes() {
        let path = Path::new("/project/notes/implementation.md");
        assert_eq!(DocumentType::from_path(path), DocumentType::Notes);

        let path = Path::new("/project/notes/todo.txt");
        assert_eq!(DocumentType::from_path(path), DocumentType::Notes);
    }

    /// Test DocumentType::from_path with documentation paths
    #[test]
    fn test_document_type_from_path_documentation() {
        let path = Path::new("/project/doc/api.md");
        assert_eq!(DocumentType::from_path(path), DocumentType::Documentation);

        // Non-md file in doc folder should be source code
        let path = Path::new("/project/doc/example.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::SourceCode);
    }

    /// Test DocumentType::from_path with test paths - /tests/ directory
    #[test]
    fn test_document_type_from_path_test_directory() {
        let path = Path::new("/project/tests/unit_tests.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);

        let path = Path::new("/project/tests/integration/api.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);
    }

    /// Test DocumentType::from_path with _test.rs suffix
    #[test]
    fn test_document_type_from_path_test_suffix() {
        let path = Path::new("/project/src/parser_test.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);

        let path = Path::new("/project/src/module/handler_test.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);
    }

    /// Test DocumentType::from_path with _tests.rs suffix
    #[test]
    fn test_document_type_from_path_tests_suffix() {
        let path = Path::new("/project/src/parser_tests.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);

        let path = Path::new("/project/src/module/handler_tests.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);
    }

    /// Test DocumentType::from_path with test_ prefix
    #[test]
    fn test_document_type_from_path_test_prefix() {
        let path = Path::new("/project/src/test_parser.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);

        let path = Path::new("/project/src/test_utils.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::Test);
    }

    /// Test DocumentType::from_path with source code paths
    #[test]
    fn test_document_type_from_path_source_code() {
        let path = Path::new("/project/src/main.rs");
        assert_eq!(DocumentType::from_path(path), DocumentType::SourceCode);

        let path = Path::new("/project/src/lib.py");
        assert_eq!(DocumentType::from_path(path), DocumentType::SourceCode);

        let path = Path::new("/project/app.js");
        assert_eq!(DocumentType::from_path(path), DocumentType::SourceCode);

        let path = Path::new("/project/main.go");
        assert_eq!(DocumentType::from_path(path), DocumentType::SourceCode);
    }

    /// Test DocumentType::boost_factor returns correct values
    #[test]
    fn test_document_type_boost_factor() {
        assert_eq!(DocumentType::SourceCode.boost_factor(), 1.5);
        assert_eq!(DocumentType::Documentation.boost_factor(), 1.0);
        assert_eq!(DocumentType::Test.boost_factor(), 0.8);  // lowered from 0.9
        assert_eq!(DocumentType::Notes.boost_factor(), 0.7);
        assert_eq!(DocumentType::Benchmark.boost_factor(), 0.3);
    }

    /// Test DocumentType::boost_factor ordering (source > doc > test > notes > benchmark)
    #[test]
    fn test_document_type_boost_factor_ordering() {
        assert!(DocumentType::SourceCode.boost_factor() > DocumentType::Documentation.boost_factor());
        assert!(DocumentType::Documentation.boost_factor() > DocumentType::Test.boost_factor());
        assert!(DocumentType::Test.boost_factor() > DocumentType::Notes.boost_factor());
        assert!(DocumentType::Notes.boost_factor() > DocumentType::Benchmark.boost_factor());
    }

    /// Test SymbolKind::boost_factor for high priority symbols
    #[test]
    fn test_symbol_kind_boost_factor_high_priority() {
        assert_eq!(SymbolKind::Function.boost_factor(), 1.4);
        assert_eq!(SymbolKind::Method.boost_factor(), 1.4);
        assert_eq!(SymbolKind::Struct.boost_factor(), 1.3);
        assert_eq!(SymbolKind::Enum.boost_factor(), 1.3);
        assert_eq!(SymbolKind::Trait.boost_factor(), 1.3);
        assert_eq!(SymbolKind::Impl.boost_factor(), 1.2);
        assert_eq!(SymbolKind::Module.boost_factor(), 1.1);
    }

    /// Test SymbolKind::boost_factor for medium priority symbols
    #[test]
    fn test_symbol_kind_boost_factor_medium_priority() {
        assert_eq!(SymbolKind::Macro.boost_factor(), 1.0);
        assert_eq!(SymbolKind::Constant.boost_factor(), 0.9);
        assert_eq!(SymbolKind::Static.boost_factor(), 0.9);
        assert_eq!(SymbolKind::TypeAlias.boost_factor(), 0.9);
        assert_eq!(SymbolKind::EnumVariant.boost_factor(), 0.8);
    }

    /// Test SymbolKind::boost_factor for low priority symbols
    #[test]
    fn test_symbol_kind_boost_factor_low_priority() {
        assert_eq!(SymbolKind::Field.boost_factor(), 0.7);
        assert_eq!(SymbolKind::DocumentChunk.boost_factor(), 0.6);
    }

    /// Test SymbolKind::boost_factor ordering (function > field > doc_chunk)
    #[test]
    fn test_symbol_kind_boost_factor_ordering() {
        assert!(SymbolKind::Function.boost_factor() > SymbolKind::Struct.boost_factor());
        assert!(SymbolKind::Struct.boost_factor() > SymbolKind::Module.boost_factor());
        assert!(SymbolKind::Module.boost_factor() > SymbolKind::Macro.boost_factor());
        assert!(SymbolKind::Macro.boost_factor() > SymbolKind::Constant.boost_factor());
        assert!(SymbolKind::Constant.boost_factor() > SymbolKind::Field.boost_factor());
        assert!(SymbolKind::Field.boost_factor() > SymbolKind::DocumentChunk.boost_factor());
    }

    /// Test is_test_file helper detects all test patterns
    #[test]
    fn test_is_test_file_all_patterns() {
        // /tests/ directory pattern
        assert!(super::is_test_file("/project/tests/unit.rs"));
        assert!(super::is_test_file("/project/tests/integration/api.rs"));

        // _test.rs suffix pattern
        assert!(super::is_test_file("/project/src/parser_test.rs"));
        assert!(super::is_test_file("/src/handler_test.rs"));

        // _tests.rs suffix pattern
        assert!(super::is_test_file("/project/src/parser_tests.rs"));
        assert!(super::is_test_file("/src/handler_tests.rs"));

        // test_ prefix pattern
        assert!(super::is_test_file("/project/src/test_parser.rs"));
        assert!(super::is_test_file("/src/test_utils.rs"));
    }

    /// Test is_test_file helper rejects non-test files
    #[test]
    fn test_is_test_file_non_test_files() {
        // Regular source files should not match
        assert!(!super::is_test_file("/project/src/main.rs"));
        assert!(!super::is_test_file("/project/src/parser.rs"));
        assert!(!super::is_test_file("/project/src/latest_feature.rs"));
        assert!(!super::is_test_file("/project/src/contest.rs"));
    }

    /// Test boost_factor_for_query reduces doc boost for "implementation" query
    #[test]
    fn test_boost_factor_for_query_implementation() {
        let query = "show me the implementation"; // implementation keyword present

        // Documentation types should be reduced by 0.5x
        let doc_boost = DocumentType::Documentation.boost_factor_for_query(query);
        let notes_boost = DocumentType::Notes.boost_factor_for_query(query);
        let benchmark_boost = DocumentType::Benchmark.boost_factor_for_query(query);

        assert_eq!(doc_boost, 1.0 * 0.5);       // 0.5
        assert_eq!(notes_boost, 0.7 * 0.5);     // 0.35
        assert_eq!(benchmark_boost, 0.3 * 0.5); // 0.15

        // Code types should remain unchanged
        let source_boost = DocumentType::SourceCode.boost_factor_for_query(query);
        let test_boost = DocumentType::Test.boost_factor_for_query(query);

        assert_eq!(source_boost, 1.5); // unchanged
        assert_eq!(test_boost, 0.8);   // unchanged
    }

    /// Test boost_factor_for_query unchanged for "where is X" query
    #[test]
    fn test_boost_factor_for_query_where_is() {
        let query = "where is the AuthService defined"; // no implementation keywords

        // All types should return base boost
        assert_eq!(DocumentType::SourceCode.boost_factor_for_query(query), 1.5);
        assert_eq!(DocumentType::Documentation.boost_factor_for_query(query), 1.0);
        assert_eq!(DocumentType::Test.boost_factor_for_query(query), 0.8);
        assert_eq!(DocumentType::Notes.boost_factor_for_query(query), 0.7);
        assert_eq!(DocumentType::Benchmark.boost_factor_for_query(query), 0.3);
    }

    /// Test boost_factor_for_query with various implementation keywords
    #[test]
    fn test_boost_factor_for_query_all_keywords() {
        let keywords = ["implementation", "algorithm", "logic", "code", "function", "method"];

        for keyword in keywords {
            let query = format!("show me the {} details", keyword); // query with keyword
            let doc_boost = DocumentType::Documentation.boost_factor_for_query(&query);

            // Documentation should be reduced for all implementation keywords
            assert_eq!(
                doc_boost, 0.5,
                "Documentation boost should be 0.5 for keyword '{}'",
                keyword
            );
        }
    }

    /// Test boost_factor_for_query is case insensitive
    #[test]
    fn test_boost_factor_for_query_case_insensitive() {
        let queries = ["IMPLEMENTATION", "Implementation", "iMpLeMeNtAtIoN"];

        for query in queries {
            let doc_boost = DocumentType::Documentation.boost_factor_for_query(query);
            assert_eq!(doc_boost, 0.5, "Should be case insensitive for '{}'", query);
        }
    }

    /// Test boost_factor_for_intent boosts functions for Understand intent
    #[test]
    fn test_boost_factor_for_intent_understand_boosts_function() {
        let base_func = SymbolKind::Function.boost_factor(); // 1.4
        let boosted = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);

        // Should be 1.5x of base (1.4 * 1.5 = 2.1)
        assert!((boosted - base_func * 1.5).abs() < 0.001);
        assert!(boosted > base_func); // verify boost is positive
    }

    /// Test boost_factor_for_intent boosts methods for Understand intent
    #[test]
    fn test_boost_factor_for_intent_understand_boosts_method() {
        let base_method = SymbolKind::Method.boost_factor(); // 1.4
        let boosted = SymbolKind::Method.boost_factor_for_intent(&QueryIntent::Understand);

        // Should be 1.5x of base (1.4 * 1.5 = 2.1)
        assert!((boosted - base_method * 1.5).abs() < 0.001);
        assert!(boosted > base_method); // verify boost is positive
    }

    /// Test boost_factor_for_intent deprioritizes fields for Understand intent
    #[test]
    fn test_boost_factor_for_intent_understand_deprioritizes_field() {
        let base_field = SymbolKind::Field.boost_factor(); // 0.7
        let boosted = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

        // Should be 0.3x of base (0.7 * 0.3 = 0.21)
        assert!((boosted - base_field * 0.3).abs() < 0.001);
        assert!(boosted < base_field); // verify reduction
    }

    /// Test boost_factor_for_intent boosts structs for FindDefinition intent
    #[test]
    fn test_boost_factor_for_intent_find_definition_boosts_struct() {
        let base_struct = SymbolKind::Struct.boost_factor(); // 1.3
        let boosted = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::FindDefinition);

        // Should be 1.1x of base (1.3 * 1.1 = 1.43)
        assert!((boosted - base_struct * 1.1).abs() < 0.001);
        assert!(boosted > base_struct); // verify boost is positive
    }

    /// Test boost_factor_for_intent boosts enums for FindDefinition intent
    #[test]
    fn test_boost_factor_for_intent_find_definition_boosts_enum() {
        let base_enum = SymbolKind::Enum.boost_factor(); // 1.3
        let boosted = SymbolKind::Enum.boost_factor_for_intent(&QueryIntent::FindDefinition);

        // Should be 1.1x of base
        assert!((boosted - base_enum * 1.1).abs() < 0.001);
    }

    /// Test boost_factor_for_intent boosts traits for FindDefinition intent
    #[test]
    fn test_boost_factor_for_intent_find_definition_boosts_trait() {
        let base_trait = SymbolKind::Trait.boost_factor(); // 1.3
        let boosted = SymbolKind::Trait.boost_factor_for_intent(&QueryIntent::FindDefinition);

        // Should be 1.1x of base
        assert!((boosted - base_trait * 1.1).abs() < 0.001);
    }

    /// Test boost_factor_for_intent returns base for Search intent
    #[test]
    fn test_boost_factor_for_intent_search_uses_base() {
        let base_func = SymbolKind::Function.boost_factor();
        let boosted = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Search);

        // Should be unchanged for Search intent
        assert!((boosted - base_func).abs() < 0.001);
    }

    /// Test that Understand intent changes function/field relative ordering
    #[test]
    fn test_boost_factor_for_intent_understand_changes_ordering() {
        let func_boost = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);
        let field_boost = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

        // Function should be much higher than field with Understand intent
        // func: 1.4 * 1.2 = 1.68, field: 0.7 * 0.5 = 0.35
        assert!(func_boost > field_boost * 4.0);
    }

    // ==================== ContentType Tests ====================

    /// Test ContentType::from_document_type classifies SourceCode as Code
    #[test]
    fn test_content_type_from_document_type_source_code() {
        let content_type = ContentType::from_document_type(&DocumentType::SourceCode);
        assert_eq!(content_type, ContentType::Code);
    }

    /// Test ContentType::from_document_type classifies Test as Code
    #[test]
    fn test_content_type_from_document_type_test() {
        let content_type = ContentType::from_document_type(&DocumentType::Test);
        assert_eq!(content_type, ContentType::Code);
    }

    /// Test ContentType::from_document_type classifies Documentation as Doc
    #[test]
    fn test_content_type_from_document_type_documentation() {
        let content_type = ContentType::from_document_type(&DocumentType::Documentation);
        assert_eq!(content_type, ContentType::Doc);
    }

    /// Test ContentType::from_document_type classifies Notes as Notes
    #[test]
    fn test_content_type_from_document_type_notes() {
        let content_type = ContentType::from_document_type(&DocumentType::Notes);
        assert_eq!(content_type, ContentType::Notes);
    }

    /// Test ContentType::from_document_type classifies Benchmark as Notes
    #[test]
    fn test_content_type_from_document_type_benchmark() {
        let content_type = ContentType::from_document_type(&DocumentType::Benchmark);
        assert_eq!(content_type, ContentType::Notes);
    }

    /// Test ContentType::from_path classifies source files as Code
    #[test]
    fn test_content_type_from_path_source() {
        let path = Path::new("/project/src/main.rs");
        assert_eq!(ContentType::from_path(path), ContentType::Code);

        let path = Path::new("/project/src/lib.py");
        assert_eq!(ContentType::from_path(path), ContentType::Code);
    }

    /// Test ContentType::from_path classifies test files as Code
    #[test]
    fn test_content_type_from_path_test() {
        let path = Path::new("/project/tests/unit.rs");
        assert_eq!(ContentType::from_path(path), ContentType::Code);

        let path = Path::new("/project/src/parser_test.rs");
        assert_eq!(ContentType::from_path(path), ContentType::Code);
    }

    /// Test ContentType::from_path classifies doc files as Doc
    #[test]
    fn test_content_type_from_path_doc() {
        let path = Path::new("/project/doc/api.md");
        assert_eq!(ContentType::from_path(path), ContentType::Doc);

        let path = Path::new("/project/doc/guide.md");
        assert_eq!(ContentType::from_path(path), ContentType::Doc);
    }

    /// Test ContentType::from_path classifies notes files as Notes
    #[test]
    fn test_content_type_from_path_notes() {
        let path = Path::new("/project/notes/implementation.md");
        assert_eq!(ContentType::from_path(path), ContentType::Notes);

        let path = Path::new("/project/notes/benchmarks/perf.md");
        assert_eq!(ContentType::from_path(path), ContentType::Notes);
    }

    /// Test ContentType::all returns all 3 variants
    #[test]
    fn test_content_type_all() {
        let all = ContentType::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&ContentType::Code));
        assert!(all.contains(&ContentType::Doc));
        assert!(all.contains(&ContentType::Notes));
    }

    /// Test ContentType::name returns correct names
    #[test]
    fn test_content_type_name() {
        assert_eq!(ContentType::Code.name(), "code");
        assert_eq!(ContentType::Doc.name(), "docs");
        assert_eq!(ContentType::Notes.name(), "notes");
    }

    /// Test ContentType Display implementation
    #[test]
    fn test_content_type_display() {
        assert_eq!(format!("{}", ContentType::Code), "code");
        assert_eq!(format!("{}", ContentType::Doc), "docs");
        assert_eq!(format!("{}", ContentType::Notes), "notes");
    }
}
