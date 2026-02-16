//! Core ProjectType enum definition and basic methods.

use crate::indexer::crawler::{DetectedLanguage, Language};

/// Project type detected from configuration files
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
	/// Rust project (Cargo.toml present)
	RustCargo,
	/// Node.js project (package.json present)
	NodeJs,
	/// Python project (pyproject.toml, setup.py, or requirements.txt)
	Python,
	/// Go project (go.mod present)
	GoMod,
	/// Java/Gradle project (build.gradle present)
	Gradle,
	/// Java/Maven project (pom.xml present)
	Maven,
	/// .NET project (*.csproj or *.sln present)
	DotNet,
	/// Mixed or unknown project type
	Unknown,
}

impl ProjectType {
	/// Get the expected primary language for this project type
	pub fn expected_language(&self) -> Option<DetectedLanguage> {
		match self {
			ProjectType::RustCargo => {
				Some(DetectedLanguage::Supported(Language::Rust))
			}
			ProjectType::NodeJs => Some(DetectedLanguage::JavaScript),
			ProjectType::Python => Some(DetectedLanguage::Python),
			ProjectType::GoMod => Some(DetectedLanguage::GoLang),
			ProjectType::Gradle | ProjectType::Maven => {
				Some(DetectedLanguage::Java)
			}
			ProjectType::DotNet => Some(DetectedLanguage::CSharp),
			ProjectType::Unknown => None,
		}
	}

	/// Get display name for this project type
	pub fn display_name(&self) -> &'static str {
		match self {
			ProjectType::RustCargo => "Rust (Cargo)",
			ProjectType::NodeJs => "Node.js",
			ProjectType::Python => "Python",
			ProjectType::GoMod => "Go",
			ProjectType::Gradle => "Java (Gradle)",
			ProjectType::Maven => "Java (Maven)",
			ProjectType::DotNet => ".NET",
			ProjectType::Unknown => "Unknown",
		}
	}
}
