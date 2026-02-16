use rustean::cli::{
    self, DocsAction, MemoryAction,
};
use rustean::domain::errors::command::CommandResult;
use rustean::retrieval::daemon::ensure_daemon_ready;
use rustean::Commands;

/// Route embed subcommand
pub(crate) fn route_embed(
    path: &str,
    force: bool,
) -> CommandResult {
    if !ensure_daemon_ready() {
        crate::log_warning("Daemon not ready");
    }
    cli::commands::embed_command(path, force)
}

/// Route retrieve command variants
#[allow(clippy::fn_params_excessive_bools)]
pub(crate) fn route_retrieve(
    cmd: Commands,
) -> CommandResult {
    let Commands::Retrieve {
        query, limit, max_tokens,
        no_expand, no_rerank, no_context,
        xml, structured, threshold, min_results,
    } = cmd
    else {
        return Ok(());
    };
    if !ensure_daemon_ready() {
        crate::log_warning("Daemon not ready");
    }
    let flags = cli::commands::RetrieveFlags {
        no_expand, no_rerank, no_context,
        xml, structured,
    };
    let opts = cli::commands::RetrieveOptions {
        limit, max_tokens, flags,
        threshold, min_results,
    };
    cli::commands::retrieve_command(&query, &opts)
}

/// Route docs subcommands
pub(crate) fn route_docs(
    action: DocsAction,
) -> CommandResult {
    if !ensure_daemon_ready() {
        crate::log_warning("Daemon not ready");
    }
    match action {
        DocsAction::Generate { force } => {
            cli::commands::docs_generate_command(
                force,
            )
        }
        DocsAction::Status => {
            cli::commands::docs_status_command()
        }
        DocsAction::Show { symbol } => {
            cli::commands::docs_show_command(&symbol)
        }
        DocsAction::Search { query, limit } => {
            cli::commands::docs_search_command(
                &query, limit,
            )
        }
    }
}

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
    let flags = cli::commands::InfoFlags { sections };
    let opts =
        cli::commands::InfoDisplayOpts::from_flags(
            flags, all,
        );
    cli::commands::info_command(&symbol, &opts)
}
