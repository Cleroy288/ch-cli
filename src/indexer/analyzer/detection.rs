//! Project type detection utilities.

use std::path::Path;

use super::ProjectType;

/// Check if a path should be skipped during analysis
pub fn should_skip_path(path: &Path) -> bool {
	if let Some(path_str) = path.to_str() {
		path_str.contains("/target/")
			|| path_str.contains("/node_modules/")
			|| path_str.contains("/.git/")
			|| path_str.contains("/vendor/")
			|| path_str.contains("/build/")
			|| path_str.contains("/dist/")
			|| path_str.contains("/__pycache__/")
	} else {
		false
	}
}

/// Detect project type from configuration files
pub fn detect_project_type(root: &Path) -> Option<ProjectType> {
	// Check for Rust (highest priority for this tool)
	if root.join("Cargo.toml").exists() {
		return Some(ProjectType::RustCargo);
	}

	// Check for Go
	if root.join("go.mod").exists() {
		return Some(ProjectType::GoMod);
	}

	// Check for Node.js
	if root.join("package.json").exists() {
		return Some(ProjectType::NodeJs);
	}

	// Check for Python
	if root.join("pyproject.toml").exists()
		|| root.join("setup.py").exists()
		|| root.join("requirements.txt").exists()
	{
		return Some(ProjectType::Python);
	}

	// Check for Java/Gradle
	if root.join("build.gradle").exists()
		|| root.join("build.gradle.kts").exists()
	{
		return Some(ProjectType::Gradle);
	}

	// Check for Java/Maven
	if root.join("pom.xml").exists() {
		return Some(ProjectType::Maven);
	}

	// Check for .NET (look for any .csproj or .sln)
	if let Ok(entries) = std::fs::read_dir(root) {
		for entry in entries.flatten() {
			if let Some(ext) = entry.path().extension() {
				if ext == "csproj" || ext == "sln" {
					return Some(ProjectType::DotNet);
				}
			}
		}
	}

	Some(ProjectType::Unknown)
}
