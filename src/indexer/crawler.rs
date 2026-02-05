//! File system crawler for the semantic indexer.
//!
//! This module provides efficient directory traversal with:
//! - `.gitignore` support via the `ignore` crate
//! - Parallel file processing via `rayon`
//! - Language-based file filtering

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use crate::indexer::parser::ParseError;
use crate::indexer::Symbol;

/// Supported programming languages for indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Language {
    Rust,
    /// Markdown documentation files
    Markdown,
    // Future: JavaScript, TypeScript, Python, Go, etc.
}

impl Language {
    /// Get file extensions for this language
    pub fn extensions(&self) -> &[&str] {
        match self {
            Language::Rust => &["rs"],
            Language::Markdown => &["md", "txt"],
        }
    }

    /// Detect language from file extension
    pub fn from_extension(ext: &str) -> Option<Language> {
        match ext.to_lowercase().as_str() {
            "rs" => Some(Language::Rust),
            "md" | "txt" => Some(Language::Markdown),
            _ => None,
        }
    }

    /// Detect language from file path
    pub fn from_path(path: &Path) -> Option<Language> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(Language::from_extension)
    }

    /// Get the display name for this language
    pub fn display_name(&self) -> &'static str {
        match self {
            Language::Rust => "Rust",
            Language::Markdown => "Markdown",
        }
    }

    /// Get all supported languages for indexing
    pub fn all_supported() -> &'static [Language] {
        &[Language::Rust, Language::Markdown]
    }
}

/// A language detected in the codebase (may or may not be supported for indexing)
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

/// Statistics about a crawl/index operation
#[derive(Debug, Clone, Default)]
pub struct CrawlStats {
    /// Total files discovered
    pub files_found: usize,
    /// Files successfully parsed
    pub files_parsed: usize,
    /// Files that failed to parse
    pub files_failed: usize,
    /// Total symbols extracted
    pub symbols_found: usize,
    /// Time taken in milliseconds
    pub duration_ms: u64,
}

/// Result of crawling a single file
#[derive(Debug)]
pub struct FileResult {
    /// Path to the file
    pub path: PathBuf,
    /// Symbols found (empty if parsing failed)
    pub symbols: Vec<Symbol>,
    /// Error if parsing failed
    pub error: Option<ParseError>,
}

/// Configuration for the crawler
#[derive(Debug, Clone)]
pub struct CrawlerConfig {
    /// Languages to index
    pub languages: Vec<Language>,
    /// Whether to respect .gitignore
    pub respect_gitignore: bool,
    /// Whether to follow symlinks
    pub follow_symlinks: bool,
    /// Maximum file size to parse (in bytes)
    pub max_file_size: Option<u64>,
    /// Additional patterns to ignore
    pub ignore_patterns: Vec<String>,
    /// Number of threads (0 = auto)
    pub num_threads: usize,
}

impl Default for CrawlerConfig {
    fn default() -> Self {
        Self {
            languages: vec![Language::Rust, Language::Markdown],
            respect_gitignore: true,
            follow_symlinks: false,
            max_file_size: Some(1024 * 1024), // 1MB default
            ignore_patterns: vec![
                "target".to_string(),
                "node_modules".to_string(),
                ".git".to_string(),
            ],
            num_threads: 0, // Auto-detect
        }
    }
}

/// File system crawler for discovering and indexing source files
pub struct Crawler {
    config: CrawlerConfig,
}

impl Crawler {
    /// Create a new crawler with default configuration
    pub fn new() -> Self {
        Self {
            config: CrawlerConfig::default(),
        }
    }

    /// Create a new crawler with custom configuration
    pub fn with_config(config: CrawlerConfig) -> Self {
        Self { config }
    }

    /// Get the configuration
    pub fn config(&self) -> &CrawlerConfig {
        &self.config
    }

    /// Discover all indexable files in a directory
    pub fn discover_files<P: AsRef<Path>>(&self, root: P) -> Vec<PathBuf> {
        let root = root.as_ref();
        let mut files = Vec::new();

        let mut builder = WalkBuilder::new(root);
        builder
            .hidden(true) // Skip hidden files by default
            .git_ignore(self.config.respect_gitignore)
            .git_global(self.config.respect_gitignore)
            .git_exclude(self.config.respect_gitignore)
            .follow_links(self.config.follow_symlinks);

        // Add custom ignore patterns
        for pattern in &self.config.ignore_patterns {
            let _ = builder.add_ignore(Path::new(pattern));
        }

        for entry in builder.build().flatten() {
            let path = entry.path();

            // Skip directories
            if path.is_dir() {
                continue;
            }

            // Check if it's a supported language
            if Language::from_path(path).is_none() {
                continue;
            }

            // Check file size if configured
            if let Some(max_size) = self.config.max_file_size {
                if let Ok(metadata) = path.metadata() {
                    if metadata.len() > max_size {
                        continue;
                    }
                }
            }

            files.push(path.to_path_buf());
        }

        files
    }
}

impl Default for Crawler {
    fn default() -> Self {
        Self::new()
    }
}

