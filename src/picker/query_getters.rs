use super::queries::PickerQuery;

impl PickerQuery {
    /// Get the search query
    pub fn query(&self) -> &str {
        &self.query
    }
}

impl Default for PickerQuery {
    fn default() -> Self {
        Self::new()
    }
}
