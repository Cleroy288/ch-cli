use crate::picker::{
	PickerMode, PickerQuery, PickerScanner,
};

/// File/folder picker state.
///
/// Manages the picker's current mode, search query,
/// file system scanner, and selection state.
pub struct Picker {
    /// Current picker mode
    pub(crate) mode: PickerMode,
    /// The position in the input where @ was typed
    pub(crate) trigger_position: usize,
    /// Query operations
    pub(crate) query: PickerQuery,
    /// Scanner operations
    pub(crate) scanner: PickerScanner,
}

impl Picker {
    /// Create a new Picker
    pub fn new() -> Self {
        Self {
            mode: PickerMode::Inactive,
            trigger_position: 0,
            query: PickerQuery::new(),
            scanner: PickerScanner::new(),
        }
    }
}

impl Default for Picker {
    fn default() -> Self {
        Self::new()
    }
}
