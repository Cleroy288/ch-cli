/// UI components — one per visual section.
pub mod debug;
pub(crate) mod debug_hit;
pub mod debug_render;
mod inline_bar;
mod inline_blocks;
mod inline_panel;
mod inline_panel_agent;
pub(crate) mod inline_panel_code;
pub(crate) mod inline_panel_hit;
pub(crate) mod inline_panel_scroll;
pub mod input;
mod input_prompt;
pub(crate) mod input_styling;
pub(crate) mod input_suggest;
pub mod picker;
mod preflight_lines;
mod preflight_panel;
pub(crate) mod spinner;
mod status_line;
mod title;

pub use debug::render_debug_panel;
pub use inline_panel::render_inline_panel;
pub use input::{
    compute_input_height, render_input, set_cursor,
};
pub use picker::render_picker;
pub use preflight_panel::render_preflight_panel;
pub use status_line::render_status_line;
pub use title::render_title;
