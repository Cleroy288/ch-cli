use std::path::PathBuf;

use crate::domain::tool_ref::ToolKind;

/// Current state of the picker overlay.
#[derive(Debug, Clone, PartialEq)]
pub enum PickerMode {
    Inactive,
    Browse { dir: PathBuf },
    Symbols {
        file_path: PathBuf,
        parent: Option<String>,
    },
    Tools,
    ToolLoading { tool: ToolKind },
    ToolResults { tool: ToolKind },
    RepoSelect { tool: ToolKind },
    McpToolBrowse,
    SlashCommand,
    SlashArg { command: String },
    JiraAssigneeFilter,
    JiraBoardSelect,
    JiraTicketDetail,
    GitRepoSelect,
    GitLoading,
    GitHistory,
    GitDetail,
}
