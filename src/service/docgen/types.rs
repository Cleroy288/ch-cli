//! DTOs for the doc generation service.

/// Documentation generation status info
#[derive(Debug, Clone)]
pub struct DocStatusInfo {
	/// Total doc entries
	pub total: usize,
	/// Entries with completed docs
	pub completed: usize,
	/// Entries still pending
	pub pending: usize,
	/// Whether generation is active
	pub is_generating: bool,
	/// Whether all docs are ready
	pub is_ready: bool,
}
