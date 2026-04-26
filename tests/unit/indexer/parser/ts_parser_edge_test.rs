//! Edge-case tests for TsParser: dedup, .d.ts, etc.

use std::path::Path;

use rustean::indexer::parser::TsParser;
use rustean::indexer::symbols::SymbolKind;

/// Exported function must not produce duplicates
#[test]
fn exported_function_no_duplicate() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
export function save(data: Data): void {
    db.insert(data);
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert — exactly one "save" symbol
    let count = symbols
        .iter()
        .filter(|s| s.name == "save")
        .count();
    assert_eq!(count, 1, "expected 1 'save', got {count}");
}

/// Exported arrow function must not produce duplicates
#[test]
fn exported_arrow_no_duplicate() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
export const handler = (req: Request) => {
    return new Response("ok");
};
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let count = symbols
        .iter()
        .filter(|s| s.name == "handler")
        .count();
    assert_eq!(
        count, 1,
        "expected 1 'handler', got {count}"
    );
}

/// Mixed exported + private — no duplicates
#[test]
fn mixed_exports_no_duplicates() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
export interface Config { key: string; }
export class Service {}
function helper() {}
type ID = string;
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert — each name appears exactly once
    for name in ["Config", "Service", "helper", "ID"] {
        let count = symbols
            .iter()
            .filter(|s| s.name == name)
            .count();
        assert_eq!(
            count, 1,
            "expected 1 '{name}', got {count}"
        );
    }
}

/// has_parser_support excludes .d.ts files
#[test]
fn d_ts_excluded_from_parser_support() {
    use rustean::app::handlers::picker_symbol_actions
        ::has_parser_support;

    // Assert
    assert!(!has_parser_support("types.d.ts"));
    assert!(!has_parser_support("index.d.ts"));
    assert!(has_parser_support("index.ts"));
    assert!(has_parser_support("App.tsx"));
    assert!(has_parser_support("main.rs"));
    assert!(!has_parser_support("readme.md"));
}
