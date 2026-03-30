use std::path::Path;
use std::time::SystemTime;

/// Max mtime of .git/HEAD, .git/refs/, .git/packed-refs.
pub fn git_mtime(repo: &Path) -> SystemTime {
	let head = mtime_of(
		&repo.join(".git").join("HEAD"),
	);
	let refs = mtime_of(
		&repo.join(".git").join("refs"),
	);
	let packed = mtime_of(
		&repo.join(".git").join("packed-refs"),
	);
	head.max(refs).max(packed)
}

fn mtime_of(path: &Path) -> SystemTime {
	std::fs::metadata(path)
		.and_then(|m| m.modified())
		.unwrap_or(SystemTime::UNIX_EPOCH)
}
