use super::McpToolEntry;

/// Complete list of all MCP tools
pub const ALL_MCP_TOOLS: &[McpToolEntry] = &[
    // -- Memory (3) --
    mcp("memory_search", "Search past interactions", "Memory"),
    mcp("memory_recent", "Show recent interactions", "Memory"),
    mcp("memory_stats", "Show memory statistics", "Memory"),
    // -- Code (4) --
    mcp("code_search", "Search code symbols", "Code"),
    mcp("code_goto", "Find symbol definition", "Code"),
    mcp("code_refs", "Find all references", "Code"),
    mcp("code_callers", "Find callers of function", "Code"),
    // -- Nav (3) --
    mcp("code_symbols", "List project symbols", "Nav"),
    mcp("code_info", "Get symbol details", "Nav"),
    mcp("code_stats", "Show index statistics", "Nav"),
    // -- Bitbucket workspace (4) --
    mcp("bb_current_user", "Get authenticated user", "BB"),
    mcp("bb_list_workspaces", "List workspaces", "BB"),
    mcp("bb_list_workspace_members", "List members", "BB"),
    mcp("bb_list_workspace_projects", "List projects", "BB"),
    // -- Bitbucket repo (3) --
    mcp("bb_list_repositories", "List repos", "BB"),
    mcp("bb_get_repository", "Get repo details", "BB"),
    mcp("bb_list_default_reviewers", "List reviewers", "BB"),
    // -- Bitbucket refs (2) --
    mcp("bb_list_branches", "List branches", "BB"),
    mcp("bb_list_tags", "List tags", "BB"),
    // -- Bitbucket PR (5) --
    mcp("bb_list_pull_requests", "List pull requests", "BB"),
    mcp("bb_get_pull_request", "Get a pull request", "BB"),
    mcp("bb_get_pr_diff", "Get PR diff", "BB"),
    mcp("bb_get_pr_diffstat", "Get PR diffstat", "BB"),
    mcp("bb_list_pr_comments", "List PR comments", "BB"),
    // -- Bitbucket PR extra (4) --
    mcp("bb_get_pr_activity", "Get PR activity log", "BB"),
    mcp("bb_list_pr_commits", "List PR commits", "BB"),
    mcp("bb_list_pr_statuses", "List PR statuses", "BB"),
    mcp("bb_list_pr_tasks", "List PR tasks", "BB"),
    // -- Bitbucket infra (3) --
    mcp("bb_list_commits", "List repo commits", "BB"),
    mcp("bb_get_commit", "Get commit details", "BB"),
    mcp("bb_get_source", "Browse source at rev", "BB"),
    // -- Bitbucket pipeline (4) --
    mcp("bb_list_pipelines", "List pipelines", "BB"),
    mcp("bb_get_pipeline", "Get pipeline details", "BB"),
    mcp("bb_list_pipeline_steps", "List pipeline steps", "BB"),
    mcp("bb_list_environments", "List environments", "BB"),
    // -- Bitbucket download (1) --
    mcp("bb_list_downloads", "List repo downloads", "BB"),
    // -- Jira issue (5) --
    mcp("jira_search", "Search issues with JQL", "Jira"),
    mcp("jira_get_issue", "Get issue details", "Jira"),
    mcp("jira_list_comments", "List issue comments", "Jira"),
    mcp("jira_get_transitions", "Get transitions", "Jira"),
    mcp("jira_get_changelog", "Get issue changelog", "Jira"),
    // -- Jira project (5) --
    mcp("jira_list_projects", "List projects", "Jira"),
    mcp("jira_get_project", "Get project details", "Jira"),
    mcp("jira_list_components", "List components", "Jira"),
    mcp("jira_list_versions", "List versions", "Jira"),
    mcp("jira_get_watchers", "Get issue watchers", "Jira"),
    // -- Jira meta (4) --
    mcp("jira_list_priorities", "List priorities", "Jira"),
    mcp("jira_list_statuses", "List statuses", "Jira"),
    mcp("jira_list_issue_types", "List issue types", "Jira"),
    mcp("jira_list_labels", "List labels", "Jira"),
];

/// Shorthand constructor for McpToolEntry
const fn mcp(
    name: &'static str,
    description: &'static str,
    category: &'static str,
) -> McpToolEntry {
    McpToolEntry {
        name,
        description,
        category,
    }
}
