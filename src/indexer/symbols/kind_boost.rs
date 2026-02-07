//! Boost factor calculations for symbol kinds based on query intent.

use crate::retrieval::daemon::protocol::QueryIntent;

use super::kind::SymbolKind;

impl SymbolKind {
	/// Get boost factor for ranking
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

	/// Get boost factor adjusted for query intent
	/// Boosts functions for understanding queries, deprioritizes fields
	pub fn boost_factor_for_intent(&self, intent: &QueryIntent) -> f32 {
		let base_boost = self.boost_factor(); // base symbol kind boost

		match intent {
			QueryIntent::Understand => {
				// For "how does X work" queries, prioritize behavior over data
				self.apply_understand_boost(base_boost)
			}
			QueryIntent::FindDefinition => {
				// For definition queries, prioritize type definitions
				self.apply_find_definition_boost(base_boost)
			}
			QueryIntent::Debug => {
				// For debugging queries, prioritize functions with error handling
				self.apply_debug_boost(base_boost)
			}
			_ => base_boost, // other intents use base boost
		}
	}

	/// Apply boost multiplier for Understand intent
	/// Boosts functions/methods (1.5x), deprioritizes fields/docs (0.3x)
	fn apply_understand_boost(&self, base_boost: f32) -> f32 {
		match self {
			// Functions/methods explain behavior - heavily boost them
			SymbolKind::Function | SymbolKind::Method => base_boost * 1.5,
			// Structs/impls can show structure - slightly boost
			SymbolKind::Struct | SymbolKind::Impl => base_boost * 1.2,
			// Fields are data, not behavior - deprioritize
			SymbolKind::Field => base_boost * 0.3,
			// Documentation chunks are docs, not code - heavily deprioritize
			SymbolKind::DocumentChunk => base_boost * 0.2,
			_ => base_boost,
		}
	}

	/// Apply boost multiplier for Debug intent
	/// Boosts functions/methods for debugging, deprioritizes fields
	fn apply_debug_boost(&self, base_boost: f32) -> f32 {
		match self {
			// Functions/methods likely contain error handling - boost
			SymbolKind::Function | SymbolKind::Method => base_boost * 1.3,
			// Structs that might be error types - slight boost
			SymbolKind::Struct | SymbolKind::Enum => base_boost * 1.1,
			// Fields less relevant for debugging - deprioritize
			SymbolKind::Field => base_boost * 0.5,
			_ => base_boost,
		}
	}

	/// Apply boost multiplier for FindDefinition intent
	/// Boosts struct/enum/trait definitions (2.0x), reduces docs (0.2x)
	fn apply_find_definition_boost(&self, base_boost: f32) -> f32 {
		match self {
			// Type definitions - strong boost
			SymbolKind::Struct => base_boost * 2.0,
			SymbolKind::Enum => base_boost * 2.0,
			SymbolKind::Trait => base_boost * 2.0,
			SymbolKind::TypeAlias => base_boost * 1.8,

			// Function definitions - strong boost
			// increased for definition priority
			SymbolKind::Function => base_boost * 1.8,
			SymbolKind::Method => base_boost * 1.6,
			// boost module definitions
			SymbolKind::Module => base_boost * 1.5,

			// Impl blocks are usages - reduce
			SymbolKind::Impl => base_boost * 0.5,

			// Docs are not definitions - heavily reduce
			SymbolKind::DocumentChunk => base_boost * 0.1,

			_ => base_boost,
		}
	}
}
