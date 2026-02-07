//! Tests for doc_parser module

use std::path::PathBuf;

use ch_cli::indexer::doc_parser::DocParser;

#[test]
fn test_parse_simple_markdown() {
    let content = r#"# Main Title

This is the introduction.

## Section One

Content of section one.

## Section Two

Content of section two.

### Subsection

Nested content.
"#;

    let path = PathBuf::from("test.md");
    let symbols = DocParser::parse(&path, content);

    assert_eq!(symbols.len(), 4);
    assert_eq!(symbols[0].name, "Main Title");
    assert_eq!(symbols[1].name, "Section One");
    assert_eq!(symbols[2].name, "Section Two");
    assert_eq!(symbols[3].name, "Subsection");
    assert_eq!(symbols[3].parent, Some("Section Two".to_string()));
}

#[test]
fn test_parse_header_levels() {
    // Note: parse_header is private, testing via parse
    let content = "# Title\nContent";
    let path = PathBuf::from("test.md");
    let symbols = DocParser::parse(&path, content);
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name, "Title");
}

#[test]
fn test_content_extraction() {
    let content = r#"# Header

Line 1
Line 2
Line 3
"#;

    let path = PathBuf::from("test.md");
    let symbols = DocParser::parse(&path, content);

    assert_eq!(symbols.len(), 1);
    assert!(symbols[0].content.as_ref().unwrap().contains("Line 1"));
    assert!(symbols[0].content.as_ref().unwrap().contains("Line 2"));
}

#[test]
fn test_intro_without_header() {
    let content = r#"This is intro content without a header.

# First Header

Content after header.
"#;

    let path = PathBuf::from("test.md");
    let symbols = DocParser::parse(&path, content);

    assert_eq!(symbols.len(), 2);
    assert_eq!(symbols[0].name, "Introduction");
    assert_eq!(symbols[1].name, "First Header");
}
