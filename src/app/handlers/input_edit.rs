use crate::app::claude_request::spawn_claude_request;
use crate::app::handlers::symbol_resolver;
use crate::app::memory_save;
use crate::app::parser;
use crate::app::App;
use crate::message::user_message::UserMessage;
use crate::service::prompt_history_io;

impl App {
    pub(crate) fn handle_home(&mut self) {
        self.cursor_position.jump_to_start();
    }

    pub(crate) fn handle_end(&mut self) {
        self.cursor_position.jump_to_end(
            self.input.len(),
        );
    }

    pub(crate) fn handle_enter(&mut self) {
        if self.handle_slash_command() {
            self.clear_input_state();
            return;
        }
        if self.build_preflight() {
            return;
        }
        self.send_current_input();
    }

    /// Freeze inline blocks, parse, send, clear.
    pub(crate) fn send_current_input(&mut self) {
        if let Some(ib) =
            &mut self.inline_blocks
        {
            ib.freeze();
        }
        self.inline_blocks = None;
        if let Some(msg) = self.parse_message() {
            let root = std::path::PathBuf::from(
                &self.project_root,
            );
            self.prompt_history.add(&self.input);
            prompt_history_io::save(
                &self.prompt_history, &root,
            );
            self.send_and_store(msg);
        }
        self.clear_input_state();
    }

    /// Expand paste placeholders then parse
    /// input text into a UserMessage.
    fn parse_message(&self) -> Option<UserMessage> {
        let expanded = self.expand_paste_blocks();
        parser::parse_input_to_message(
            expanded,
            &self.file_references,
            &self.symbol_selectors,
        )
    }

    /// spawn Claude request, store in history.
    fn send_and_store(
        &mut self,
        mut msg: UserMessage,
    ) {
        symbol_resolver::resolve_symbols(
            &mut msg.segments,
        );
        self.pending_user_input = Some(
            memory_save::extract_user_input(
                &self.input, &msg.segments,
            ),
        );
        spawn_claude_request(
            self, &msg.segments,
        );
        self.history.add_message(msg);
    }

    pub(crate) fn clear_input_state(&mut self) {
        self.input.clear();
        self.cursor_position.jump_to_start();
        self.file_references.clear();
        self.symbol_selectors.clear();
        self.tool_references.clear();
        self.paste_blocks.clear();
        self.agent_suggestions.clear();
    }

    /// Retain only refs whose end <= input length
    pub(crate) fn update_file_references(
        &mut self,
    ) {
        let len = self.input.len();
        self.file_references
            .retain(|r| r.end <= len);
        self.symbol_selectors
            .retain(|s| s.end <= len);
        self.tool_references
            .retain(|t| t.end <= len);
    }

    pub(crate) fn insert_text_at_cursor(
        &mut self,
        text: &str,
    ) {
        for chr in text.chars() {
            let pos = self.cursor_position.get();
            self.input.insert(pos, chr);
            self.cursor_position.move_right(
                &self.input,
            );
        }
    }
}
