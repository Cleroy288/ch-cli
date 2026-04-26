use std::io::BufRead;
use std::sync::{mpsc, Arc};
use std::thread;

use crate::domain::claude::StreamChunk;
use crate::domain::skill::SkillEntry;
use crate::message::MessageSegment;
use crate::service::backend::CliBackend;
use crate::service::claude::{
	build_prompt, default_instruction,
	detect_query_mode, mode_instruction,
	wrap_prompt,
};

use super::claude_request_agent::merge_agent_sys;
use super::parser_agent;
use super::App;

/// Session context captured for the background request.
struct SessionSnapshot {
	sess: Option<String>,
	model: String,
	effort: String,
	backend: Arc<dyn CliBackend>,
}

/// Spawn a background Claude request from user segments.
pub fn spawn_claude_request(
	app: &mut App,
	segments: &[MessageSegment],
) {
	let (prompt, merged_sys) =
		build_expanded_prompt(segments, &app.skills);
	let session = snapshot_session(app);
	let (sender, recv) = mpsc::channel();
	app.claude_rx = Some(recv);
	app.streaming_text.clear();

	thread::spawn(move || {
		let child = session.backend.spawn_streaming(
			&prompt,
			session.sess.as_deref(),
			&session.model,
			&session.effort,
			merged_sys.as_deref(),
		);
		pipe_stream(child, &sender, &*session.backend);
	});
}

/// Build the user prompt and merged system prompt
/// from raw segments and available skills.
fn build_expanded_prompt(
	segments: &[MessageSegment],
	skills: &[SkillEntry],
) -> (String, Option<String>) {
	let raw = build_prompt(segments);
	let expanded = inject_skill_content(&raw, skills);
	let (main, agents) =
		parser_agent::parse_agent_lines(&expanded);
	let (prompt, sys) = extract_mode(&main);
	let merged = merge_agent_sys(sys, &agents);
	(prompt, merged)
}

/// Snapshot the session context required by the
/// background request thread.
fn snapshot_session(app: &App) -> SessionSnapshot {
	SessionSnapshot {
		sess: app.claude_session_id.clone(),
		model: app.model_name().to_string(),
		effort: app.effort_level().to_string(),
		backend: Arc::clone(&app.backend),
	}
}

/// Replace /skill-name with skill file content
fn inject_skill_content(
	raw: &str,
	skills: &[SkillEntry],
) -> String {
	let trimmed = raw.trim();
	if !trimmed.starts_with('/') {
		return raw.to_string();
	}
	for skill in skills {
		let prefix = format!("/{}", skill.name);
		if !trimmed.starts_with(&prefix) {
			continue;
		}
		let after = &trimmed[prefix.len()..];
		if !after.is_empty()
			&& !after.starts_with(' ')
		{
			continue;
		}
		return expand_skill(
			&skill.path, after.trim(),
		);
	}
	raw.to_string()
}

/// Read skill file and prepend to user text
fn expand_skill(
	path: &std::path::Path,
	user_text: &str,
) -> String {
	let content = std::fs::read_to_string(path)
		.unwrap_or_default();
	if user_text.is_empty() {
		content
	} else {
		format!("{content}\n\n{user_text}")
	}
}

fn extract_mode(
	raw: &str,
) -> (String, Option<String>) {
	let (mode, text) = detect_query_mode(raw);
	match mode {
		Some(qmode) => (
			wrap_prompt(text),
			Some(mode_instruction(qmode)),
		),
		None => (
			wrap_prompt(raw),
			Some(default_instruction()),
		),
	}
}

fn pipe_stream(
	child: Result<
		std::process::Child,
		crate::domain::errors::BackendError,
	>,
	sender: &mpsc::Sender<StreamChunk>,
	backend: &dyn CliBackend,
) {
	match child {
		Ok(mut proc) => {
			read_stream_lines(
				&mut proc, sender, backend,
			);
			let _ = proc.wait();
		}
		Err(err) => {
			let _ = sender.send(
				StreamChunk::Error(err.to_string()),
			);
		}
	}
}

fn read_stream_lines(
	child: &mut std::process::Child,
	sender: &mpsc::Sender<StreamChunk>,
	backend: &dyn CliBackend,
) {
	let stdout = match child.stdout.take() {
		Some(out) => out,
		None => return,
	};
	let reader = std::io::BufReader::new(stdout);
	for line in reader.lines().map_while(Result::ok)
	{
		for chunk in
			backend.parse_stream_line(&line)
		{
			if sender.send(chunk).is_err() {
				return;
			}
		}
	}
}
