use serde::{Deserialize, Serialize};

/// The user's intent behind a search query.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
pub enum QueryIntent {
	FindDefinition,
	FindUsages,
	Understand,
	Modify,
	Debug,
	Search,
}
