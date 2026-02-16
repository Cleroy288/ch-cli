use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::PickerMode;

impl App {
    /// Handle picker-specific keyboard events.
    ///
    /// Routes to Browse or Symbols handler based on
    /// current picker mode.
    /// Returns true if the app should quit.
    pub(crate) fn handle_picker_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match self.picker.mode() {
            PickerMode::Inactive => false,
            PickerMode::Browse { .. } => {
                self.handle_browse_key(key)
            }
            PickerMode::Symbols { .. } => {
                self.handle_symbols_key(key)
            }
            PickerMode::Tools => {
                self.handle_tools_key(key)
            }
            PickerMode::DocBrowser => {
                self.handle_doc_browser_key(key)
            }
        }
    }
}
