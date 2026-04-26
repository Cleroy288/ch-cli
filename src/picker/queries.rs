/// Query and selection state for the Picker.
pub struct PickerQuery {
    pub(super) query: String,
    pub(super) selected_index: usize,
}

impl PickerQuery {
    pub fn new() -> Self {
        Self {
            query: String::new(),
            selected_index: 0,
        }
    }

    pub fn query(&self) -> &str {
        &self.query
    }

    pub fn push(&mut self, chr: char) {
        self.query.push(chr);
        self.selected_index = 0;
    }

    pub fn pop(&mut self) {
        self.query.pop();
        self.selected_index = 0;
    }

    pub fn clear(&mut self) {
        self.query.clear();
        self.selected_index = 0;
    }

    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    pub fn move_up(&mut self) {
        self.selected_index =
            self.selected_index.saturating_sub(1);
    }

    pub fn move_down(&mut self, max_items: usize) {
        if self.selected_index + 1 < max_items {
            self.selected_index += 1;
        }
    }

    pub fn set_selected_index(&mut self, idx: usize) {
        self.selected_index = idx;
    }
}

impl Default for PickerQuery {
    fn default() -> Self {
        Self::new()
    }
}
