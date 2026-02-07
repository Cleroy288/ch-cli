//! FastPathResult building

use super::super::fast_path_patterns::{
	FastPathIntent, FastPathResult, SymbolCandidate,
};

use super::intent::classify_intent;

/// Deduplicate symbols by name, keeping highest confidence
pub(crate) fn deduplicate_symbols(
	symbols: &mut Vec<SymbolCandidate>,
) {
	symbols.sort_by(|a, b| {
		b.confidence
			.partial_cmp(&a.confidence)
			.unwrap()
	});
	symbols.dedup_by(|a, b| a.name == b.name);
}

/// Build FastPathResult from extracted symbols
pub(crate) fn build_result(
	symbols: Vec<SymbolCandidate>,
	is_conceptual: bool,
) -> FastPathResult {
	let confidence = if symbols.is_empty() {
		0.0
	} else {
		let sum: f32 = symbols
			.iter()
			.map(|s| s.confidence)
			.sum();
		sum / symbols.len() as f32
	};

	let intent = classify_intent(
		is_conceptual,
		symbols.is_empty(),
	);
	let use_fast_path = intent == FastPathIntent::Explicit
		&& confidence >= 0.7;

	FastPathResult {
		symbols,
		confidence,
		intent,
		use_fast_path,
	}
}
