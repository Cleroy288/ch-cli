use std::path::Path;

use crate::domain::backend_kind::BackendKind;
use crate::domain::effort;
use crate::service::config;

pub const DEFAULT_MODEL: &str = "sonnet";
const VALID_MODELS: &[&str] =
	&["haiku", "sonnet", "opus"];

pub fn is_valid_model(name: &str) -> bool {
	VALID_MODELS.contains(&name)
}

pub fn save_model(project: &str, model: &str) {
	let root = Path::new(project);
	let owned = model.to_string();
	let _ = config::update_config(root, move |cfg| {
		cfg.claude.model = owned;
	});
}

pub fn load_model(project: &str) -> String {
	let root = Path::new(project);
	let model = config::load_config(root)
		.claude
		.model;
	if is_valid_model(&model) {
		model
	} else {
		DEFAULT_MODEL.to_string()
	}
}

pub fn load_effort(project: &str) -> String {
	let root = Path::new(project);
	let effort = config::load_config(root)
		.claude
		.effort;
	if effort::is_valid(&effort) {
		effort
	} else {
		effort::DEFAULT_EFFORT.to_string()
	}
}

pub fn save_effort(project: &str, level: &str) {
	let root = Path::new(project);
	let owned = level.to_string();
	let _ = config::update_config(root, move |cfg| {
		cfg.claude.effort = owned;
	});
}

pub fn save_backend(
	project: &str,
	kind: BackendKind,
) {
	let root = Path::new(project);
	let _ = config::update_config(root, move |cfg| {
		cfg.claude.backend = kind;
	});
}

pub fn load_backend(project: &str) -> BackendKind {
	let root = Path::new(project);
	config::load_config(root).claude.backend
}
