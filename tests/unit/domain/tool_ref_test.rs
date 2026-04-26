//! Tests for domain tool reference types.

use rustean::domain::tool_ref::{
    ToolKind, ToolReference, format_tool_ref,
};

/// Branch prefix is correct
#[test]
fn branch_prefix_is_hash_branch() {
    assert_eq!(ToolKind::Branch.prefix(), "#branch");
}

/// PR prefix is correct
#[test]
fn pr_prefix_is_hash_pr() {
    assert_eq!(
        ToolKind::PullRequest.prefix(), "#PR"
    );
}

/// Jira prefix is correct
#[test]
fn jira_prefix_is_hash_jira() {
    assert_eq!(ToolKind::Jira.prefix(), "#JIRA");
}

/// format_tool_ref builds branch reference
#[test]
fn format_branch_ref() {
    // Act
    let result = format_tool_ref(
        ToolKind::Branch,
        "feat/login",
        None,
    );

    // Assert
    assert_eq!(result, "#branch[feat/login]");
}

/// format_tool_ref builds PR reference
#[test]
fn format_pr_ref_with_title() {
    // Act
    let result = format_tool_ref(
        ToolKind::PullRequest,
        "42 - Fix bug",
        None,
    );

    // Assert
    assert_eq!(result, "#PR[42 - Fix bug]");
}

/// format_tool_ref builds Jira reference
#[test]
fn format_jira_ref() {
    // Act
    let result = format_tool_ref(
        ToolKind::Jira,
        "EVB-123 - Add feature",
        None,
    );

    // Assert
    assert_eq!(
        result,
        "#JIRA[EVB-123 - Add feature]"
    );
}

/// format_tool_ref with repo folder
#[test]
fn format_branch_ref_with_folder() {
    // Act
    let result = format_tool_ref(
        ToolKind::Branch,
        "feat/login",
        Some("eevee-backend"),
    );

    // Assert
    assert_eq!(
        result,
        "#branch[eevee-backend:feat/login]",
    );
}

/// ToolReference stores span and kind
#[test]
fn tool_reference_creation() {
    // Arrange & Act
    let tool_ref = ToolReference {
        start: 5,
        end: 25,
        kind: ToolKind::Branch,
        key: "main".to_string(),
        display: "main".to_string(),
    };

    // Assert
    assert_eq!(tool_ref.start, 5);
    assert_eq!(tool_ref.end, 25);
    assert_eq!(tool_ref.kind, ToolKind::Branch);
    assert_eq!(tool_ref.key, "main");
}

/// McpTool prefix is correct
#[test]
fn mcp_tool_prefix_is_hash_tool() {
    assert_eq!(ToolKind::McpTool.prefix(), "#tool");
}

/// format_tool_ref builds MCP tool reference
#[test]
fn format_mcp_tool_ref() {
    // Act
    let result = format_tool_ref(
        ToolKind::McpTool,
        "code_search",
        None,
    );

    // Assert
    assert_eq!(result, "#tool[code_search]");
}

/// ToolKind implements Copy and Eq
#[test]
fn tool_kind_is_copy_and_eq() {
    // Arrange
    let kind = ToolKind::Jira;
    let copy = kind;

    // Assert
    assert_eq!(kind, copy);
}
