//! DocStore entry management - CRUD and iteration.
//!
//! Provides upsert, get, remove, and iteration
//! methods for documentation entries.

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::store_core::DocStore;

/// Entry access and mutation operations.
impl DocStore {
	/// Add or update an entry.
	pub fn upsert(&mut self, entry: DocEntry) {
		self.entries.insert(entry.id.clone(), entry);
		self.update_completion_status();
	}

	/// Get entry by ID.
	pub fn get(
		&self, entry_id: &str,
	) -> Option<&DocEntry> {
		self.entries.get(entry_id)
	}

	/// Get mutable entry by ID.
	pub fn get_mut(
		&mut self,
		entry_id: &str,
	) -> Option<&mut DocEntry> {
		self.entries.get_mut(entry_id)
	}

	/// Get entry by symbol name (first match).
	pub fn get_by_name(
		&self,
		name: &str,
	) -> Option<&DocEntry> {
		self.entries
			.values()
			.find(|entry| entry.name == name)
	}

	/// Get all entries.
	pub fn all_entries(
		&self,
	) -> impl Iterator<Item = &DocEntry> {
		self.entries.values()
	}
}

/// Entry removal and iteration operations.
impl DocStore {
	/// Get all entries (mutable).
	pub fn all_entries_mut(
		&mut self,
	) -> impl Iterator<Item = &mut DocEntry> {
		self.entries.values_mut()
	}

	/// Remove entry by ID.
	pub fn remove(
		&mut self, entry_id: &str,
	) -> Option<DocEntry> {
		let entry = self.entries.remove(entry_id);
		self.update_completion_status();
		entry
	}

	/// Clear all entries.
	pub fn clear(&mut self) {
		self.entries.clear();
		self.generation_complete = false;
	}
}
