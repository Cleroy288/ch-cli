use super::mcp_items::ALL_MCP_TOOLS;

/// A unified MCP tool display item
#[derive(Debug, Clone)]
pub struct McpDisplayItem {
	pub name: String,
	pub description: String,
	/// Source label ("rustean", "context7", etc.)
	pub source: String,
}

pub fn build_unified_list(
	discovered: &[McpDisplayItem],
) -> Vec<McpDisplayItem> {
	let mut items: Vec<McpDisplayItem> =
		ALL_MCP_TOOLS
			.iter()
			.map(|tool| McpDisplayItem {
				name: tool.name.to_string(),
				description: tool
					.description
					.to_string(),
				source: "rustean".to_string(),
			})
			.collect();
	items.extend(discovered.iter().cloned());
	items
}

/// Substring match on name, description, source
pub(super) fn matches_query(
	item: &McpDisplayItem,
	lower: &str,
) -> bool {
	item.name.to_lowercase().contains(lower)
		|| item
			.description
			.to_lowercase()
			.contains(lower)
		|| item
			.source
			.to_lowercase()
			.contains(lower)
}
