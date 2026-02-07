//! Tests for symbol types module.

use std::path::Path;

use ch_cli::indexer::symbols::{is_test_file, ContentType, DocumentType, SymbolKind};
use ch_cli::retrieval::daemon::protocol::QueryIntent;

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
    assert_eq!(DocumentType::Test.boost_factor(), 0.8);
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
    assert_eq!(SymbolKind::Module.boost_factor(), 1.3); // increased for better definition ranking
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
    // Struct and Module now have equal boost (1.3) for better definition ranking
    assert!(SymbolKind::Struct.boost_factor() >= SymbolKind::Module.boost_factor());
    assert!(SymbolKind::Module.boost_factor() > SymbolKind::Macro.boost_factor());
    assert!(SymbolKind::Macro.boost_factor() > SymbolKind::Constant.boost_factor());
    assert!(SymbolKind::Constant.boost_factor() > SymbolKind::Field.boost_factor());
    assert!(SymbolKind::Field.boost_factor() > SymbolKind::DocumentChunk.boost_factor());
}

/// Test is_test_file helper detects all test patterns
#[test]
fn test_is_test_file_all_patterns() {
    // /tests/ directory pattern
    assert!(is_test_file("/project/tests/unit.rs"));
    assert!(is_test_file("/project/tests/integration/api.rs"));

    // _test.rs suffix pattern
    assert!(is_test_file("/project/src/parser_test.rs"));
    assert!(is_test_file("/src/handler_test.rs"));

    // _tests.rs suffix pattern
    assert!(is_test_file("/project/src/parser_tests.rs"));
    assert!(is_test_file("/src/handler_tests.rs"));

    // test_ prefix pattern
    assert!(is_test_file("/project/src/test_parser.rs"));
    assert!(is_test_file("/src/test_utils.rs"));
}

/// Test is_test_file helper rejects non-test files
#[test]
fn test_is_test_file_non_test_files() {
    // Regular source files should not match
    assert!(!is_test_file("/project/src/main.rs"));
    assert!(!is_test_file("/project/src/parser.rs"));
    assert!(!is_test_file("/project/src/latest_feature.rs"));
    assert!(!is_test_file("/project/src/contest.rs"));
}

/// Test boost_factor_for_query reduces doc boost for "implementation" query
#[test]
fn test_boost_factor_for_query_implementation() {
    let query = "show me the implementation"; // implementation keyword present

    // Documentation types should be reduced by 0.5x
    let doc_boost = DocumentType::Documentation.boost_factor_for_query(query);
    let notes_boost = DocumentType::Notes.boost_factor_for_query(query);
    let benchmark_boost = DocumentType::Benchmark.boost_factor_for_query(query);

    assert_eq!(doc_boost, 1.0 * 0.5);
    assert_eq!(notes_boost, 0.7 * 0.5);
    assert_eq!(benchmark_boost, 0.3 * 0.5);

    // Code types should remain unchanged
    let source_boost = DocumentType::SourceCode.boost_factor_for_query(query);
    let test_boost = DocumentType::Test.boost_factor_for_query(query);

    assert_eq!(source_boost, 1.5);
    assert_eq!(test_boost, 0.8);
}

/// Test boost_factor_for_query unchanged for "where is X" query
#[test]
fn test_boost_factor_for_query_where_is() {
    let query = "where is the AuthService defined"; // no implementation keywords

    // All types should return base boost
    assert_eq!(DocumentType::SourceCode.boost_factor_for_query(query), 1.5);
    assert_eq!(
        DocumentType::Documentation.boost_factor_for_query(query),
        1.0
    );
    assert_eq!(DocumentType::Test.boost_factor_for_query(query), 0.8);
    assert_eq!(DocumentType::Notes.boost_factor_for_query(query), 0.7);
    assert_eq!(DocumentType::Benchmark.boost_factor_for_query(query), 0.3);
}

/// Test boost_factor_for_query with various implementation keywords
#[test]
fn test_boost_factor_for_query_all_keywords() {
    let keywords = [
        "implementation",
        "algorithm",
        "logic",
        "code",
        "function",
        "method",
    ];

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
    let base_func = SymbolKind::Function.boost_factor();
    let boosted = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);

    // Should be 1.5x of base (1.4 * 1.5 = 2.1)
    assert!((boosted - base_func * 1.5).abs() < 0.001);
    assert!(boosted > base_func);
}

/// Test boost_factor_for_intent boosts methods for Understand intent
#[test]
fn test_boost_factor_for_intent_understand_boosts_method() {
    let base_method = SymbolKind::Method.boost_factor();
    let boosted = SymbolKind::Method.boost_factor_for_intent(&QueryIntent::Understand);

    // Should be 1.5x of base (1.4 * 1.5 = 2.1)
    assert!((boosted - base_method * 1.5).abs() < 0.001);
    assert!(boosted > base_method);
}

/// Test boost_factor_for_intent deprioritizes fields for Understand intent
#[test]
fn test_boost_factor_for_intent_understand_deprioritizes_field() {
    let base_field = SymbolKind::Field.boost_factor();
    let boosted = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

    // Should be 0.3x of base (0.7 * 0.3 = 0.21)
    assert!((boosted - base_field * 0.3).abs() < 0.001);
    assert!(boosted < base_field);
}

/// Test boost_factor_for_intent boosts structs for FindDefinition intent
#[test]
fn test_boost_factor_for_intent_find_definition_boosts_struct() {
    let base_struct = SymbolKind::Struct.boost_factor();
    let boosted = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::FindDefinition);

    // Should be 2.0x of base (1.3 * 2.0 = 2.6)
    assert!((boosted - base_struct * 2.0).abs() < 0.001);
    assert!(boosted > base_struct);
}

/// Test boost_factor_for_intent boosts enums for FindDefinition intent
#[test]
fn test_boost_factor_for_intent_find_definition_boosts_enum() {
    let base_enum = SymbolKind::Enum.boost_factor();
    let boosted = SymbolKind::Enum.boost_factor_for_intent(&QueryIntent::FindDefinition);

    // Should be 2.0x of base
    assert!((boosted - base_enum * 2.0).abs() < 0.001);
}

/// Test boost_factor_for_intent boosts traits for FindDefinition intent
#[test]
fn test_boost_factor_for_intent_find_definition_boosts_trait() {
    let base_trait = SymbolKind::Trait.boost_factor();
    let boosted = SymbolKind::Trait.boost_factor_for_intent(&QueryIntent::FindDefinition);

    // Should be 2.0x of base
    assert!((boosted - base_trait * 2.0).abs() < 0.001);
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

/// Test DocumentType::boost_factor_for_intent boosts source code 3.0x for Understand
#[test]
fn test_document_type_understand_source_boost() {
    let base = DocumentType::SourceCode.boost_factor(); // 1.5
    let boosted = DocumentType::SourceCode.boost_factor_for_intent(&QueryIntent::Understand);

    // Should be 3.0x of base (1.5 * 3.0 = 4.5)
    assert!((boosted - base * 3.0).abs() < 0.001);
    assert_eq!(boosted, 4.5);
}

/// Test DocumentType::boost_factor_for_intent reduces docs for Understand
#[test]
fn test_document_type_understand_reduces_docs() {
    let base = DocumentType::Documentation.boost_factor(); // 1.0
    let boosted = DocumentType::Documentation.boost_factor_for_intent(&QueryIntent::Understand);

    // Should be 0.1x of base (1.0 * 0.1 = 0.1) - heavily reduced to favor source code
    assert!((boosted - base * 0.1).abs() < 0.001);
    assert_eq!(boosted, 0.1);
}
