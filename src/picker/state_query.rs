use super::state::Picker;

impl Picker {
    /// Add a character to the search query
    pub fn push_query(&mut self, c: char) {
        self.query.push(c);
    }

    /// Remove the last character from the query
    pub fn pop_query(&mut self) {
        self.query.pop();
    }

    /// Clear the query
    pub fn clear_query(&mut self) {
        self.query.clear();
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        self.query.move_up();
    }

    /// Move selection down
    pub fn move_down(&mut self, max_items: usize) {
        self.query.move_down(max_items);
    }
}
