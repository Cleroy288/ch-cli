//! Unit tests for cli::commands::search
//! — migrated from inline tests

use rustean::cli::commands::error::CommandError;
use rustean::cli::commands::search::parse_symbol_kind;
use rustean::indexer::SymbolKind;

/// parse_symbol_kind accepts function aliases
#[test]
fn parse_kind_function_variants() {
    assert!(matches!(
        parse_symbol_kind("function"),
        Ok(SymbolKind::Function)
    ));
    assert!(matches!(
        parse_symbol_kind("fn"),
        Ok(SymbolKind::Function)
    ));
    assert!(matches!(
        parse_symbol_kind("FUNCTION"),
        Ok(SymbolKind::Function)
    ));
}

/// parse_symbol_kind recognizes "struct"
#[test]
fn parse_kind_struct() {
    assert!(matches!(
        parse_symbol_kind("struct"),
        Ok(SymbolKind::Struct)
    ));
}

/// parse_symbol_kind rejects unknown kind string
#[test]
fn parse_kind_invalid() {
    let result = parse_symbol_kind("invalid");
    assert!(result.is_err());
    match result {
        Err(CommandError::InvalidKind(s)) => {
            assert_eq!(s, "invalid")
        }
        _ => panic!("Expected InvalidKind"),
    }
}

/// parse_symbol_kind rejects empty string
#[test]
fn parse_kind_empty() {
    assert!(parse_symbol_kind("").is_err());
}
