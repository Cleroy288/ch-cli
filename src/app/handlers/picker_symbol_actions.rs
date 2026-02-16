//! Symbol picker activation and navigation.
//!
//! Handles opening the symbol picker, drilling into
//! container symbols, and cancelling selection.

use std::sync::mpsc;
use std::thread;

use crate::app::App;
use crate::indexer::parser::RustParser;
use crate::indexer::symbols::Symbol;
use crate::ui::strings::tui_labels::STATUS_NO_SYMBOLS;

impl App {
    /// Open symbol picker for the last file reference.
    ///
    /// Called when `(` is typed after a file reference.
    /// Parses the file and enters Symbols mode.
    /// Sets a status message if the file has no symbols.
    pub(crate) fn activate_symbol_picker(
        &mut self,
        file_ref_idx: usize,
    ) {
        let file_ref =
            self.file_references.get(file_ref_idx);
        let Some(file_ref) = file_ref else { return };

        let path = file_ref.full_path.as_string();
        if !path.ends_with(".rs") {
            return;
        }

        let symbols = parse_file_symbols(&path);
        if symbols.is_empty() {
            self.status_message =
                Some(STATUS_NO_SYMBOLS.to_string());
            return;
        }

        let trigger = self.cursor_position.get();
        let file_path =
            std::path::PathBuf::from(&path);
        self.picker.activate_symbols(
            trigger, file_path, symbols,
        );
        self.spawn_doc_names_fetch(&path);
    }

    /// Drill into a container symbol's children
    pub(crate) fn drill_into_symbol(
        &mut self,
        name: String,
    ) {
        if let Some(browser) =
            self.picker.symbol_browser_mut()
        {
            browser.drill_into(name);
        }
        self.picker.clear_query();
    }

    /// Spawn background thread to fetch doc names.
    ///
    /// Loads DocStore from disk and extracts names of
    /// documented symbols for the given absolute path.
    fn spawn_doc_names_fetch(&mut self, path: &str) {
        let project = self.project_path.clone();
        let abs_path = path.to_string();
        let (tx, rx) = mpsc::channel();
        self.doc_names_rx = Some(rx);

        thread::spawn(move || {
            let names =
                load_doc_names(&project, &abs_path);
            let _ = tx.send(names);
        });
    }

    /// Cancel symbol picker, remove `(` from input
    pub(crate) fn cancel_symbol_picker(&mut self) {
        // Remove the `(` character we inserted
        let cursor = self.cursor_position.get();
        if cursor > 0 {
            let prev_char =
                self.input.chars().nth(cursor - 1);
            if prev_char == Some('(') {
                self.input.remove(cursor - 1);
                self.cursor_position.move_left();
            }
        }
        self.picker.deactivate();
    }
}

/// Parse a Rust file and return its symbols.
///
/// Returns empty Vec on any error (parser init, file
/// read, or parse failure).
fn parse_file_symbols(path: &str) -> Vec<Symbol> {
    let Ok(mut parser) = RustParser::new() else {
        return Vec::new();
    };
    parser.parse_file(path).unwrap_or_default()
}

/// Load documented symbol names for a file from disk.
///
/// Reads DocStore and returns names of Ready entries
/// matching the given absolute file path.
fn load_doc_names(
    project: &str,
    abs_path: &str,
) -> std::collections::HashSet<String> {
    use std::path::Path;
    use crate::retrieval::docgen::DocStore;

    let project_path = Path::new(project);
    let file_path = Path::new(abs_path);
    let Ok(store) = DocStore::load(project_path)
    else {
        return std::collections::HashSet::new();
    };
    store
        .get_by_file(file_path)
        .into_iter()
        .filter(|e| e.is_ready())
        .map(|e| e.name.clone())
        .collect()
}
