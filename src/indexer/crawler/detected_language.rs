use std::path::Path;

use super::language::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectedLanguage {
	Supported(Language),
	JavaScript,
	TypeScript,
	Python,
	GoLang,
	Java,
	CSharp,
	Cpp,
	CLang,
	Ruby,
	Php,
	Swift,
	Kotlin,
}

impl DetectedLanguage {
	pub fn from_extension(ext: &str) -> Option<Self> {
		match ext.to_lowercase().as_str() {
			"rs" => Some(DetectedLanguage::Supported(Language::Rust)),
			"md" | "txt" => Some(DetectedLanguage::Supported(Language::Markdown)),
			"js" | "mjs" | "cjs" | "jsx" => Some(DetectedLanguage::JavaScript),
			"ts" | "tsx" | "mts" | "cts" => Some(DetectedLanguage::TypeScript),
			"py" | "pyw" | "pyi" => Some(DetectedLanguage::Python),
			"go" => Some(DetectedLanguage::GoLang),
			"java" => Some(DetectedLanguage::Java),
			"cs" => Some(DetectedLanguage::CSharp),
			"cpp" | "cc" | "cxx" | "hpp" | "hxx"
		| "h++" => Some(DetectedLanguage::Cpp),
			"c" | "h" => Some(DetectedLanguage::CLang),
			"rb" | "rake" => Some(DetectedLanguage::Ruby),
			"php" => Some(DetectedLanguage::Php),
			"swift" => Some(DetectedLanguage::Swift),
			"kt" | "kts" => Some(DetectedLanguage::Kotlin),
			_ => None,
		}
	}

	pub fn from_path(path: &Path) -> Option<Self> {
		path.extension()
			.and_then(|ext| ext.to_str())
			.and_then(Self::from_extension)
	}

	pub fn display_name(&self) -> &'static str {
		match self {
			DetectedLanguage::Supported(lang) => lang.display_name(),
			DetectedLanguage::JavaScript => "JavaScript",
			DetectedLanguage::TypeScript => "TypeScript",
			DetectedLanguage::Python => "Python",
			DetectedLanguage::GoLang => "Go",
			DetectedLanguage::Java => "Java",
			DetectedLanguage::CSharp => "C#",
			DetectedLanguage::Cpp => "C++",
			DetectedLanguage::CLang => "C",
			DetectedLanguage::Ruby => "Ruby",
			DetectedLanguage::Php => "Php",
			DetectedLanguage::Swift => "Swift",
			DetectedLanguage::Kotlin => "Kotlin",
		}
	}

	pub fn is_supported(&self) -> bool {
		matches!(self, DetectedLanguage::Supported(_))
	}

	pub fn as_supported(&self) -> Option<Language> {
		match self {
			DetectedLanguage::Supported(lang) => Some(*lang),
			_ => None,
		}
	}
}
