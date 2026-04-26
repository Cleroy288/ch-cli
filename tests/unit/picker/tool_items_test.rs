//! Tests for tool menu items and filtering.

use rustean::picker::tool_items::{
    filter_tools, tool_count,
};

/// Total tool count is 4
#[test]
fn tool_count_is_four() {
    assert_eq!(tool_count(), 4);
}

/// Empty query returns all tools
#[test]
fn filter_empty_returns_all() {
    // Act
    let items = filter_tools("");

    // Assert
    assert_eq!(items.len(), 4);
}

/// Filter by 'b' returns Branches only
#[test]
fn filter_by_b_returns_branches() {
    // Act
    let items = filter_tools("b");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Branches");
}

/// Filter by 'p' returns Pull Requests
#[test]
fn filter_by_p_returns_pull_requests() {
    // Act
    let items = filter_tools("p");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Pull Requests");
}

/// Filter by 'j' returns Jira Issues
#[test]
fn filter_by_j_returns_jira() {
    // Act
    let items = filter_tools("j");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Jira Issues");
}

/// Filter by non-matching query returns empty
#[test]
fn filter_no_match_returns_empty() {
    // Act
    let items = filter_tools("xyz");

    // Assert
    assert!(items.is_empty());
}

/// Filter by 'm' returns MCP Tools
#[test]
fn filter_by_m_returns_mcp_tools() {
    // Act
    let items = filter_tools("m");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "MCP Tools");
}

/// Filter is case-insensitive
#[test]
fn filter_case_insensitive() {
    // Act
    let items = filter_tools("B");

    // Assert
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Branches");
}
