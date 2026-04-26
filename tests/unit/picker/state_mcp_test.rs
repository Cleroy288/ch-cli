//! Tests for Picker MCP browse state methods.

use rustean::picker::{Picker, PickerMode};

/// activate_mcp_browse sets McpToolBrowse mode
#[test]
fn activate_sets_mcp_browse_mode() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_tools(0);

    // Act
    picker.activate_mcp_browse();

    // Assert
    assert_eq!(
        *picker.mode(),
        PickerMode::McpToolBrowse,
    );
}

/// activate_mcp_browse clears query
#[test]
fn activate_clears_query() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_tools(0);
    picker.push_query('x');

    // Act
    picker.activate_mcp_browse();

    // Assert
    assert_eq!(picker.query(), "");
}

/// filtered returns all items with empty query
#[test]
fn filtered_returns_all_with_empty_query() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_mcp_browse();

    // Act
    let items = picker.filtered_mcp_items();

    // Assert
    assert_eq!(items.len(), 50);
}

/// selected returns first item by default
#[test]
fn selected_returns_first_by_default() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_mcp_browse();

    // Act
    let tool = picker.selected_mcp_tool();

    // Assert
    assert!(tool.is_some());
    assert_eq!(tool.unwrap().name, "memory_search");
}

/// back_to_tools_from_mcp returns to Tools mode
#[test]
fn back_returns_to_tools_mode() {
    // Arrange
    let mut picker = Picker::default();
    picker.activate_mcp_browse();

    // Act
    picker.back_to_tools_from_mcp();

    // Assert
    assert_eq!(*picker.mode(), PickerMode::Tools);
}
