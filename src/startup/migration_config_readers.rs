use std::fs;
use std::path::Path;

/// Generic helper: read + deserialize a JSON file
pub fn read_json<T: serde::de::DeserializeOwned>(
	data: &Path,
	name: &str,
) -> Option<T> {
	let text =
		fs::read_to_string(data.join(name)).ok()?;
	serde_json::from_str(&text).ok()
}

pub fn read_model(data: &Path) -> String {
	#[derive(serde::Deserialize)]
	struct M {
		model: String,
	}
	read_json::<M>(data, super::migration_config::OLD_MODEL)
		.map(|m| m.model)
		.unwrap_or_else(|| "sonnet".to_string())
}

pub fn read_session_id(
	data: &Path,
) -> Option<String> {
	#[derive(serde::Deserialize)]
	struct S {
		session_id: String,
	}
	read_json::<S>(data, super::migration_config::OLD_SESSION)
		.map(|sd| sd.session_id)
}
