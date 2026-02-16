/// UI components module.
///
/// Each component is responsible for rendering a UI part:
/// - `title`: ASCII art title rendering
/// - `input`: Input box with styled file/folder references
/// - `debug`: Debug panel showing message history
/// - `picker`: File/folder picker overlay
///
/// All rendering functions are designed to be pure,
/// taking data and producing widgets without side effects.
pub mod debug;
#[doc(hidden)]
pub mod debug_render;
pub mod input;
pub(crate) mod input_styling;
pub mod picker;
pub mod title;
#[doc(hidden)]
pub mod title_progress;

// Re-export commonly used functions
pub use debug::render_debug_panel;
pub use input::{render_input, set_cursor};
pub use picker::render_picker;
pub use title::render_title;
