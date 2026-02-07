//! Detected programming languages (supported and unsupported).
//!
//! This module defines languages detected in the codebase, including those
//! not yet supported for indexing.

use std::path::Path;

use super::language::Language;

/// A language detected in the codebase
/// (may or may not be supported for indexing)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DetectedLanguage {
	/// A supported language that can be indexed
	Supported(Language),
	/// JavaScript (not yet supported)
	JavaScript,
	/// TypeScript (not yet supported)
	TypeScript,
	/// Python (not yet supported)
	Python,
	/// Go (not yet supported)
	Go,
	/// Java (not yet supported)
	Java,
	/// C# (not yet supported)
	CSharp,
	/// C++ (not yet supported)
	Cpp,
	/// C (not yet supported)
	C,
	/// Ruby (not yet supported)
	Ruby,
	/// PHP (not yet supported)
	PHP,
	/// Swift (not yet supported)
	Swift,
	/// Kotlin (not yet supported)
	Kotlin,
}

impl DetectedLanguage {
	/// Detect language from file extension (includes unsupported languages)
	pub fn from_extension(ext: &str) -> Option<Self> {
		match ext.to_lowercase().as_str() {
			// Supported languages
			"rs" => Some(DetectedLanguage::Supported(Language::Rust)),
			"md" | "txt" => Some(DetectedLanguage::Supported(Language::Markdown)),
			// Unsupported but recognized languages
			"js" | "mjs" | "cjs" | "jsx" => Some(DetectedLanguage::JavaScript),
			"ts" | "tsx" | "mts" | "cts" => Some(DetectedLanguage::TypeScript),
			"py" | "pyw" | "pyi" => Some(DetectedLanguage::Python),
			"go" => Some(DetectedLanguage::Go),
			"java" => Some(DetectedLanguage::Java),
			"cs" => Some(DetectedLanguage::CSharp),
			"cpp" | "cc" | "cxx" | "hpp" | "hxx" | "h++" => Some(DetectedLanguage::Cpp),
			"c" | "h" => Some(DetectedLanguage::C),
			"rb" | "rake" => Some(DetectedLanguage::Ruby),
			"php" => Some(DetectedLanguage::PHP),
			"swift" => Some(DetectedLanguage::Swift),
			"kt" | "kts" => Some(DetectedLanguage::Kotlin),
			_ => None,
		}
	}

	/// Detect language from file path
	pub fn from_path(path: &Path) -> Option<Self> {
		path.extension()
			.and_then(|ext| ext.to_str())
			.and_then(Self::from_extension)
	}

	/// Get the display name for this language
	pub fn display_name(&self) -> &'static str {
		match self {
			DetectedLanguage::Supported(lang) => lang.display_name(),
			DetectedLanguage::JavaScript => "JavaScript",
			DetectedLanguage::TypeScript => "TypeScript",
			DetectedLanguage::Python => "Python",
			DetectedLanguage::Go => "Go",
			DetectedLanguage::Java => "Java",
			DetectedLanguage::CSharp => "C#",
			DetectedLanguage::Cpp => "C++",
			DetectedLanguage::C => "C",
			DetectedLanguage::Ruby => "Ruby",
			DetectedLanguage::PHP => "PHP",
			DetectedLanguage::Swift => "Swift",
			DetectedLanguage::Kotlin => "Kotlin",
		}
	}

	/// Check if this language is supported for indexing
	pub fn is_supported(&self) -> bool {
		matches!(self, DetectedLanguage::Supported(_))
	}

	/// Get the underlying supported language if this is a supported language
	pub fn as_supported(&self) -> Option<Language> {
		match self {
			DetectedLanguage::Supported(lang) => Some(*lang),
			_ => None,
		}
	}
}
