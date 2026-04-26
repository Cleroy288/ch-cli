mod mcp_items_data;

pub use mcp_items_data::ALL_MCP_TOOLS;

/// A single MCP tool entry
#[derive(Debug, Clone)]
pub struct McpToolEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub category: &'static str,
}

pub fn filter_mcp_tools(
    query: &str,
) -> Vec<&'static McpToolEntry> {
    if query.is_empty() {
        return ALL_MCP_TOOLS.iter().collect();
    }
    let lower = query.to_lowercase();
    ALL_MCP_TOOLS
        .iter()
        .filter(|t| {
            t.name.to_lowercase().contains(&lower)
                || t.description
                    .to_lowercase()
                    .contains(&lower)
        })
        .collect()
}

/// Total MCP tool count
pub fn mcp_tool_count() -> usize {
    ALL_MCP_TOOLS.len()
}
