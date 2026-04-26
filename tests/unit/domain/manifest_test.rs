//! Tests for ProjectManifest domain type.

use rustean::domain::manifest::{
	ProjectEntry, ProjectManifest,
};

/// upsert adds a new entry
#[test]
fn upsert_adds_new_entry() {
	let mut manifest = ProjectManifest::default();
	let entry = ProjectEntry {
		hash: "a1b2c3d4".to_string(),
		path: "/home/user/proj".to_string(),
	};

	manifest.upsert(entry.clone());

	assert_eq!(manifest.projects.len(), 1);
	assert_eq!(manifest.projects[0], entry);
}

/// upsert updates existing entry by hash
#[test]
fn upsert_updates_existing() {
	let mut manifest = ProjectManifest::default();
	manifest.upsert(ProjectEntry {
		hash: "a1b2c3d4".to_string(),
		path: "/old/path".to_string(),
	});
	manifest.upsert(ProjectEntry {
		hash: "a1b2c3d4".to_string(),
		path: "/new/path".to_string(),
	});

	assert_eq!(manifest.projects.len(), 1);
	assert_eq!(
		manifest.projects[0].path, "/new/path",
	);
}

/// upsert keeps different hashes separate
#[test]
fn upsert_keeps_distinct_hashes() {
	let mut manifest = ProjectManifest::default();
	manifest.upsert(ProjectEntry {
		hash: "aaaa1111".to_string(),
		path: "/proj-a".to_string(),
	});
	manifest.upsert(ProjectEntry {
		hash: "bbbb2222".to_string(),
		path: "/proj-b".to_string(),
	});

	assert_eq!(manifest.projects.len(), 2);
}
