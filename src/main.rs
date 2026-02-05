use ch_cli::{app::App, cli, events, startup, ui, Cli, Commands};
use ch_cli::cli::{DaemonAction, DocsAction};
use ch_cli::retrieval::daemon::{ensure_daemon_ready, prewarm_daemon};
use clap::Parser;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    // Parse CLI arguments
    let cli_args = Cli::parse();

    // Pre-warm daemon in background for commands that might need it later
    // This starts the daemon early so models are loaded by the time we need them
    prewarm_daemon_if_needed(&cli_args.command);

    // Handle CLI commands or launch TUI
    match cli_args.command {
        // No command or explicit TUI command -> launch TUI
        None | Some(Commands::Tui) => {
            // Run startup flow (check for index, prompt if needed)
            match startup::run_startup() {
                Ok(_) => {
                    // Startup successful (indexed or skipped), launch TUI
                    if let Err(e) = run_tui() {
                        eprintln!("Error: {}", e);
                        return ExitCode::FAILURE;
                    }
                }
                Err(e) if e.kind() == io::ErrorKind::Interrupted => {
                    // User chose to quit
                    return ExitCode::SUCCESS;
                }
                Err(e) => {
                    eprintln!("Startup error: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        }

        // Index command
        Some(Commands::Index {
            path,
            semantic,
            verbose,
        }) => {
            if let Err(e) = cli::commands::index_command(&path, semantic, verbose) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Search command
        Some(Commands::Search {
            query,
            limit,
            fuzzy,
            kind,
            semantic,
            context,
            rerank,
        }) => {
            // Ensure daemon is ready for semantic search
            if semantic && !ensure_daemon_ready() {
                eprintln!("Warning: Daemon not ready, search may be slower");
            }
            if let Err(e) = cli::commands::search_command(&query, limit, fuzzy, kind.as_deref(), semantic, context, rerank) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Goto command
        Some(Commands::Goto { symbol }) => {
            if let Err(e) = cli::commands::goto_command(&symbol) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Refs command
        Some(Commands::Refs {
            symbol,
            include_definition,
        }) => {
            if let Err(e) = cli::commands::refs_command(&symbol, include_definition) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Symbols command
        Some(Commands::Symbols { file, kind }) => {
            if let Err(e) = cli::commands::symbols_command(file.as_deref(), kind.as_deref()) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Stats command
        Some(Commands::Stats) => {
            if let Err(e) = cli::commands::stats_command() {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Daemon command
        Some(Commands::Daemon { action }) => {
            let result = match action {
                DaemonAction::Start => cli::commands::daemon_start_command(),
                DaemonAction::Stop => cli::commands::daemon_stop_command(),
                DaemonAction::Status => cli::commands::daemon_status_command(),
                DaemonAction::Restart => cli::commands::daemon_restart_command(),
                DaemonAction::Run { socket } => {
                    cli::commands::daemon_run_command(socket.as_deref())
                }
            };
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Embed command
        Some(Commands::Embed { path, force }) => {
            // Ensure daemon is ready for embedding
            if !ensure_daemon_ready() {
                eprintln!("Warning: Daemon not ready, embedding may be slower");
            }
            if let Err(e) = cli::commands::embed_command(&path, force) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Retrieve command (agentic pipeline)
        Some(Commands::Retrieve {
            query,
            limit,
            max_tokens,
            no_expand,
            no_rerank,
            no_context,
            xml,
            structured,
            threshold,
            min_results,
        }) => {
            // Ensure daemon is ready before retrieval (blocking wait)
            if !ensure_daemon_ready() {
                eprintln!("Warning: Daemon not ready, retrieval may be slower");
            }
            if let Err(e) = cli::commands::retrieve_command(
                &query, limit, max_tokens, no_expand, no_rerank, no_context, xml, structured,
                threshold, min_results,
            ) {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }

        // Docs command
        Some(Commands::Docs { action }) => {
            // Ensure daemon is ready for docs commands
            if !ensure_daemon_ready() {
                eprintln!("Warning: Daemon not ready, operation may fail");
            }
            let result = match action {
                DocsAction::Generate { force } => cli::commands::docs_generate_command(force),
                DocsAction::Status => cli::commands::docs_status_command(),
                DocsAction::Show { symbol } => cli::commands::docs_show_command(&symbol),
                DocsAction::Search { query, limit } => cli::commands::docs_search_command(&query, limit),
            };
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}

/// Pre-warm the daemon for commands that will need it
/// Starts daemon in background without blocking
fn prewarm_daemon_if_needed(command: &Option<Commands>) {
    match command {
        // Commands that need daemon - it will be started by ensure_daemon_ready
        Some(Commands::Retrieve { .. }) => {}
        Some(Commands::Embed { .. }) => {}
        Some(Commands::Search { semantic: true, .. }) => {}
        Some(Commands::Docs { .. }) => {}

        // Daemon commands - don't interfere
        Some(Commands::Daemon { .. }) => {}

        // All other commands - prewarm in background for future use
        _ => {
            // Start daemon in background (non-blocking)
            // This way the daemon will be warm for subsequent commands
            std::thread::spawn(|| {
                prewarm_daemon();
            });
        }
    }
}

/// Run the interactive TUI
fn run_tui() -> io::Result<()> {
    // Initialize the application state
    let mut app = App::new();

    // Run the terminal UI
    let result = ratatui::run(|terminal| {
        loop {
            // Draw the UI
            terminal.draw(|frame| ui::render(frame, &app))?;

            // Handle events and check if we should quit
            if events::handle_events(&mut app)? {
                break Ok(());
            }
        }
    });

    // Show goodbye message after terminal is restored
    ui::show_goodbye_message();

    result
}
