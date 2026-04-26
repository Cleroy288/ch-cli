use std::io;
use std::path::Path;

use crate::domain::config::SetupState;
use crate::indexer::IndexResult;
use crate::service::config;

use super::{
	analyze_codebase, check_index_exists,
	detect_codebase_changes, ensure_mcp_registered,
	index_with_progress,
	index_with_progress_incremental,
	prompt_for_indexing_with_analysis,
	prompt_for_update,
};
use super::action::StartupAction;
use super::unsupported_display
	::display_unsupported_language_message;

fn run_early_startup() -> io::Result<()> {
	let root = std::path::Path::new(".");
	super::migration::migrate_data_layout(root);
	super::migration_config
		::migrate_to_unified_config(root);
	let _ = crate::service::manifest
		::ensure_registered(root);
	ensure_mcp_registered();
	crate::service::skills::provision_defaults();
	super::check_and_prompt_credentials()?;
	super::check_and_prompt_agent()?;
	super::integration_mode
		::check_and_prompt_integration_mode()?;
	super::repo_scan::check_and_prompt_repo_scan()
}

pub fn run_startup(
) -> io::Result<Option<IndexResult>> {
	run_early_startup()?;
	let analysis = analyze_codebase();

	if !analysis.is_primary_supported
		&& !analysis.has_supported_files()
	{
		if let Some(primary) =
			analysis.primary_language
		{
			display_unsupported_language_message(
				primary,
				0,
				analysis.total_source_files,
			)?;
		}
		return Ok(None);
	}

	if check_index_exists() {
		handle_existing_index()
	} else {
		handle_first_launch(&analysis)
	}
}

fn handle_existing_index(
) -> io::Result<Option<IndexResult>> {
	let Some(changes) = detect_codebase_changes()
	else {
		return Ok(None);
	};
	match prompt_for_update(&changes)? {
		StartupAction::Update(_) => {
			index_with_progress_incremental(
				&changes,
			)
		}
		StartupAction::Quit => Err(io::Error::new(
			io::ErrorKind::Interrupted,
			"User quit",
		)),
		_ => Ok(None),
	}
}

fn handle_first_launch(
	analysis: &crate::indexer::CodebaseAnalysis,
) -> io::Result<Option<IndexResult>> {
	let root = Path::new(".");
	let cfg = config::load_config(root);
	if cfg.setup.indexing {
		return Ok(None);
	}
	prompt_first_launch(root, analysis)
}

fn prompt_first_launch(
	root: &Path,
	analysis: &crate::indexer::CodebaseAnalysis,
) -> io::Result<Option<IndexResult>> {
	let show_note =
		!analysis.is_primary_supported
			&& analysis.has_supported_files();

	match prompt_for_indexing_with_analysis(
		show_note, analysis,
	)? {
		StartupAction::Index => {
			index_with_progress()
		}
		StartupAction::Skip => {
			mark_setup_done(root, |s| {
				s.indexing = true;
			})?;
			Ok(None)
		}
		StartupAction::Quit => Err(io::Error::new(
			io::ErrorKind::Interrupted,
			"User quit",
		)),
		_ => Ok(None),
	}
}

pub(crate) fn mark_setup_done<F>(
	root: &Path,
	mutate: F,
) -> io::Result<()>
where
	F: FnOnce(&mut SetupState),
{
	config::update_config(root, |cfg| {
		mutate(&mut cfg.setup);
	})
}
