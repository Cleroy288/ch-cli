/// Query operations for the Picker.
///
/// Handles search query manipulation and result filtering.
pub struct PickerQuery {
    /// Search query for filtering
    pub(super) query: String,
    /// Currently selected index in the results
    pub(super) selected_index: usize,
}

impl PickerQuery {
    /// Create a new PickerQuery
    pub fn new() -> Self {
        Self {
            query: String::new(),
            selected_index: 0,
        }
    }

    /// Add a character to the search query
    pub fn push(&mut self, chr: char) {
        self.query.push(chr);
        self.reset_selection();
    }

    /// Remove the last character from the query
    pub fn pop(&mut self) {
        self.query.pop();
        self.reset_selection();
    }

    /// Clear the query
    pub fn clear(&mut self) {
        self.query.clear();
        self.reset_selection();
    }
}
