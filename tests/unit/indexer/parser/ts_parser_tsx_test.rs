//! Tests for TsParser TSX and export support.

use std::path::Path;

use rustean::indexer::parser::TsParser;
use rustean::indexer::symbols::SymbolKind;

/// Parse TSX component (arrow function)
#[test]
fn parse_tsx_component() {
    // Arrange
    let mut parser = TsParser::tsx().unwrap();
    let source = r#"
const Header = ({ title }: Props) => {
    return <h1>{title}</h1>;
};
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.tsx"))
        .unwrap();

    // Assert
    let comp = symbols
        .iter()
        .find(|s| s.name == "Header")
        .unwrap();
    assert_eq!(comp.kind, SymbolKind::Function);
}

/// Parse TS enum declaration
#[test]
fn parse_ts_enum() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
enum Direction {
    North,
    South,
    East,
    West,
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let enm = symbols
        .iter()
        .find(|s| s.name == "Direction")
        .unwrap();
    assert_eq!(enm.kind, SymbolKind::Enum);
}

/// Parse exported function
#[test]
fn parse_ts_exported_function() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
export function fetchUser(id: string): User {
    return db.get(id);
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let func = symbols
        .iter()
        .find(|s| s.name == "fetchUser")
        .unwrap();
    assert_eq!(func.kind, SymbolKind::Function);
}

/// Parse exported interface
#[test]
fn parse_ts_exported_interface() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
export interface ApiResponse {
    status: number;
    data: unknown;
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let iface = symbols
        .iter()
        .find(|s| s.name == "ApiResponse")
        .unwrap();
    assert_eq!(iface.kind, SymbolKind::Trait);
}

/// Empty source returns no symbols
#[test]
fn parse_ts_empty_source_no_symbols() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();

    // Act
    let symbols = parser
        .parse_source("", Path::new("empty.ts"))
        .unwrap();

    // Assert
    assert!(symbols.is_empty());
}
