// -- Status messages --
pub const STATUS_NO_SYMBOLS: &str =
	" No symbols available ";

// -- Input --
pub const INPUT_WAITING: &str =
	"Waiting for Claude...";
pub const INPUT_PLACEHOLDER: &str =
	"Type something... (@ for files, ESC to quit)";
pub const PROMPT_CHAR: &str = "\u{276f} ";

// -- Model switching --
pub const MODEL_STATUS_PREFIX: &str = "Model: ";
pub const MODEL_INVALID: &str =
	"Invalid model. Use: haiku, sonnet, opus";

// -- Effort switching --
pub const EFFORT_STATUS_PREFIX: &str = "Effort: ";

// -- Slash command picker --
pub const PICKER_COMMANDS: &str = " Commands ";
pub const PICKER_MODEL_ARG: &str = " Model ";
pub const PICKER_EFFORT_ARG: &str = " Effort ";

// -- Claude streaming status --
pub const CLAUDE_NO_RESPONSE: &str =
	"Claude: no response received";
pub const CLAUDE_INTERRUPTED: &str =
	"Claude: stream interrupted";
pub const CLAUDE_STOPPED_PREFIX: &str =
	"Claude stopped: ";

// -- Jira board --
pub const JIRA_BOARD_TITLE: &str =
	" Jira Board ";
pub const JIRA_NO_SPRINT: &str =
	"No active sprint";
pub const JIRA_SP_LABEL: &str = "SP";
pub const JIRA_NO_TICKETS: &str =
	"No tickets found";

// -- Session --
pub const NEW_SESSION: &str =
	"New conversation started";

// -- Quit confirmation --
pub const QUIT_HINT: &str =
	"Press Esc again to quit";

// -- Tool errors --
pub const ERR_NO_BB_REPO: &str =
	"No BB repo found. Run scan.";
pub const ERR_NO_BB_CREDS: &str =
	"No BB credentials";
pub const ERR_NO_JIRA_CREDS: &str =
	"No Jira credentials";

// -- Tool fetch --
pub const TOOL_FETCH_DISCONNECTED: &str =
	"Fetch disconnected";

// -- Jira assignee filter --
pub const JIRA_ASSIGNEE_TITLE: &str =
	" Select Assignee ";
/// Prefix for the filter row in the ticket list
pub const JIRA_FILTER_ROW: &str =
	"\u{25b8} Filter by assignee: ";

// -- Enhance prompt --
pub const ENHANCE_WORKING: &str =
	"Enhancing prompt...";
pub const ENHANCE_EMPTY: &str =
	"Nothing to enhance";
pub const ENHANCE_BUSY: &str =
	"Enhancement in progress";
pub const ENHANCE_USAGE: &str =
	"Usage: /e-prompt <text>";
pub const ENHANCE_ERROR: &str =
	"Enhance error: ";

// -- Pre-flight panel --
pub const PREFLIGHT_TITLE: &str =
	" Pre-flight ";
pub const PREFLIGHT_FOOTER: &str =
	"[Enter] Send  [Esc] Cancel";
pub const LABEL_PROMPT: &str = "Prompt: ";
pub const LABEL_MODE: &str = "Mode:   ";
pub const LABEL_FILES: &str = "Files:  ";
pub const AGENT_OPEN: &str = "  agent(";
pub const AGENT_SEP: &str = "): ";

// -- Agent instructions --
pub const AGENT_INSTRUCTION_HEADER: &str =
	"PARALLEL SUBAGENTS: Execute these \
	 tasks as parallel subagents:";
pub const AGENT_INSTRUCTION_FOOTER: &str =
	"Each subagent runs independently. \
	 Report all results.";

// -- Agent suggestions --
pub const SUGGEST_PREFIX: &str = "Suggested:";
