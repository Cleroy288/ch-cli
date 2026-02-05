// UI Layout Constants
/// Height of the ASCII art title box in lines
pub const TITLE_BOX_HEIGHT: u16 = 7;

/// Height of the input box in lines
pub const INPUT_BOX_HEIGHT: u16 = 3;

/// Number of messages to display in the debug panel history
pub const HISTORY_DISPLAY_COUNT: usize = 10;

/// Maximum height of the picker overlay in lines
pub const MAX_PICKER_HEIGHT: u16 = 15;

/// Width of the type chooser picker in characters
pub const PICKER_WIDTH: u16 = 40;

/// Number of type options in the picker (folder, file)
pub const PICKER_TYPE_OPTIONS: usize = 2;

// File System Constants
/// Maximum recursion depth for directory scanning
pub const MAX_RECURSION_DEPTH: usize = 10;

/// Visual symbol for directories in the UI
pub const DIR_SYMBOL: &str = "▸";

/// Visual symbol for files in the UI
pub const FILE_SYMBOL: &str = "◆";

// Conversation History Constants
/// Maximum number of messages to keep in conversation history
pub const DEFAULT_MAX_MESSAGES: usize = 100;
