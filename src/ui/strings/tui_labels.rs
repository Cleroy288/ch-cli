//! TUI component labels and headers.

// -- Navigation help --
pub const NAV_UP_DOWN: &str = "\u{2191}\u{2193}";
pub const NAV_NAVIGATE: &str = "navigate";
pub const NAV_ENTER: &str = "Enter";
pub const NAV_SELECT: &str = "select";
pub const NAV_F5: &str = "F5";
pub const NAV_REFRESH: &str = "refresh";
pub const NAV_ESC: &str = "ESC";
pub const NAV_CANCEL: &str = "cancel";

// -- Picker titles --
pub const PICKER_FILES: &str = " Files ";
pub const PICKER_FOLDERS: &str = " Folders ";
pub const PICKER_SEARCH: &str = " Search ";

// -- Goodbye --
pub const GOODBYE_APP_NAME: &str = "rustean";
pub const GOODBYE_SEE_YOU: &str =
	"See you next time!";

// -- Debug panel --
pub const MSG_HISTORY: &str = "Message History";

// -- Doc generation progress --
pub const DOC_PROGRESS_FMT: &str = " Docs: {}% ";
pub const DOC_PROGRESS_DONE: &str = " Docs: done ";

// -- Status messages --
pub const STATUS_NO_SYMBOLS: &str =
	" No symbols available ";

// -- Output panel titles --
pub const PANEL_OUTPUT: &str = " output ";
pub const PANEL_DOC_PREVIEW: &str = " doc preview ";
pub const PANEL_CLAUDE: &str = " claude ";

// -- Doc loading --
pub const DOC_LOADING: &str =
	"Loading documentation...";

// -- Doc preview panel --
pub const DOC_NO_PREVIEW: &str =
	"No documentation available for this symbol.";
pub const DOC_DEPENDS_ON: &str = "Depends on";
pub const DOC_USED_BY: &str = "Used by";
pub const DOC_NONE_MARKER: &str = "\u{2014}";

// -- Tools picker --
pub const PICKER_TOOLS: &str = " Tools ";
pub const TOOL_DOC_BROWSER: &str = "Doc Browser";
pub const TOOL_DOC_BROWSER_DESC: &str =
	"Browse generated documentation";

// -- Input loading --
pub const INPUT_WAITING: &str =
	"Waiting for Claude...";

// -- Doc browser picker --
pub const PICKER_DOC_BROWSER: &str = " Doc Browser ";
pub const DOC_BROWSER_NO_DOCS: &str =
	"No documentation found. Run docs generate first.";
