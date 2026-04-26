/// Options for project indexing
#[derive(Debug, Clone, Default)]
pub struct IndexOptions {
	pub verbose: bool,
	pub persistence: bool,
}

impl IndexOptions {
	/// Preset: persistence enabled, verbose off
	pub fn persistent() -> Self {
		Self {
			verbose: false,
			persistence: true,
		}
	}
}
