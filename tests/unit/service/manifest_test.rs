//! Tests for manifest service (load/save/register).

use rustean::domain::manifest::{
	ProjectEntry, ProjectManifest,
};
use rustean::service::manifest;

/// Round-trip: save then load preserves data
#[test]
fn save_and_load_roundtrip() {
	// Arrange
	let mut m = ProjectManifest::default();
	m.upsert(ProjectEntry {
		hash: "abcd1234".to_string(),
		path: "/home/x/proj".to_string(),
	});
	manifest::save_manifest(&m).unwrap();

	// Act
	let loaded = manifest::load_manifest();

	// Assert
	assert!(!loaded.projects.is_empty());
}

/// ensure_registered adds project to manifest
#[test]
fn ensure_registered_adds_project() {
	// Arrange
	let root = tempfile::tempdir().unwrap();

	// Act
	manifest::ensure_registered(root.path())
		.unwrap();

	// Assert
	let m = manifest::load_manifest();
	assert!(!m.projects.is_empty());
}
