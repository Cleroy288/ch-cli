use super::main_routes_extra as extra;
use rustean::cli;
use rustean::domain::errors::command::CommandResult;
use rustean::Commands;

/// Route command to the right handler function
pub(crate) fn route_command(
    cmd: Commands,
) -> CommandResult {
    match cmd {
        Commands::Tui => Ok(()),
        Commands::Search { .. } => {
            route_search(cmd)
        }
        Commands::Info { .. } => {
            extra::route_info(cmd)
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
        Commands::Index { path, verbose } => {
            cli::commands::index_command(
                &path, verbose,
            )
        }
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

/// Route search command
fn route_search(cmd: Commands) -> CommandResult {
    let Commands::Search {
        query, limit, fuzzy, kind, full,
    } = cmd
    else {
        return Ok(());
    };
    let flags = cli::commands::SearchCommandFlags {
        fuzzy, full,
    };
    let opts = cli::commands::SearchCommandOptions {
        limit,
        kind: kind.as_deref(),
        flags,
    };
    cli::commands::search_command(&query, &opts)
}
