//! Tests for TsParser symbol extraction.

use std::path::Path;

use rustean::indexer::parser::TsParser;
use rustean::indexer::symbols::SymbolKind;

/// Parse TS function declaration
#[test]
fn parse_ts_function_declaration() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
function greet(name: string): string {
    return "Hello " + name;
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let func = symbols
        .iter()
        .find(|s| s.name == "greet")
        .unwrap();
    assert_eq!(func.kind, SymbolKind::Function);
}

/// Parse TS arrow function assigned to const
#[test]
fn parse_ts_arrow_function() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
const add = (a: number, b: number): number => {
    return a + b;
};
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let func = symbols
        .iter()
        .find(|s| s.name == "add")
        .unwrap();
    assert_eq!(func.kind, SymbolKind::Function);
}

/// Parse TS class declaration
#[test]
fn parse_ts_class() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
class UserService {
    private name: string;

    constructor(name: string) {
        this.name = name;
    }

    getName(): string {
        return this.name;
    }
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert — class mapped to Struct kind
    let cls = symbols
        .iter()
        .find(|s| s.name == "UserService")
        .unwrap();
    assert_eq!(cls.kind, SymbolKind::Struct);

    // Assert — method detected
    let method = symbols
        .iter()
        .find(|s| s.name == "getName")
        .unwrap();
    assert_eq!(method.kind, SymbolKind::Method);
}

/// Parse TS interface declaration
#[test]
fn parse_ts_interface() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
interface User {
    id: number;
    name: string;
    email: string;
}
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert — interface mapped to Trait kind
    let iface = symbols
        .iter()
        .find(|s| s.name == "User")
        .unwrap();
    assert_eq!(iface.kind, SymbolKind::Trait);
}

/// Parse TS type alias
#[test]
fn parse_ts_type_alias() {
    // Arrange
    let mut parser = TsParser::typescript().unwrap();
    let source = r#"
type Result<T> = T | Error;
"#;

    // Act
    let symbols = parser
        .parse_source(source, Path::new("test.ts"))
        .unwrap();

    // Assert
    let alias = symbols
        .iter()
        .find(|s| s.name == "Result")
        .unwrap();
    assert_eq!(alias.kind, SymbolKind::TypeAlias);
}
