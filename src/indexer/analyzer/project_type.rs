use crate::indexer::crawler::{DetectedLanguage, Language};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectType {
	RustCargo,
	NodeJs,
	Python,
	GoMod,
	Gradle,
	Maven,
	DotNet,
	Unknown,
}

impl ProjectType {
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
