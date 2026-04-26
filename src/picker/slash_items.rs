use crate::domain::skill::SkillEntry;

use super::mode::PickerMode;

/// A slash command or argument item
pub struct SlashItem {
	pub name: &'static str,
	pub description: &'static str,
	pub kind: ItemKind,
}

/// Item kind for display labeling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ItemKind {
	Command,
	Mode,
	Arg,
	Skill,
}

/// Unified display item (static or dynamic)
pub struct DisplayItem {
	pub name: String,
	pub description: String,
	pub kind: ItemKind,
}

const SLASH_COMMANDS: &[SlashItem] = &[
	SlashItem {
		name: "Q",
		description: "Question — explain, no code",
		kind: ItemKind::Mode,
	},
	SlashItem {
		name: "A",
		description: "Action — implement changes",
		kind: ItemKind::Mode,
	},
	SlashItem {
		name: "P",
		description: "Plan — analyze before coding",
		kind: ItemKind::Mode,
	},
	SlashItem {
		name: "new",
		description: "New conversation",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "model",
		description: "Switch model",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "effort",
		description: "Thinking depth",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "e-prompt",
		description: "Enhance prompt (Ctrl+E)",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "agent",
		description: "Add subagent line",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "backend",
		description: "Switch AI backend",
		kind: ItemKind::Command,
	},
	SlashItem {
		name: "git",
		description: "Git history graph",
		kind: ItemKind::Command,
	},
];

const MODEL_ARGS: &[SlashItem] = &[
	SlashItem {
		name: "haiku",
		description: "Fast, lightweight",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "sonnet",
		description: "Balanced",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "opus",
		description: "Most capable",
		kind: ItemKind::Arg,
	},
];

const BACKEND_ARGS: &[SlashItem] = &[
	SlashItem {
		name: "claude",
		description: "Claude Code CLI",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "gemini",
		description: "Gemini CLI",
		kind: ItemKind::Arg,
	},
];

const EFFORT_ARGS: &[SlashItem] = &[
	SlashItem {
		name: "low",
		description: "Fast, minimal thinking",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "medium",
		description: "Think on hard problems",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "high",
		description: "Deep thinking (default)",
		kind: ItemKind::Arg,
	},
	SlashItem {
		name: "max",
		description: "Maximum depth",
		kind: ItemKind::Arg,
	},
];

/// Items for the current picker mode
pub fn items_for_mode(
	mode: &PickerMode,
	query: &str,
) -> Vec<&'static SlashItem> {
	match mode {
		PickerMode::SlashCommand => {
			filter_by_prefix(SLASH_COMMANDS, query)
		}
		PickerMode::SlashArg { command } => {
			args_for_command(command, query)
		}
		_ => vec![],
	}
}

fn args_for_command(
	command: &str,
	query: &str,
) -> Vec<&'static SlashItem> {
	match command {
		"model" => {
			filter_by_prefix(MODEL_ARGS, query)
		}
		"effort" => {
			filter_by_prefix(EFFORT_ARGS, query)
		}
		"backend" => {
			filter_by_prefix(BACKEND_ARGS, query)
		}
		_ => vec![],
	}
}

/// Case-insensitive prefix filter
fn filter_by_prefix(
	items: &'static [SlashItem],
	query: &str,
) -> Vec<&'static SlashItem> {
	let lower = query.to_lowercase();
	items
		.iter()
		.filter(|item| {
			item.name
				.to_lowercase()
				.starts_with(&lower)
		})
		.collect()
}

/// All items: built-in + skills, unified
pub fn all_display_items(
	mode: &PickerMode,
	query: &str,
	skills: &[SkillEntry],
) -> Vec<DisplayItem> {
	let builtins = items_for_mode(mode, query);
	let mut items: Vec<DisplayItem> = builtins
		.into_iter()
		.map(|i| DisplayItem {
			name: i.name.to_string(),
			description: i.description.to_string(),
			kind: i.kind,
		})
		.collect();
	if matches!(mode, PickerMode::SlashCommand) {
		let filtered =
			filter_skills(skills, query);
		items.extend(filtered);
	}
	items
}

/// Filter skills by query prefix
fn filter_skills(
	skills: &[SkillEntry],
	query: &str,
) -> Vec<DisplayItem> {
	let lower = query.to_lowercase();
	skills
		.iter()
		.filter(|s| {
			s.name.to_lowercase()
				.starts_with(&lower)
		})
		.map(|s| DisplayItem {
			name: s.name.clone(),
			description: s.description.clone(),
			kind: ItemKind::Skill,
		})
		.collect()
}
