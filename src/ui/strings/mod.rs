//! Centralized UI string constants.
//!
//! All user-facing text lives here. No inline
//! string literals in commands or UI code.
//!
//! # Modules
//! - `cli_messages`: CLI command output text
//! - `daemon_messages`: Daemon status/error text
//! - `progress_messages`: Progress bar/spinner text
//! - `tui_labels`: TUI component labels

pub mod cli_messages;
pub mod daemon_messages;
pub mod progress_messages;
pub mod tui_labels;
