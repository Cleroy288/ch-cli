//! JSON serialization for StructuredOutput

use super::output::StructuredOutput;

impl StructuredOutput {
	/// Convert to JSON string
	pub fn to_json(
		&self,
	) -> Result<String, serde_json::Error> {
		serde_json::to_string_pretty(self)
	}
}
