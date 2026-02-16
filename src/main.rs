mod main_routes;
mod main_routes_extra;
mod main_tui;

use rustean::retrieval::daemon::prewarm_daemon;
use rustean::{Cli, Commands};
use clap::Parser;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli_args = Cli::parse();
    prewarm_daemon_if_needed(&cli_args.command);

    match cli_args.command {
        None | Some(Commands::Tui) => {
            main_tui::run_tui_flow()
        }
        Some(cmd) => dispatch_command(cmd),
    }
}

/// Dispatch a CLI subcommand to its handler
fn dispatch_command(cmd: Commands) -> ExitCode {
    let result = main_routes::route_command(cmd);
    if let Err(err) = result {
        log_error(&format!("Error: {}", err));
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

/// Log an error to stderr
pub(crate) fn log_error(msg: &str) {
    let _ = writeln!(io::stderr(), "{}", msg);
}

/// Log a warning to stderr
pub(crate) fn log_warning(msg: &str) {
    let _ =
        writeln!(io::stderr(), "Warning: {}", msg);
}

/// Pre-warm daemon for commands that need it
fn prewarm_daemon_if_needed(
    command: &Option<Commands>,
) {
    match command {
        Some(Commands::Retrieve { .. })
        | Some(Commands::Embed { .. })
        | Some(Commands::Search {
            semantic: true, ..
        })
        | Some(Commands::Docs { .. })
        | Some(Commands::Daemon { .. }) => {}
        _ => {
            std::thread::spawn(prewarm_daemon);
        }
    }
}
