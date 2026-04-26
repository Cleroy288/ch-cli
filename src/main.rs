mod main_routes;
mod main_routes_extra;
mod main_tui;

use rustean::{Cli, Commands};
use clap::Parser;
use std::io::{self, Write};
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli_args = Cli::parse();

    match cli_args.command {
        None | Some(Commands::Tui) => {
            main_tui::run_tui_flow()
        }
        Some(cmd) => dispatch_command(cmd),
    }
}

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
