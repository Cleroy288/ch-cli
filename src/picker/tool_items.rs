use crate::domain::tool_ref::ToolKind;

/// A tool menu entry
pub struct ToolMenuItem {
    pub name: &'static str,
    pub description: &'static str,
    pub kind: ToolKind,
}

const TOOL_MENU: &[ToolMenuItem] = &[
    ToolMenuItem {
        name: "Branches",
        description: "Bitbucket branches",
        kind: ToolKind::Branch,
    },
    ToolMenuItem {
        name: "Pull Requests",
        description: "Open Bitbucket PRs",
        kind: ToolKind::PullRequest,
    },
    ToolMenuItem {
        name: "Jira Issues",
        description: "Jira issues & assigned",
        kind: ToolKind::Jira,
    },
    ToolMenuItem {
        name: "MCP Tools",
        description: "Reference an MCP tool",
        kind: ToolKind::McpTool,
    },
];

pub fn filter_tools(
    query: &str,
) -> Vec<&'static ToolMenuItem> {
    let lower = query.to_lowercase();
    TOOL_MENU
        .iter()
        .filter(|item| {
            item.name
                .to_lowercase()
                .starts_with(&lower)
        })
        .collect()
}

pub fn tool_count() -> usize {
    TOOL_MENU.len()
}
