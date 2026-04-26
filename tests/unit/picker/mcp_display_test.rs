use rustean::picker::mcp_display::{
	McpDisplayItem, build_unified_list,
};

#[test]
fn unified_list_includes_builtin() {
	// Act
	let items = build_unified_list(&[]);

	// Assert
	assert_eq!(items.len(), 50);
	assert!(
		items.iter().all(|i| i.source == "rustean")
	);
}

#[test]
fn unified_list_appends_discovered() {
	// Arrange
	let discovered = vec![McpDisplayItem {
		name: "ext_tool".to_string(),
		description: "External".to_string(),
		source: "context7".to_string(),
	}];

	// Act
	let items = build_unified_list(&discovered);

	// Assert
	assert_eq!(items.len(), 51);
	assert_eq!(items[50].name, "ext_tool");
	assert_eq!(items[50].source, "context7");
}
