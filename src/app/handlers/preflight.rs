/// Pre-flight confirmation panel logic.
///
/// When Enter is pressed and input has agent lines,
/// show a recap panel. Enter confirms, Esc cancels.

use crate::app::parser_agent;
use crate::app::App;
use crate::domain::preflight::PreFlightData;
use crate::service::claude::detect_query_mode;

impl App {
	/// Parse agents and show preflight if found.
	///
	/// Returns true if preflight was activated
	/// (caller should skip normal send).
	pub(crate) fn build_preflight(
		&mut self,
	) -> bool {
		let expanded = self.expand_paste_blocks();
		let (main, agents) =
			parser_agent::parse_agent_lines(&expanded);
		if agents.is_empty() {
			return false;
		}
		let mode_label = detect_mode_label(&main);
		self.preflight = Some(PreFlightData {
			main_prompt: main,
			agents,
			mode_label,
			file_count: self.file_references.len(),
			symbol_count: self.symbol_selectors.len(),
		});
		true
	}

	/// Confirm preflight and send the message.
	pub(crate) fn confirm_preflight(&mut self) {
		self.preflight = None;
		self.send_current_input();
	}

	/// Cancel preflight without sending.
	pub(crate) fn cancel_preflight(&mut self) {
		self.preflight = None;
	}
}

/// Detect query mode label from input text.
fn detect_mode_label(text: &str) -> String {
	let (mode, _) = detect_query_mode(text);
	match mode {
		Some(m) => format!("{m:?}"),
		None => "Default".to_string(),
	}
}
