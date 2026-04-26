mod command;

// Tool definitions -- existing
pub mod mcp_defs_code;
pub mod mcp_defs_memory;
pub mod mcp_defs_nav;
pub mod mcp_tool_defs;
mod mcp_tool_defs_extra;

// Tool definitions -- Atlassian
pub mod mcp_defs_bb;
pub mod mcp_defs_bb_extra;
pub mod mcp_defs_bb_pipe;
pub mod mcp_defs_bb_pr;
pub mod mcp_defs_jira;
pub mod mcp_defs_jira_meta;
pub mod mcp_defs_shared;

// Tool definitions -- Aikido
pub mod mcp_defs_aikido;

// Formatters
pub mod mcp_format;
pub mod mcp_format_aikido;
mod mcp_format_aikido_detail;
mod mcp_format_aikido_issue;
mod mcp_format_aikido_list;
pub mod mcp_format_code;
pub mod mcp_format_info;
pub mod mcp_format_nav;

// Execution -- existing
pub mod mcp_exec_code;
pub mod mcp_exec_nav;
pub mod mcp_exec_search;

// Execution -- Atlassian
pub mod mcp_exec_bb;
pub mod mcp_exec_bb_pipe;
pub mod mcp_exec_bb_pr;
pub mod mcp_exec_jira;
pub mod mcp_exec_jira_path;

// Execution -- Aikido
pub mod mcp_exec_aikido;
mod mcp_exec_aikido_filter;

// Infrastructure
pub mod mcp_dispatch;
pub mod mcp_handlers;
pub mod mcp_helpers;
pub mod mcp_helpers_bb;
pub mod mcp_tools;
mod mcp_tools_exec;
pub mod mcp_types;

pub use command::mcp_server_command;
