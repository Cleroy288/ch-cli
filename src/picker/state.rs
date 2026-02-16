use crate::fs::FileCache;
use crate::picker::{
	PickerMode, PickerQuery, PickerScanner,
};
use crate::picker::doc_browser::DocBrowser;
use crate::picker::symbol_browser::SymbolBrowser;

/// File/folder/symbol picker state.
///
/// Manages the picker's current mode, search query,
/// file system scanner, selection state, and optional
/// browser for symbols or documentation entries.
pub struct Picker {
	/// Current picker mode
	pub(crate) mode: PickerMode,
	/// The position in the input where trigger typed
	pub(crate) trigger_position: usize,
	/// Query operations
	pub(crate) query: PickerQuery,
	/// Scanner operations
	pub(crate) scanner: PickerScanner,
	/// Symbol browser (active in Symbols mode)
	pub(crate) symbol_browser: Option<SymbolBrowser>,
	/// Doc browser (active in DocBrowser mode)
	pub(crate) doc_browser: Option<DocBrowser>,
}

impl Picker {
	/// Create a new Picker with a shared file cache
	pub fn new(cache: FileCache) -> Self {
		Self {
			mode: PickerMode::Inactive,
			trigger_position: 0,
			query: PickerQuery::new(),
			scanner: PickerScanner::new(cache),
			symbol_browser: None,
			doc_browser: None,
		}
	}

	/// Sync file cache if watcher updated it
	pub fn sync_cache(&mut self) {
		self.scanner.sync_if_dirty();
	}
}

impl Default for Picker {
	fn default() -> Self {
		let cache = FileCache::empty();
		Self::new(cache)
	}
}
