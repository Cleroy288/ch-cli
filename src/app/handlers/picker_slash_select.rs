use crate::app::App;
use crate::picker::mode::PickerMode;
use crate::picker::slash_items::{
	self, DisplayItem, ItemKind,
};

impl App {
    /// Select the current slash item on Enter
    pub(super) fn select_slash_item(&mut self) {
        let query = self.picker.query().to_string();
        let items = slash_items::all_display_items(
            self.picker.mode(),
            &query,
            &self.skills,
        );
        let idx = self.picker.selected_index();
        let Some(item) = items.get(idx) else {
            return;
        };
        if item.kind == ItemKind::Skill {
            self.apply_skill(&item.name);
            return;
        }
        let arg_cmd = match self.picker.mode() {
            PickerMode::SlashArg { command } => {
                Some(command.clone())
            }
            _ => None,
        };
        if let Some(cmd) = arg_cmd {
            self.apply_arg(&cmd, &item.name);
        } else {
            self.apply_builtin(item);
        }
    }

    fn apply_skill(&mut self, name: &str) {
        let text = format!("/{name} ");
        self.set_input_text(&text);
        self.picker.deactivate();
    }

    fn apply_builtin(
        &mut self,
        item: &DisplayItem,
    ) {
        match item.name.as_str() {
            "model" => {
                self.set_input_text("/model ");
                self.picker
                    .activate_slash_arg("model");
            }
            "effort" => {
                self.set_input_text("/effort ");
                self.picker
                    .activate_slash_arg("effort");
            }
            "e-prompt" => {
                self.set_input_text("/e-prompt ");
                self.picker.deactivate();
            }
            "agent" => {
                self.insert_agent_line();
                self.picker.deactivate();
            }
            "git" => {
                self.picker.deactivate();
                self.set_input_text("");
                self.handle_git_command();
            }
            "Q" | "A" | "P" => {
                let text =
                    format!("/{} ", item.name);
                self.set_input_text(&text);
                self.picker.deactivate();
            }
            _ => {
                let text =
                    format!("/{}", item.name);
                self.set_input_text(&text);
                self.picker.deactivate();
            }
        }
    }

    fn apply_arg(
        &mut self,
        cmd: &str,
        name: &str,
    ) {
        let text = format!("/{cmd} {name}");
        self.set_input_text(&text);
        self.picker.deactivate();
    }

    /// Replace input text and move cursor to end
    fn set_input_text(&mut self, text: &str) {
        self.input.clear();
        self.input.push_str(text);
        self.cursor_position.set(self.input.len());
    }

    /// Append `\n- agent(sonnet): ` to the input.
    fn insert_agent_line(&mut self) {
        self.input.push_str(
            "\n- agent(sonnet): ",
        );
        self.cursor_position.set(
            self.input.len(),
        );
    }
}
