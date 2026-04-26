use std::path::Path;
use std::sync::mpsc;
use std::thread;

use crate::domain::mcp_config::McpServerConfig;
use crate::picker::mcp_display::McpDisplayItem;
use crate::service::agents::config_claude
	::discover_claude_servers;
use crate::service::agents::discovery
	::discover_tools_stdio;
use crate::service::agents::preference::load_agent;

///
/// Returns a receiver that will emit discovered
/// tools once all servers have been queried.
pub fn start_mcp_discovery(
) -> Option<mpsc::Receiver<Vec<McpDisplayItem>>> {
	let root = Path::new(".");
	let _pref = load_agent(root)?;
	let servers = discover_claude_servers();
	if servers.is_empty() {
		return None;
	}
	let (sender, recv) = mpsc::channel();
	thread::spawn(move || {
		let items = discover_all(&servers);
		let _ = sender.send(items);
	});
	Some(recv)
}

/// Query all stdio servers in parallel
fn discover_all(
	servers: &[McpServerConfig],
) -> Vec<McpDisplayItem> {
	let handles: Vec<_> = servers
		.iter()
		.filter(|srv| srv.is_stdio())
		.map(|server| {
			let server = server.clone();
			thread::spawn(move || {
				discover_one(&server)
			})
		})
		.collect();
	handles
		.into_iter()
		.flat_map(|handle| {
			handle.join().unwrap_or_default()
		})
		.collect()
}

/// Discover tools from one server
fn discover_one(
	server: &McpServerConfig,
) -> Vec<McpDisplayItem> {
	let Ok(tools) = discover_tools_stdio(server)
	else {
		return Vec::new();
	};
	tools
		.into_iter()
		.map(|tool| McpDisplayItem {
			name: tool.name,
			description: tool.description,
			source: server.name.clone(),
		})
		.collect()
}
