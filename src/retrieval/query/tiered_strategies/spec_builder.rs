//! SearchSpec building from fast-path symbols

use crate::retrieval::daemon::protocol::SearchSpec;

use super::intent::detect_intent;

/// Build SearchSpec from fast-path symbols
#[doc(hidden)]
pub fn build_fast_path_spec_impl(
	query: &str,
	symbols: Vec<String>,
) -> SearchSpec {
	let intent = detect_intent(query);

	SearchSpec {
		original_query: query.to_string(),
		symbol_names: symbols,
		intent,
		file_filters: Vec::new(),
		context_hints: Vec::new(),
	}
}
