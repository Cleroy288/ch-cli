use crate::domain::query_intent::QueryIntent;

use super::document_type::DocumentType;

/// Keywords indicating a query wants code, not docs
const CODE_KEYWORDS: &[&str] = &[
	"implementation", "algorithm", "logic", "code",
	"function", "method", "work", "works", "working",
	"implement", "source", "actual",
];

impl DocumentType {
	pub fn boost_factor(&self) -> f32 {
		match self {
			Self::SourceCode => 1.5,
			Self::Documentation => 1.0,
			Self::Test => 0.8,
			Self::Notes => 0.7,
			Self::Benchmark => 0.3,
		}
	}

	/// Lowers documentation boost for implementation queries.
	pub fn boost_factor_for_query(
		&self,
		query: &str,
	) -> f32 {
		let base = self.boost_factor();
		if !query_wants_code(query) {
			return base;
		}
		match self {
			Self::Documentation | Self::Notes
			| Self::Benchmark => base * 0.5,
			Self::SourceCode | Self::Test => base,
		}
	}

	/// For Understand intent, reduce doc/test boost.
	pub fn boost_factor_for_intent(
		&self,
		intent: &QueryIntent,
	) -> f32 {
		let base = self.boost_factor();
		let multiplier = intent_multiplier(self, intent);
		base * multiplier
	}
}

/// Intent-based multiplier for document type boost.
fn intent_multiplier(
	doc: &DocumentType,
	intent: &QueryIntent,
) -> f32 {
	match (intent, doc) {
		(
			QueryIntent::Understand,
			DocumentType::SourceCode,
		) => 3.0,
		(
			QueryIntent::Understand,
			DocumentType::Documentation
			| DocumentType::Notes,
		) => 0.1,
		(
			QueryIntent::Understand,
			DocumentType::Benchmark,
		) => 0.05,
		(
			QueryIntent::Understand,
			DocumentType::Test,
		) => 0.1,
		(
			QueryIntent::FindDefinition,
			DocumentType::SourceCode,
		) => 2.0,
		(
			QueryIntent::FindDefinition,
			DocumentType::Documentation,
		) => 0.3,
		(
			QueryIntent::FindDefinition,
			DocumentType::Notes,
		) => 0.2,
		(
			QueryIntent::FindDefinition,
			DocumentType::Benchmark,
		) => 0.1,
		(
			QueryIntent::FindDefinition,
			DocumentType::Test,
		) => 0.2,
		_ => 1.0,
	}
}

fn query_wants_code(query: &str) -> bool {
	let lower = query.to_lowercase();
	CODE_KEYWORDS
		.iter()
		.any(|keyword| lower.contains(keyword))
}
