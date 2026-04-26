pub const TITLE_BOX_HEIGHT: u16 = 1;
pub const STATUS_LINE_HEIGHT: u16 = 1;
pub const HISTORY_DISPLAY_COUNT: usize = 10;
pub const PICKER_WIDTH: u16 = 40;

pub const MAX_RECURSION_DEPTH: usize = 10;
pub const DIR_SYMBOL: &str = "\u{25b8}";
pub const FILE_SYMBOL: &str = "\u{25c6}";

pub const DEFAULT_MAX_MESSAGES: usize = 100;

pub const PROMPT_WIDTH: u16 = 2;
/// top sep + 1 text line + bottom sep
pub const MIN_INPUT_HEIGHT: u16 = 3;
/// input area capped at 1/MAX_INPUT_FRAC of screen
pub const MAX_INPUT_FRAC: u16 = 3;
