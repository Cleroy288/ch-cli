use super::mode::PickerMode;
use super::state::Picker;
use super::symbol_browser::SymbolBrowser;

impl Picker {
    /// Check if the picker is active
    pub fn is_active(&self) -> bool {
        !matches!(self.mode, PickerMode::Inactive)
    }

    /// Get the current mode
    pub fn mode(&self) -> &PickerMode {
        &self.mode
    }

    /// Get the search query
    pub fn query(&self) -> &str {
        self.query.query()
    }

    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.query.selected_index()
    }

    /// Get the trigger position
    pub fn trigger_position(&self) -> usize {
        self.trigger_position
    }

    /// Get the symbol browser (if in Symbols mode)
    pub fn symbol_browser(
        &self,
    ) -> Option<&SymbolBrowser> {
        self.symbol_browser.as_ref()
    }

    /// Get mutable symbol browser
    pub fn symbol_browser_mut(
        &mut self,
    ) -> Option<&mut SymbolBrowser> {
        self.symbol_browser.as_mut()
    }
}
