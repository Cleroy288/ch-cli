use crossterm::execute;
use crossterm::event::{
	DisableBracketedPaste, EnableBracketedPaste,
	DisableMouseCapture, EnableMouseCapture,
};

use rustean::app::claude_poll::tick_claude_response;
use rustean::app::enhance_poll::tick_enhance_result;
use rustean::app::git_poll::tick_git_results;
use rustean::app::mcp_discovery_poll
	::tick_mcp_discovery;
use rustean::app::tool_poll::tick_tool_results;
use rustean::fs::FileCache;
use rustean::{app::App, events, startup, ui};
use std::io;
use std::process::ExitCode;

pub(crate) fn run_tui_flow() -> ExitCode {
	match startup::run_startup() {
		Ok(_) => launch_tui_with_watcher(),
		Err(err) if err.kind()
			== io::ErrorKind::Interrupted =>
			ExitCode::SUCCESS,
		Err(err) => {
			crate::log_error(&format!(
				"Startup error: {}", err
			));
			ExitCode::FAILURE
		}
	}
}

/// Launch TUI with file watcher
fn launch_tui_with_watcher() -> ExitCode {
	let project_path = std::env::current_dir()
		.unwrap_or_default();
	let file_cache = FileCache::new_with_scan();
	let watcher_msg =
		startup::watcher::new_watcher_msg();
	let watcher_handle = startup::spawn_watcher(
		&project_path, file_cache.clone(),
		watcher_msg.clone(),
	);

	let path_str =
		project_path.display().to_string();
	let tui_result =
		run_tui(&path_str, file_cache, watcher_msg);
	watcher_handle.stop();

	if let Err(err) = tui_result {
		crate::log_error(
			&format!("Error: {}", err),
		);
		return ExitCode::FAILURE;
	}
	ExitCode::SUCCESS
}

fn run_tui(
	// TODO: pass project_path to App for display
	_project_path: &str,
	file_cache: FileCache,
	watcher_msg: startup::watcher::WatcherMsg,
) -> io::Result<()> {
	let mut app = App::new(file_cache);
	app.watcher_msg = Some(watcher_msg);
	app.mcp_discovery_rx =
		startup::start_mcp_discovery();
	execute!(
		io::stdout(),
		EnableBracketedPaste,
		EnableMouseCapture,
	)?;
	let result = ratatui::run(|terminal| {
		loop {
			app.sync_file_cache();
			terminal.draw(|frame| {
				ui::render(frame, &app);
			})?;
			tick_claude_response(&mut app);
			tick_tool_results(&mut app);
			tick_enhance_result(&mut app);
			tick_git_results(&mut app);
			tick_mcp_discovery(&mut app);
			app.tick_watcher_msg();
			if events::handle_events(&mut app)? {
				break Ok(());
			}
		}
	});
	let _ = execute!(
		io::stdout(),
		DisableBracketedPaste,
		DisableMouseCapture,
	);
	ui::show_goodbye_message();
	result
}
