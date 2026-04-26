mod doc_extraction;
mod doc_extraction_collect;
mod helpers;
mod parser_methods;
mod reference_processing;
mod rust_parser;
mod rust_parser_io;
mod symbol_building;
mod symbol_building_node;
mod symbol_processing;
mod ts_parser;
mod types;

// Re-export public types
pub use helpers::{is_rust_keyword, parse_visibility};
pub use rust_parser::RustParser;
pub use ts_parser::TsParser;
pub use types::{ExtractedReference, ParseError, Result};
