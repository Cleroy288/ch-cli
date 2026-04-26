use crate::domain::query_intent::QueryIntent;

use super::kind::SymbolKind;

impl SymbolKind {
	/// Functions/methods ranked higher than fields/constants
	pub fn boost_factor(&self) -> f32 {
		match self {
			// High priority: actual code structures
			SymbolKind::Function => 1.4,
			SymbolKind::Method => 1.4,
			SymbolKind::Struct => 1.3,
			SymbolKind::Enum => 1.3,
			SymbolKind::Trait => 1.3,
			SymbolKind::Impl => 1.2,
			// increased from 1.1 for module ranking
			SymbolKind::Module => 1.3,

			// Medium priority
			SymbolKind::Constant => 0.9,
			SymbolKind::Static => 0.9,
			SymbolKind::TypeAlias => 0.9,
			SymbolKind::Macro => 1.0,
			SymbolKind::EnumVariant => 0.8,

			// Lower priority
			SymbolKind::Field => 0.7,

			// Documentation symbols
			SymbolKind::DocumentChunk => 0.6,
		}
	}

	/// Multiplies base boost by an intent-specific factor.
	pub fn boost_factor_for_intent(
		&self,
		intent: &QueryIntent,
	) -> f32 {
		self.boost_factor()
			* intent_multiplier(self, intent)
	}
}

/// Intent-based multiplier for symbol kind boost.
/// Flattened (intent, kind) match for readability.
fn intent_multiplier(
	kind: &SymbolKind,
	intent: &QueryIntent,
) -> f32 {
	match (intent, kind) {
		// Understand: behavior over data
		(
			QueryIntent::Understand,
			SymbolKind::Function | SymbolKind::Method,
		) => 1.5,
		(
			QueryIntent::Understand,
			SymbolKind::Struct | SymbolKind::Impl,
		) => 1.2,
		(
			QueryIntent::Understand,
			SymbolKind::Field,
		) => 0.3,
		(
			QueryIntent::Understand,
			SymbolKind::DocumentChunk,
		) => 0.2,
		// FindDefinition: type/fn defs first
		(
			QueryIntent::FindDefinition,
			SymbolKind::Struct
			| SymbolKind::Enum
			| SymbolKind::Trait,
		) => 2.0,
		(
			QueryIntent::FindDefinition,
			SymbolKind::TypeAlias,
		) => 1.8,
		(
			QueryIntent::FindDefinition,
			SymbolKind::Function,
		) => 1.8,
		(
			QueryIntent::FindDefinition,
			SymbolKind::Method,
		) => 1.6,
		(
			QueryIntent::FindDefinition,
			SymbolKind::Module,
		) => 1.5,
		(
			QueryIntent::FindDefinition,
			SymbolKind::Impl,
		) => 0.5,
		(
			QueryIntent::FindDefinition,
			SymbolKind::DocumentChunk,
		) => 0.1,
		// Debug: functions/error types
		(
			QueryIntent::Debug,
			SymbolKind::Function | SymbolKind::Method,
		) => 1.3,
		(
			QueryIntent::Debug,
			SymbolKind::Struct | SymbolKind::Enum,
		) => 1.1,
		(QueryIntent::Debug, SymbolKind::Field) => 0.5,
		_ => 1.0,
	}
}
