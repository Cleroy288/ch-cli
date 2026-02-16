//! Polling logic for background doc names fetch.
//!
//! Non-blocking channel poll called each event loop
//! tick. Receives doc names for the symbol browser
//! "(doc)" indicators.

use std::sync::mpsc;

use super::App;

/// Poll for background doc names result.
///
/// Called every event loop tick (~100ms).
/// Non-blocking: uses try_recv on the channel.
pub fn tick_doc_preview(app: &mut App) {
	let Some(recv) = &app.doc_names_rx else {
		return;
	};
	match recv.try_recv() {
		Ok(names) => {
			if let Some(browser) =
				app.picker.symbol_browser_mut()
			{
				browser.set_doc_names(names);
			}
			app.doc_names_rx = None;
		}
		Err(mpsc::TryRecvError::Empty) => {}
		Err(mpsc::TryRecvError::Disconnected) => {
			app.doc_names_rx = None;
		}
	}
}
