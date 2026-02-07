//! DocGen service — documentation generation
//! use cases.

mod default;
mod helpers;
pub mod types;

pub use default::DefaultDocGenService;

use std::path::Path;

use crate::domain::errors::docgen::DocGenError;
use crate::retrieval::docgen::DocEntry;

/// Service trait for doc generation operations
pub trait DocGenService {
	/// Start doc generation for a project
	fn start_generation(
		&self,
		path: &Path,
		force: bool,
	) -> Result<(), DocGenError>;

	/// Get generation status for a project
	fn get_status(
		&self,
		path: &Path,
	) -> Result<types::DocStatusInfo, DocGenError>;

	/// Get doc for a specific symbol
	fn get_doc(
		&self,
		path: &Path,
		symbol: &str,
	) -> Result<Option<DocEntry>, DocGenError>;

	/// Search generated docs
	fn search_docs(
		&self,
		path: &Path,
		query: &str,
		limit: usize,
	) -> Result<Vec<DocEntry>, DocGenError>;
}
