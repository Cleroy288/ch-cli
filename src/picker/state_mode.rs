use std::path::PathBuf;

use crate::indexer::symbols::Symbol;

use super::mode::PickerMode;
use super::state::Picker;
use super::symbol_browser::SymbolBrowser;

impl Picker {
    /// Activate picker in Browse mode at project root
    pub fn activate(&mut self, position: usize) {
        self.mode = PickerMode::Browse {
            dir: PathBuf::from("."),
        };
        self.query.clear();
        self.trigger_position = position;
        self.symbol_browser = None;
    }

    /// Deactivate the picker
    pub fn deactivate(&mut self) {
        self.mode = PickerMode::Inactive;
        self.query.clear();
        self.symbol_browser = None;
        self.doc_browser = None;
    }

    /// Enter symbols mode for a file
    pub fn activate_symbols(
        &mut self,
        position: usize,
        file_path: PathBuf,
        symbols: Vec<Symbol>,
    ) {
        let browser = SymbolBrowser::new(
            file_path.clone(),
            symbols,
        );
        self.mode = PickerMode::Symbols {
            file_path,
            parent: None,
        };
        self.query.clear();
        self.trigger_position = position;
        self.symbol_browser = Some(browser);
    }

    /// Browse into a subdirectory
    pub fn browse_into(&mut self, dir: PathBuf) {
        self.mode = PickerMode::Browse { dir };
        self.query.clear();
    }

    /// Check if picker is in Symbols mode
    pub fn is_symbol_mode(&self) -> bool {
        matches!(
            self.mode,
            PickerMode::Symbols { .. }
        )
    }
}
