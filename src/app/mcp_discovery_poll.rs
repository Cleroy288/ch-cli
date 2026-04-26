use std::sync::mpsc::TryRecvError;

use super::App;

/// Poll for MCP discovery results (non-blocking)
pub fn tick_mcp_discovery(app: &mut App) {
	let Some(recv) = &app.mcp_discovery_rx else {
		return;
	};
	match recv.try_recv() {
		Ok(items) => {
			app.picker.set_discovered_tools(items);
			app.mcp_discovery_rx = None;
		}
		Err(TryRecvError::Empty) => {}
		Err(TryRecvError::Disconnected) => {
			app.mcp_discovery_rx = None;
		}
	}
}
