//! Codebase language analyzer.
//!
//! Detects the primary programming language(s) used in a codebase
//! by analyzing file extensions and project configuration files.

use std::collections::HashMap;
use std::path::Path;

use ignore::WalkBuilder;

use crate::indexer::crawler::{DetectedLanguage, Language};

/// Result of analyzing a codebase's language composition
#[derive(Debug, Clone)]
pub struct CodebaseAnalysis {
    /// Primary detected language (most files)
    pub primary_language: Option<DetectedLanguage>,
    /// All languages detected with file counts
    pub language_counts: HashMap<DetectedLanguage, usize>,
    /// Total source files analyzed
    pub total_source_files: usize,
    /// Whether the primary language is supported for indexing
    pub is_primary_supported: bool,
    /// Detected project type (if identifiable)
    pub project_type: Option<ProjectType>,
}

impl CodebaseAnalysis {
    /// Get supported languages in this codebase (sorted by file count, descending)
    pub fn supported_languages(&self) -> Vec<(Language, usize)> {
        let mut supported: Vec<(Language, usize)> = self
            .language_counts
            .iter()
            .filter_map(|(lang, count)| lang.as_supported().map(|l| (l, *count)))
            .collect();
        supported.sort_by(|a, b| b.1.cmp(&a.1));
        supported
    }

    /// Get unsupported languages in this codebase (sorted by file count, descending)
    pub fn unsupported_languages(&self) -> Vec<(DetectedLanguage, usize)> {
        let mut unsupported: Vec<(DetectedLanguage, usize)> = self
            .language_counts
            .iter()
            .filter(|(lang, _)| !lang.is_supported())
            .map(|(lang, count)| (*lang, *count))
            .collect();
        unsupported.sort_by(|a, b| b.1.cmp(&a.1));
        unsupported
    }

    /// Check if the codebase has any supported files for indexing
    pub fn has_supported_files(&self) -> bool {
        self.language_counts
            .iter()
            .any(|(lang, count)| lang.is_supported() && *count > 0)
    }

    /// Get the total count of supported files
    pub fn supported_file_count(&self) -> usize {
        self.language_counts
            .iter()
            .filter(|(lang, _)| lang.is_supported())
            .map(|(_, count)| count)
            .sum()
    }
}

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
            ProjectType::RustCargo => Some(DetectedLanguage::Supported(Language::Rust)),
            ProjectType::NodeJs => Some(DetectedLanguage::JavaScript),
            ProjectType::Python => Some(DetectedLanguage::Python),
            ProjectType::GoMod => Some(DetectedLanguage::Go),
            ProjectType::Gradle | ProjectType::Maven => Some(DetectedLanguage::Java),
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

/// Analyzes a codebase to determine its primary programming language
pub struct CodebaseAnalyzer {
    /// Maximum files to scan (for performance on large repos)
    max_files: usize,
}

impl CodebaseAnalyzer {
    /// Create a new analyzer with default settings
    pub fn new() -> Self {
        Self {
            max_files: 10_000, // Reasonable limit for quick analysis
        }
    }

    /// Create an analyzer with custom max file limit
    pub fn with_max_files(max_files: usize) -> Self {
        Self { max_files }
    }

    /// Analyze a codebase and return language composition
    pub fn analyze<P: AsRef<Path>>(&self, root: P) -> CodebaseAnalysis {
        let root = root.as_ref();
        let mut language_counts: HashMap<DetectedLanguage, usize> = HashMap::new();
        let mut total_source_files = 0;

        // First, detect project type from config files
        let project_type = self.detect_project_type(root);

        // Walk the directory tree
        let walker = WalkBuilder::new(root)
            .hidden(true)
            .git_ignore(true)
            .git_global(true)
            .git_exclude(true)
            .build();

        for entry in walker.flatten().take(self.max_files) {
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Skip common non-source directories even if not in .gitignore
            if let Some(path_str) = path.to_str() {
                if path_str.contains("/target/")
                    || path_str.contains("/node_modules/")
                    || path_str.contains("/.git/")
                    || path_str.contains("/vendor/")
                    || path_str.contains("/build/")
                    || path_str.contains("/dist/")
                    || path_str.contains("/__pycache__/")
                {
                    continue;
                }
            }

            // Detect language from extension
            if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                if let Some(lang) = DetectedLanguage::from_extension(ext) {
                    *language_counts.entry(lang).or_insert(0) += 1;
                    total_source_files += 1;
                }
            }
        }

        // Determine primary language (most files)
        let primary_language = language_counts
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(lang, _)| *lang);

        let is_primary_supported = primary_language
            .map(|lang| lang.is_supported())
            .unwrap_or(false);

        CodebaseAnalysis {
            primary_language,
            language_counts,
            total_source_files,
            is_primary_supported,
            project_type,
        }
    }

    /// Detect project type from configuration files
    fn detect_project_type(&self, root: &Path) -> Option<ProjectType> {
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
        if root.join("build.gradle").exists() || root.join("build.gradle.kts").exists() {
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
}

impl Default for CodebaseAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
