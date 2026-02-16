//! Lightweight markdown to ratatui Line renderer.
//!
//! Handles headings, code blocks, tables, lists,
//! blockquotes, horizontal rules, bold, italic,
//! and inline code — covering Claude CLI output.

mod blockquote;
mod blocks;
mod code_block;
mod heading;
pub mod inline;
mod list;
mod rule;
mod table;

pub use blocks::render_markdown;
