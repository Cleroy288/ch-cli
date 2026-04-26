//! Tests for the parser module.
//!
//! Contains unit tests for RustParser symbol and reference extraction.

use std::path::Path;

use rustean::indexer::parser::RustParser;
use rustean::indexer::symbols::{SymbolKind, Visibility};

#[test]
fn test_parse_simple_function() {
    let mut parser = RustParser::new().unwrap();
    let source = r#"
fn hello() {
    println!("Hello, world!");
}
"#;
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

    // Should find the struct
    assert!(symbols
        .iter()
        .any(|s| s.name == "Point" && s.kind == SymbolKind::Struct));
    // Should find the impl
    assert!(symbols
        .iter()
        .any(|s| s.name == "Point" && s.kind == SymbolKind::Impl));
}

#[test]
fn test_parse_const_and_static() {
    let mut parser = RustParser::new().unwrap();
    let source = r#"
pub const MAX_SIZE: usize = 100;
static mut COUNTER: i32 = 0;
"#;
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

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
    let symbols = parser.parse_source(source, Path::new("test.rs")).unwrap();

    let alias = symbols.iter().find(|s| s.name == "Result").unwrap();
    assert_eq!(alias.kind, SymbolKind::TypeAlias);
}

#[test]
fn test_parse_real_file() {
    let mut parser = RustParser::new().unwrap();
    let symbols = parser
        .parse_file("src/lib.rs")
        .expect("failed to parse lib.rs");
    // Should find at least the App re-export
    assert!(!symbols.is_empty());
}
