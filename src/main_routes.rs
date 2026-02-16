use super::main_routes_extra as extra;
use rustean::cli::{self, DaemonAction};
use rustean::domain::errors::command::CommandResult;
use rustean::retrieval::daemon::ensure_daemon_ready;
use rustean::Commands;

/// Route command to the right handler function
pub(crate) fn route_command(
    cmd: Commands,
) -> CommandResult {
    match cmd {
        Commands::Tui => Ok(()),
        Commands::Search { .. } => route_search(cmd),
        Commands::Retrieve { .. } => {
            extra::route_retrieve(cmd)
        }
        Commands::Info { .. } => {
            extra::route_info(cmd)
        }
        Commands::Daemon { action } => {
            route_daemon(action)
        }
        Commands::Docs { action } => {
            extra::route_docs(action)
        }
        Commands::Memory { action } => {
            extra::route_memory(action)
        }
        Commands::McpServer => {
            cli::commands::mcp_server_command()
        }
        _ => route_simple_command(cmd),
    }
}

/// Route simple commands (index, goto, refs, etc.)
fn route_simple_command(
    cmd: Commands,
) -> CommandResult {
    match cmd {
        Commands::Index {
            path, semantic, verbose,
        } => cli::commands::index_command(
            &path, semantic, verbose,
        ),
        Commands::Goto { symbol } => {
            cli::commands::goto_command(&symbol)
        }
        Commands::Refs {
            symbol, include_definition,
        } => cli::commands::refs_command(
            &symbol, include_definition,
        ),
        Commands::Symbols { file, kind } => {
            route_symbols(file, kind)
        }
        Commands::Stats => {
            cli::commands::stats_command()
        }
        Commands::Embed { path, force } => {
            extra::route_embed(&path, force)
        }
        _ => Ok(()),
    }
}

/// Route symbols command with optional filters
fn route_symbols(
    file: Option<String>,
    kind: Option<String>,
) -> CommandResult {
    cli::commands::symbols_command(
        file.as_deref(), kind.as_deref(),
    )
}

/// Route search command variants
#[allow(clippy::fn_params_excessive_bools)]
fn route_search(cmd: Commands) -> CommandResult {
    let Commands::Search {
        query, limit, fuzzy, kind,
        semantic, context, rerank, full,
    } = cmd
    else {
        return Ok(());
    };
    if semantic && !ensure_daemon_ready() {
        crate::log_warning("Daemon not ready");
    }
    let flags = cli::commands::SearchCommandFlags {
        fuzzy, semantic, context, rerank, full,
    };
    let opts = cli::commands::SearchCommandOptions {
        limit,
        kind: kind.as_deref(),
        flags,
    };
    cli::commands::search_command(&query, &opts)
}

/// Route daemon subcommands
fn route_daemon(
    action: DaemonAction,
) -> CommandResult {
    match action {
        DaemonAction::Start => {
            cli::commands::daemon_start_command()
        }
        DaemonAction::Stop => {
            cli::commands::daemon_stop_command()
        }
        DaemonAction::Status => {
            cli::commands::daemon_status_command()
        }
        DaemonAction::Restart => {
            cli::commands::daemon_restart_command()
        }
        DaemonAction::Run { socket } => {
            cli::commands::daemon_run_command(
                socket.as_deref(),
            )
        }
    }
}
