//! Tests for ADF → plain text extraction.

use rustean::service::tools::jira_adf::adf_to_text;
use serde_json::json;

/// Null ADF value produces empty string
#[test]
fn adf_to_text_null_value_empty() {
    // Arrange
    let adf = json!(null);

    // Act
    let result = adf_to_text(&adf);

    // Assert
    assert_eq!(result, "");
}

/// Empty object with no content or text
#[test]
fn adf_to_text_empty_object_empty() {
    // Arrange
    let adf = json!({});

    // Act
    let result = adf_to_text(&adf);

    // Assert
    assert_eq!(result, "");
}

/// Simple text node extracts its text
#[test]
fn adf_to_text_simple_text_node() {
    // Arrange
    let adf = json!({
        "type": "text",
        "text": "hello"
    });

    // Act
    let result = adf_to_text(&adf);

    // Assert
    assert_eq!(result, "hello");
}

/// Paragraph with nested text node
#[test]
fn adf_to_text_paragraph_with_text() {
    // Arrange
    let adf = json!({
        "type": "paragraph",
        "content": [
            {"type": "text", "text": "world"}
        ]
    });

    // Act
    let result = adf_to_text(&adf);

    // Assert
    assert_eq!(result, "world");
}

/// Multiple text nodes concatenated
#[test]
fn adf_to_text_multiple_nodes_concat() {
    // Arrange
    let adf = json!({
        "type": "doc",
        "content": [
            {
                "type": "paragraph",
                "content": [
                    {"type": "text", "text": "foo "},
                    {"type": "text", "text": "bar"}
                ]
            }
        ]
    });

    // Act
    let result = adf_to_text(&adf);

    // Assert
    assert_eq!(result, "foo bar");
}
