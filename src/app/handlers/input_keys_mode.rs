use crossterm::event::KeyModifiers;

use crate::app::App;
use crate::domain::review_path::extract_review_blocks;
use crate::review::InlineBlocks;

impl App {
    /// Home: line start. Ctrl+Home: input start.
    pub(super) fn handle_home_key(
        &mut self,
        mods: KeyModifiers,
    ) {
        if mods.contains(KeyModifiers::CONTROL) {
            self.handle_home();
        } else {
            self.cursor_position
                .jump_to_line_start(&self.input);
        }
    }

    /// End: line end. Ctrl+End: input end.
    pub(super) fn handle_end_key(
        &mut self,
        mods: KeyModifiers,
    ) {
        if mods.contains(KeyModifiers::CONTROL) {
            self.handle_end();
        } else {
            self.cursor_position
                .jump_to_line_end(&self.input);
        }
    }

    pub(super) fn handle_char_input(
        &mut self,
        chr: char,
    ) {
        let pos = self.cursor_position.get();
        self.input.insert(pos, chr);
        self.cursor_position.move_right(
            &self.input,
        );
        match chr {
            '@' => self.picker.activate(pos),
            '#' => self.picker.activate_tools(pos),
            '/' if is_line_start(&self.input, pos) => {
                self.picker.activate_slash(pos);
            }
            '(' => self.try_open_symbol_picker(),
            ' ' => self.update_agent_suggestions(),
            _ => {}
        }
    }

    /// Ctrl+R: enter review mode for last response.
    pub(super) fn activate_review_mode(&mut self) {
        let resp = match &self.last_claude_response {
            Some(r) => r,
            None => return,
        };
        let blocks =
            extract_review_blocks(&resp.result);
        if blocks.is_empty() {
            return;
        }
        self.inline_blocks =
            Some(InlineBlocks::activate(
                blocks,
                resp.result.clone(),
            ));
    }

}

/// "/" triggers slash picker at word boundary
fn is_line_start(input: &str, pos: usize) -> bool {
    if pos == 0 {
        return true;
    }
    let prev = input.as_bytes().get(pos - 1);
    matches!(prev, Some(b'\n' | b' '))
}

impl App {
    /// `(` typed after a file reference => symbols.
    pub(crate) fn try_open_symbol_picker(
        &mut self,
    ) {
        let cursor = self.cursor_position.get();
        let Some(paren_pos) =
            cursor.checked_sub(1)
        else {
            return;
        };
        let ref_idx = self
            .file_references
            .iter()
            .position(|fref| {
                fref.end == paren_pos
                    && !fref.is_dir
            });
        if let Some(idx) = ref_idx {
            self.activate_symbol_picker(idx);
        }
    }
}
