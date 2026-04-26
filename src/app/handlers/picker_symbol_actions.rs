use crate::app::App;
use crate::indexer::parser::{RustParser, TsParser};
use crate::indexer::symbols::Symbol;
use crate::ui::strings::tui_labels::STATUS_NO_SYMBOLS;

impl App {
    pub(crate) fn activate_symbol_picker(
        &mut self,
        file_ref_idx: usize,
    ) {
        let file_ref =
            self.file_references.get(file_ref_idx);
        let Some(file_ref) = file_ref else { return };

        let path = file_ref.full_path.to_string();
        if !has_parser_support(&path) {
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
    }

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

    pub(crate) fn cancel_symbol_picker(&mut self) {
        let pos = self.cursor_position.get();
        if pos > 0 {
            let mut tmp = self.cursor_position;
            tmp.move_left(&self.input);
            let prev = tmp.get();
            if self.input[prev..]
                .starts_with('(')
            {
                self.input.remove(prev);
                self.cursor_position.set(prev);
            }
        }
        self.picker.deactivate();
    }
}

pub fn has_parser_support(path: &str) -> bool {
    path.ends_with(".rs")
        || path.ends_with(".tsx")
        || (path.ends_with(".ts")
            && !path.ends_with(".d.ts"))
}

fn parse_file_symbols(path: &str) -> Vec<Symbol> {
    if path.ends_with(".rs") {
        return RustParser::new()
            .and_then(|mut p| p.parse_file(path))
            .unwrap_or_default();
    }
    if path.ends_with(".tsx") {
        return TsParser::tsx()
            .and_then(|mut p| p.parse_file(path))
            .unwrap_or_default();
    }
    if path.ends_with(".ts") {
        return TsParser::typescript()
            .and_then(|mut p| p.parse_file(path))
            .unwrap_or_default();
    }
    Vec::new()
}
