/// UI components module.
///
/// Each component is responsible for rendering a specific part of the UI:
/// - `title`: ASCII art title rendering
/// - `input`: Input box with styled file/folder references
/// - `debug`: Debug panel showing message history
/// - `picker`: File/folder picker overlay
///
/// All rendering functions are designed to be as pure as possible,
/// taking data and producing widgets without side effects.
pub mod debug;
pub mod input;
pub mod picker;
pub mod title;

// Re-export commonly used functions
pub use debug::render_debug_panel;
pub use input::{render_input, set_cursor};
pub use picker::render_picker;
pub use title::render_title;
