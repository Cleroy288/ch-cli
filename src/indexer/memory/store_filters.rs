use std::path::Path;
use std::time::SystemTime;

pub(crate) fn is_jsonl_file(
	path: &Path,
) -> bool {
	path.extension()
		.and_then(|ext| ext.to_str())
		== Some("jsonl")
}

pub(crate) fn file_modified_time(
	path: &Path,
) -> SystemTime {
	std::fs::metadata(path)
		.and_then(|meta| meta.modified())
		.unwrap_or(std::time::UNIX_EPOCH)
}
