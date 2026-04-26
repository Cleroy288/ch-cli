/// Inline editable code blocks with per-block
/// TextArea, question input, and answer.
mod focus;
pub mod getters;
pub mod implement;
pub mod questions;
pub mod scroll;
pub mod state;

pub use state::{
	BlockMode, InlineBlock, InlineBlocks,
	InlineFocus,
};
