use std::path::Path;

use super::ProjectType;

pub fn should_skip_path(path: &Path) -> bool {
	let Some(path_str) = path.to_str() else {
		return false;
	};
	path_str.contains("/target/")
		|| path_str.contains("/node_modules/")
		|| path_str.contains("/.git/")
		|| path_str.contains("/vendor/")
		|| path_str.contains("/build/")
		|| path_str.contains("/dist/")
		|| path_str.contains("/__pycache__/")
}

pub fn detect_project_type(root: &Path) -> Option<ProjectType> {
	// Check primary project types (Rust, Go, Node, Python)
	if let Some(proj) = detect_primary_type(root) {
		return Some(proj);
	}
	// Check JVM and .NET project types
	detect_secondary_type(root)
}

fn detect_primary_type(root: &Path) -> Option<ProjectType> {
	if root.join("Cargo.toml").exists() {
		return Some(ProjectType::RustCargo);
	}
	if root.join("go.mod").exists() {
		return Some(ProjectType::GoMod);
	}
	if root.join("package.json").exists() {
		return Some(ProjectType::NodeJs);
	}
	if root.join("pyproject.toml").exists()
		|| root.join("setup.py").exists()
		|| root.join("requirements.txt").exists()
	{
		return Some(ProjectType::Python);
	}
	None
}

fn detect_secondary_type(root: &Path) -> Option<ProjectType> {
	if root.join("build.gradle").exists()
		|| root.join("build.gradle.kts").exists()
	{
		return Some(ProjectType::Gradle);
	}
	if root.join("pom.xml").exists() {
		return Some(ProjectType::Maven);
	}
	if has_dotnet_project(root) {
		return Some(ProjectType::DotNet);
	}
	Some(ProjectType::Unknown)
}

fn has_dotnet_project(root: &Path) -> bool {
	let Ok(entries) = std::fs::read_dir(root)
	else {
		return false;
	};
	entries.flatten().any(|entry| {
		entry
			.path()
			.extension()
			.is_some_and(|ext| {
				ext == "csproj" || ext == "sln"
			})
	})
}
