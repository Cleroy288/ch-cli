use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::SymbolBrowser;

impl App {
    /// Handle keys in Symbols mode.
    ///
    /// Enter on container → drill. Enter on leaf →
    /// finalize. Backspace → go to parent symbol.
    pub(crate) fn handle_symbols_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                let count = self.symbol_items_count();
                self.picker.move_down(count);
            }
            KeyCode::Enter => {
                self.handle_symbol_enter();
            }
            KeyCode::Esc => {
                self.cancel_symbol_picker();
            }
            KeyCode::Backspace => {
                self.handle_symbol_backspace();
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            _ => {}
        }
        false
    }

    /// Count of current visible symbol items
    fn symbol_items_count(&self) -> usize {
        let Some(browser) = self.picker.symbol_browser()
        else {
            return 0;
        };
        browser.current_items(self.picker.query()).len()
    }

    /// Handle Enter on a symbol item
    fn handle_symbol_enter(&mut self) {
        let query = self.picker.query().to_string();
        let idx = self.picker.selected_index();

        let (name, kind) = {
            let Some(browser) =
                self.picker.symbol_browser()
            else {
                return;
            };
            let items = browser.current_items(&query);
            let Some(sym) = items.get(idx) else {
                return;
            };
            (sym.name.clone(), sym.kind)
        };

        if SymbolBrowser::is_container(kind) {
            self.drill_into_symbol(name);
        } else {
            self.finalize_symbol(name);
        }
    }

    /// Backspace: go up or cancel symbol picker
    fn handle_symbol_backspace(&mut self) {
        if !self.picker.query().is_empty() {
            self.picker.pop_query();
            return;
        }
        let went_up = self
            .picker
            .symbol_browser_mut()
            .is_some_and(|browser| browser.go_up());
        if !went_up {
            self.cancel_symbol_picker();
        } else {
            self.picker.clear_query();
        }
    }
}
