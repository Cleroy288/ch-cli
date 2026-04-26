use std::sync::mpsc::TryRecvError;

use crate::domain::errors::BackendError;
use crate::ui::strings::tui_labels;

use super::App;

/// Poll for enhance result (non-blocking).
pub fn tick_enhance_result(app: &mut App) {
	let Some(recv) = &app.enhance_rx else {
		return;
	};
	match recv.try_recv() {
		Ok(Ok(text)) => apply_enhanced(app, text),
		Ok(Err(err)) => handle_error(app, err),
		Err(TryRecvError::Empty) => {}
		Err(TryRecvError::Disconnected) => {
			handle_error(
				app,
				BackendError::Process(
					"enhance disconnected".into(),
				),
			);
		}
	}
}

/// Replace input with enhanced prompt + agents.
fn apply_enhanced(app: &mut App, text: String) {
	app.input.clear();
	app.input.push_str(text.trim());
	if let Some(agents) =
		app.pre_enhance_agents.take()
	{
		app.input.push('\n');
		app.input.push_str(&agents);
	}
	app.cursor_position.set(app.input.len());
	app.enhance_rx = None;
	app.pre_enhance_input = None;
	app.status_message = None;
}

/// Restore original input on error.
fn handle_error(
	app: &mut App,
	err: BackendError,
) {
	if let Some(original) =
		app.pre_enhance_input.take()
	{
		app.input.clear();
		app.input.push_str(&original);
		app.cursor_position.set(app.input.len());
	}
	app.enhance_rx = None;
	app.pre_enhance_agents = None;
	let msg = format!(
		"{}{}", tui_labels::ENHANCE_ERROR, err
	);
	app.status_message = Some(msg);
}
