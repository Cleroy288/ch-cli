//! Symbol selection finalization.
//!
//! Handles building symbol info from the picker,
//! inserting text into input, and creating the
//! SymbolSelector that tracks the selection.

use crate::app::App;
use crate::domain::symbol_ref::build_symbol_path;
use crate::domain::{
    FileName, FilePath, InputSpan, SymbolSelector,
};
use std::path::PathBuf;

/// Symbol information extracted from browser
struct SymbolInfo {
    file_path_buf: PathBuf,
    file_name: String,
    symbol_path: String,
}

impl App {
    /// Finalize symbol selection.
    ///
    /// Appends "symbol_path)" to input, creates a
    /// SymbolSelector, and deactivates the picker.
    pub(crate) fn finalize_symbol(
        &mut self,
        leaf_name: String,
    ) {
        self.doc_preview = None;

        let symbol_info =
            self.build_symbol_info(&leaf_name);
        let Some(info) = symbol_info else {
            return;
        };

        let start = self.cursor_position.get();
        self.insert_symbol_text(&info.symbol_path);
        let end = self.cursor_position.get();

        let file_ref_start =
            self.get_file_ref_start(start);
        let selector = create_symbol_selector(
            file_ref_start,
            end,
            info,
        );
        self.symbol_selectors.push(selector);
        self.picker.deactivate();
    }

    /// Extract symbol info from picker browser
    fn build_symbol_info(
        &self,
        leaf_name: &str,
    ) -> Option<SymbolInfo> {
        let browser =
            self.picker.symbol_browser()?;
        let parents = browser.parent_stack();
        let symbol_path =
            build_symbol_path(parents, leaf_name);
        let file_path_buf =
            browser.file_path().clone();
        let file_name = file_path_buf
            .file_name()?
            .to_string_lossy()
            .to_string();
        Some(SymbolInfo {
            file_path_buf,
            file_name,
            symbol_path,
        })
    }

    /// Insert symbol path and closing paren
    fn insert_symbol_text(
        &mut self,
        symbol_path: &str,
    ) {
        let display = format!("{})", symbol_path);
        for chr in display.chars() {
            let pos = self.cursor_position.get();
            self.input.insert(pos, chr);
            self.cursor_position.move_right(
                self.input.len(),
            );
        }
    }

    /// Get file reference start position
    fn get_file_ref_start(
        &self,
        fallback: usize,
    ) -> usize {
        self.file_references
            .last()
            .map(|ref_item| ref_item.start)
            .unwrap_or(fallback)
    }
}

/// Create a SymbolSelector from parts (pure function)
fn create_symbol_selector(
    file_ref_start: usize,
    end: usize,
    info: SymbolInfo,
) -> SymbolSelector {
    let span = InputSpan {
        start: file_ref_start,
        end,
    };
    SymbolSelector::new(
        span,
        FilePath::from(
            info.file_path_buf
                .to_string_lossy()
                .to_string(),
        ),
        FileName::from(info.file_name),
        info.symbol_path.clone(),
    )
}
