use crate::app::App;
use crate::domain::backend_kind::BackendKind;
use crate::service::claude::model;
use crate::service::memory::id_gen;
use crate::ui::strings::tui_labels;

const CMD_NEW: &str = "/new";
const CMD_MODEL: &str = "/model";
const CMD_MODEL_PREFIX: &str = "/model ";
const CMD_EFFORT: &str = "/effort";
const CMD_EFFORT_PREFIX: &str = "/effort ";
const CMD_ENHANCE: &str = "/e-prompt";
const CMD_ENHANCE_PREFIX: &str = "/e-prompt ";
const CMD_BACKEND: &str = "/backend";
const CMD_BACKEND_PREFIX: &str = "/backend ";
const CMD_GIT: &str = "/git";

impl App {
    /// Handle slash commands.
    /// Returns true if a command was handled.
    pub(crate) fn handle_slash_command(
        &mut self,
    ) -> bool {
        let trimmed = self.input.trim();
        if trimmed == CMD_NEW {
            return self.handle_cmd_new();
        }
        if trimmed == CMD_MODEL
            || trimmed.starts_with(CMD_MODEL_PREFIX)
        {
            let owned = trimmed.to_string();
            return self.handle_cmd_model(&owned);
        }
        if trimmed == CMD_EFFORT
            || trimmed.starts_with(CMD_EFFORT_PREFIX)
        {
            let owned = trimmed.to_string();
            return self.handle_cmd_effort(&owned);
        }
        if trimmed == CMD_BACKEND
            || trimmed.starts_with(CMD_BACKEND_PREFIX)
        {
            let owned = trimmed.to_string();
            return self.handle_cmd_backend(&owned);
        }
        if trimmed == CMD_GIT {
            self.handle_git_command();
            return true;
        }
        if trimmed == CMD_ENHANCE {
            self.set_status_message(Some(
                tui_labels::ENHANCE_USAGE.into(),
            ));
            return true;
        }
        if trimmed.starts_with(CMD_ENHANCE_PREFIX) {
            let text = trimmed
                .strip_prefix(CMD_ENHANCE_PREFIX)
                .unwrap_or("");
            let owned = text.to_string();
            self.start_enhance_prefix(&owned);
            return true;
        }
        false
    }

    fn handle_cmd_new(&mut self) -> bool {
        self.claude_session_id = None;
        self.last_claude_response = None;
        self.memory_session_id =
            id_gen::new_session_id();
        self.pending_user_input = None;
        self.set_status_message(Some(
            tui_labels::NEW_SESSION.to_string(),
        ));
        true
    }

    fn handle_cmd_model(
        &mut self,
        trimmed: &str,
    ) -> bool {
        if trimmed == CMD_MODEL {
            let msg = format!(
                "{}{}",
                tui_labels::MODEL_STATUS_PREFIX,
                self.model_name,
            );
            self.set_status_message(Some(msg));
        } else {
            self.apply_model_arg(trimmed);
        }
        true
    }

    /// Switch model + reset effort to model default.
    fn apply_model_arg(&mut self, trimmed: &str) {
        let name = trimmed
            .strip_prefix(CMD_MODEL_PREFIX)
            .unwrap_or("")
            .trim();
        let valid = self.backend.valid_models();
        if !valid.contains(&name) {
            self.set_status_message(Some(
                tui_labels::MODEL_INVALID.to_string(),
            ));
            return;
        }
        self.model_name = name.to_string();
        model::save_model(
            &self.project_root, name,
        );
        let effort = self.backend
            .default_effort_for_model(name);
        self.effort_level = effort.to_string();
        model::save_effort(
            &self.project_root, effort,
        );
        let msg = format!(
            "{}{} (effort: {})",
            tui_labels::MODEL_STATUS_PREFIX,
            name,
            effort,
        );
        self.set_status_message(Some(msg));
    }

    fn handle_cmd_effort(
        &mut self,
        trimmed: &str,
    ) -> bool {
        if trimmed == CMD_EFFORT {
            let msg = format!(
                "{}{}",
                tui_labels::EFFORT_STATUS_PREFIX,
                self.effort_level,
            );
            self.set_status_message(Some(msg));
        } else {
            self.apply_effort_arg(trimmed);
        }
        true
    }

    /// Validate effort against current model.
    fn apply_effort_arg(
        &mut self,
        trimmed: &str,
    ) {
        let level = trimmed
            .strip_prefix(CMD_EFFORT_PREFIX)
            .unwrap_or("")
            .trim();
        let allowed = self.backend
            .valid_efforts(&self.model_name);
        if allowed.is_empty() {
            self.set_status_message(Some(format!(
                "Effort not supported on {}",
                self.model_name,
            )));
            return;
        }
        if !allowed.contains(&level) {
            let msg = format!(
                "'{}' not valid for {} ({})",
                level,
                self.model_name,
                allowed.join(", "),
            );
            self.set_status_message(Some(msg));
            return;
        }
        self.effort_level = level.to_string();
        model::save_effort(
            &self.project_root, level,
        );
        let msg = format!(
            "{}{}",
            tui_labels::EFFORT_STATUS_PREFIX,
            level,
        );
        self.set_status_message(Some(msg));
    }

    fn handle_cmd_backend(
        &mut self,
        trimmed: &str,
    ) -> bool {
        if trimmed == CMD_BACKEND {
            let msg = format!(
                "Backend: {}",
                self.backend.name(),
            );
            self.set_status_message(Some(msg));
        } else {
            self.apply_backend_arg(trimmed);
        }
        true
    }

    /// Switch backend + reset model + effort.
    fn apply_backend_arg(
        &mut self,
        trimmed: &str,
    ) {
        let name = trimmed
            .strip_prefix(CMD_BACKEND_PREFIX)
            .unwrap_or("")
            .trim();
        let kind = match name {
            "claude" => Some(BackendKind::ClaudeCode),
            "gemini" => Some(BackendKind::GeminiCli),
            _ => None,
        };
        let Some(kind) = kind else {
            self.set_status_message(Some(
                "Invalid backend (claude, gemini)"
                    .to_string(),
            ));
            return;
        };
        let backend =
            crate::service::backend::backend_for_kind(
                kind,
            );
        self.set_backend(backend);
        let default_model =
            self.backend.default_model();
        self.model_name =
            default_model.to_string();
        self.effort_level = self.backend
            .default_effort_for_model(default_model)
            .to_string();
        self.claude_session_id = None;
        model::save_backend(
            &self.project_root, kind,
        );
        let msg = format!(
            "Backend: {} (model: {}, effort: {})",
            self.backend.name(),
            self.model_name,
            self.effort_level,
        );
        self.set_status_message(Some(msg));
    }
}
