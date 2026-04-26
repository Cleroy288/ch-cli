#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolKind {
    Branch,
    PullRequest,
    Jira,
    McpTool,
}

#[derive(Debug, Clone)]
pub struct ToolItem {
    pub key: String,
    pub display: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct ToolReference {
    pub start: usize,
    pub end: usize,
    pub kind: ToolKind,
    pub key: String,
    pub display: String,
}

impl ToolKind {
    pub fn prefix(&self) -> &'static str {
        match self {
            Self::Branch => "#branch",
            Self::PullRequest => "#PR",
            Self::Jira => "#JIRA",
            Self::McpTool => "#tool",
        }
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Branch => "Branches",
            Self::PullRequest => "Pull Requests",
            Self::Jira => "Jira Issues",
            Self::McpTool => "MCP Tools",
        }
    }
}

#[derive(Debug, Clone)]
pub enum ToolFetchResult {
    Items(Vec<ToolItem>),
    JiraAssignees(Vec<String>),
    JiraBoard(super::jira::JiraBoardData),
    JiraDetail(
        super::jira_detail::JiraIssueDetail,
    ),
}

pub fn format_tool_ref(
    kind: ToolKind,
    display: &str,
    repo_folder: Option<&str>,
) -> String {
    match repo_folder {
        Some(folder) => format!(
            "{}[{}:{}]",
            kind.prefix(), folder, display,
        ),
        None => format!(
            "{}[{}]", kind.prefix(), display,
        ),
    }
}
