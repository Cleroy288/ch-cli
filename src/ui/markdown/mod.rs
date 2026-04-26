mod blockquote;
mod blocks;
pub mod code_block;
pub mod code_metrics;
mod heading;
pub mod inline;
mod list;
mod rule;
pub mod sections;
mod syntax;
mod table;

pub use blocks::render_markdown;
