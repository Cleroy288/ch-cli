//! Tests for MCP tool items and filtering.

use rustean::picker::mcp_items::{
    filter_mcp_tools, mcp_tool_count,
};

/// Total MCP tool count is 50
#[test]
fn mcp_tool_count_is_50() {
    assert_eq!(mcp_tool_count(), 50);
}

/// Empty query returns all tools
#[test]
fn filter_empty_returns_all() {
    // Act
    let items = filter_mcp_tools("");

    // Assert
    assert_eq!(items.len(), 50);
}

/// Filter by name substring matches
#[test]
fn filter_by_name_matches_code_search() {
    // Act
    let items = filter_mcp_tools("code_search");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "code_search");
}

/// Filter by description substring matches
#[test]
fn filter_by_desc_matches_jql() {
    // Act
    let items = filter_mcp_tools("JQL");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "jira_search");
}

/// Filter is case-insensitive
#[test]
fn filter_case_insensitive() {
    // Act
    let items = filter_mcp_tools("MEMORY");

    // Assert
    assert_eq!(items.len(), 3);
}

/// Filter with no match returns empty
#[test]
fn filter_no_match_returns_empty() {
    // Act
    let items = filter_mcp_tools("zzz_nonexistent");

    // Assert
    assert!(items.is_empty());
}

/// First tool is memory_search
#[test]
fn first_tool_is_memory_search() {
    // Act
    let items = filter_mcp_tools("");

    // Assert
    assert_eq!(items[0].name, "memory_search");
    assert_eq!(items[0].category, "Memory");
}
