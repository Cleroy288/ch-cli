use rustean::app::claude_poll::tick_claude_response;
use rustean::app::doc_preview_poll::tick_doc_preview;
use rustean::app::doc_progress_poll::{
    tick_doc_progress, trigger_initial_doc_gen,
};
use rustean::fs::FileCache;
use rustean::retrieval::daemon::signal_daemon_stop;
use rustean::{app::App, events, startup, ui};
use std::io;
use std::process::ExitCode;

/// Run TUI startup flow and launch interactive UI
pub(crate) fn run_tui_flow() -> ExitCode {
    match startup::run_startup() {
        Ok(_) => launch_tui_with_watcher(),
        Err(err) if err.kind()
            == io::ErrorKind::Interrupted =>
        {
            ExitCode::SUCCESS
        }
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
    let project_path =
        std::env::current_dir().unwrap_or_default();
    let path_str =
        project_path.display().to_string();
    let file_cache = FileCache::new_with_scan();
    let watcher_handle = startup::spawn_watcher(
        &project_path, file_cache.clone(),
    );

    trigger_initial_doc_gen(&path_str);
    let tui_result =
        run_tui(&path_str, file_cache);
    watcher_handle.stop();
    shutdown_daemon();

    if let Err(err) = tui_result {
        crate::log_error(
            &format!("Error: {}", err),
        );
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Stop the daemon process on TUI exit (fire-and-forget)
fn shutdown_daemon() {
    let config =
        rustean::retrieval::RetrievalConfig::default();
    let _ = signal_daemon_stop(&config.socket_path);
}

/// Run the interactive TUI loop
fn run_tui(
    project_path: &str,
    file_cache: FileCache,
) -> io::Result<()> {
    let mut app = App::new(file_cache);
    let result = ratatui::run(|terminal| {
        loop {
            app.sync_file_cache();
            terminal.draw(|frame| {
                ui::render(frame, &app);
            })?;
            tick_doc_progress(
                &mut app, project_path,
            );
            tick_doc_preview(&mut app);
            tick_claude_response(&mut app);
            if events::handle_events(&mut app)? {
                break Ok(());
            }
        }
    });
    ui::show_goodbye_message();
    result
}
