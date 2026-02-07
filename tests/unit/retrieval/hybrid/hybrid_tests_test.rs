//! Tests for hybrid search module
//!
//! Contains unit tests for document type boost, symbol kind boost,
//! query-aware boost, and intent-aware boost functionality.

use std::path::PathBuf;

use ch_cli::indexer::symbols::{DocumentType, SymbolKind};
use ch_cli::retrieval::daemon::protocol::QueryIntent;
use ch_cli::retrieval::hybrid::converters::parse_symbol_kind;

/// Test that source code files get higher boost than benchmark files
#[test]
fn test_source_code_boosted_over_benchmark() {
    let source_path = PathBuf::from("src/main.rs");
    let benchmark_path = PathBuf::from("notes/benchmarks/perf.md");

    let source_doc_type = DocumentType::from_path(&source_path);
    let benchmark_doc_type = DocumentType::from_path(&benchmark_path);

    let source_boost = source_doc_type.boost_factor();
    let benchmark_boost = benchmark_doc_type.boost_factor();

    assert!(
        source_boost > benchmark_boost,
        "Source boost ({}) should be > benchmark boost ({})",
        source_boost,
        benchmark_boost
    );
}

/// Test that functions get higher boost than fields
#[test]
fn test_function_boosted_over_field() {
    let function_boost = SymbolKind::Function.boost_factor();
    let field_boost = SymbolKind::Field.boost_factor();

    assert!(
        function_boost > field_boost,
        "Function boost ({}) should be > field boost ({})",
        function_boost,
        field_boost
    );
}

/// Test combined boost calculation
#[test]
fn test_combined_boost_calculation() {
    let source_path = PathBuf::from("src/lib.rs");
    let notes_path = PathBuf::from("notes/design.md");

    let source_func_boost =
        DocumentType::from_path(&source_path).boost_factor() * SymbolKind::Function.boost_factor();
    let notes_field_boost =
        DocumentType::from_path(&notes_path).boost_factor() * SymbolKind::Field.boost_factor();

    assert!(
        source_func_boost > notes_field_boost,
        "Source function ({}) should beat notes field ({})",
        source_func_boost,
        notes_field_boost
    );
}

/// Test parse_symbol_kind function
#[test]
fn test_parse_symbol_kind() {
    assert_eq!(parse_symbol_kind("fn"), SymbolKind::Function);
    assert_eq!(parse_symbol_kind("function"), SymbolKind::Function);
    assert_eq!(parse_symbol_kind("struct"), SymbolKind::Struct);
    assert_eq!(parse_symbol_kind("field"), SymbolKind::Field);
    assert_eq!(parse_symbol_kind("method"), SymbolKind::Method);
    assert_eq!(parse_symbol_kind("const"), SymbolKind::Constant);
}

/// Test query-aware boost reduces documentation boost for code queries
#[test]
fn test_query_aware_boost_reduces_doc() {
    let doc_path = PathBuf::from("doc/api.md");
    let query_impl = "show me the implementation";
    let query_where = "where is AuthService";

    let doc_type = DocumentType::from_path(&doc_path);

    let impl_boost = doc_type.boost_factor_for_query(query_impl);
    let where_boost = doc_type.boost_factor_for_query(query_where);

    assert_eq!(impl_boost, 0.5);
    assert_eq!(where_boost, 1.0);
    assert!(
        where_boost > impl_boost,
        "Where query should not reduce boost"
    );
}

/// Test query-aware boost keeps source code unchanged
#[test]
fn test_query_aware_boost_source_unchanged() {
    let source_path = PathBuf::from("src/main.rs");
    let query_impl = "show me the implementation";

    let source_type = DocumentType::from_path(&source_path);
    let base_boost = source_type.boost_factor();
    let impl_boost = source_type.boost_factor_for_query(query_impl);

    assert_eq!(impl_boost, base_boost);
    assert_eq!(impl_boost, 1.5);
}

/// Test combined boost uses query-aware document boost
#[test]
fn test_combined_boost_query_aware() {
    let source_path = PathBuf::from("src/lib.rs");
    let notes_path = PathBuf::from("/project/notes/design.md");
    let query = "show me the implementation";

    let source_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
        * SymbolKind::Function.boost_factor();

    let notes_boost = DocumentType::from_path(&notes_path).boost_factor_for_query(query)
        * SymbolKind::Function.boost_factor();

    assert!(
        source_boost > notes_boost * 4.0,
        "Source ({}) should beat notes ({}) by 4x+ for impl query",
        source_boost,
        notes_boost
    );
}

/// Test intent-aware boost: Understand intent boosts functions over fields
#[test]
fn test_intent_boost_understand_functions() {
    let func_base = SymbolKind::Function.boost_factor();
    let field_base = SymbolKind::Field.boost_factor();

    let func_understand = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);
    let field_understand = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

    assert!((func_understand - func_base * 1.5).abs() < 0.001);
    assert!((field_understand - field_base * 0.3).abs() < 0.001);

    assert!(
        func_understand > field_understand * 4.0,
        "Function ({}) should be 4x+ field ({}) for Understand",
        func_understand,
        field_understand
    );
}

/// Test intent-aware boost: FindDefinition intent boosts structs/enums/traits
#[test]
fn test_intent_boost_find_definition_types() {
    let struct_base = SymbolKind::Struct.boost_factor();
    let enum_base = SymbolKind::Enum.boost_factor();
    let trait_base = SymbolKind::Trait.boost_factor();

    let struct_def = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::FindDefinition);
    let enum_def = SymbolKind::Enum.boost_factor_for_intent(&QueryIntent::FindDefinition);
    let trait_def = SymbolKind::Trait.boost_factor_for_intent(&QueryIntent::FindDefinition);

    assert!((struct_def - struct_base * 2.0).abs() < 0.001);
    assert!((enum_def - enum_base * 2.0).abs() < 0.001);
    assert!((trait_def - trait_base * 2.0).abs() < 0.001);
}

/// Test intent-aware boost: Search intent uses base boost
#[test]
fn test_intent_boost_search_uses_base() {
    let func_base = SymbolKind::Function.boost_factor();
    let field_base = SymbolKind::Field.boost_factor();
    let struct_base = SymbolKind::Struct.boost_factor();

    let func_search = SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Search);
    let field_search = SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Search);
    let struct_search = SymbolKind::Struct.boost_factor_for_intent(&QueryIntent::Search);

    assert!((func_search - func_base).abs() < 0.001);
    assert!((field_search - field_base).abs() < 0.001);
    assert!((struct_search - struct_base).abs() < 0.001);
}

/// Test combined doc + intent boost for Understand query
#[test]
fn test_combined_doc_and_intent_boost() {
    let source_path = PathBuf::from("src/lib.rs");
    let query = "show me the implementation";

    let func_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
        * SymbolKind::Function.boost_factor_for_intent(&QueryIntent::Understand);
    let field_boost = DocumentType::from_path(&source_path).boost_factor_for_query(query)
        * SymbolKind::Field.boost_factor_for_intent(&QueryIntent::Understand);

    assert!(
        func_boost > field_boost * 4.0,
        "Function ({}) should be 4x+ field ({}) with combined boosts",
        func_boost,
        field_boost
    );
}
