use rustean::cli::{self, MemoryAction};
use rustean::domain::errors::command::CommandResult;
use rustean::Commands;

/// Route memory subcommands
pub(crate) fn route_memory(
    action: MemoryAction,
) -> CommandResult {
    match action {
        MemoryAction::Show { limit, session } => {
            cli::commands::memory_show_command(
                limit, session.as_deref(),
            )
        }
        MemoryAction::Search { query, limit } => {
            cli::commands::memory_search_command(
                &query, limit,
            )
        }
        MemoryAction::Stats => {
            cli::commands::memory_stats_command()
        }
        MemoryAction::Add {
            input, response, response_type,
        } => cli::commands::memory_add_command(
            &input, &response, &response_type,
        ),
    }
}

/// Route info subcommand
pub(crate) fn route_info(
    cmd: Commands,
) -> CommandResult {
    let Commands::Info {
        symbol, code, callers,
        callees, refs, all,
    } = cmd
    else {
        return Ok(());
    };
    let sections = cli::commands::InfoSections {
        code, callers, callees, refs,
    };
    let flags =
        cli::commands::InfoFlags { sections };
    let opts =
        cli::commands::InfoDisplayOpts::from_flags(
            flags, all,
        );
    cli::commands::info_command(&symbol, &opts)
}
