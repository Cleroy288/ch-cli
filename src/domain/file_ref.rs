use std::path::PathBuf;

/// NewType for file paths to distinguish them from arbitrary strings.
/// Enforces that we're always working with valid path representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePath(PathBuf);

impl FilePath {
    /// Create a new FilePath from a PathBuf
    pub fn new(path: PathBuf) -> Self {
        Self(path)
    }

    /// Create a FilePath from a string slice
    pub fn from_string(s: &str) -> Self {
        Self(PathBuf::from(s))
    }

    /// Get the underlying PathBuf
    pub fn as_path_buf(&self) -> &PathBuf {
        &self.0
    }

    /// Convert to a string representation
    pub fn as_string(&self) -> String {
        self.0.to_str().unwrap_or("").to_string()
    }

    /// Get just the filename (last component)
    pub fn file_name(&self) -> Option<String> {
        self.0
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }
}

impl From<PathBuf> for FilePath {
    fn from(path: PathBuf) -> Self {
        Self(path)
    }
}

impl From<String> for FilePath {
    fn from(s: String) -> Self {
        Self(PathBuf::from(s))
    }
}

impl From<&str> for FilePath {
    fn from(s: &str) -> Self {
        Self(PathBuf::from(s))
    }
}

/// NewType for file/folder display names.
/// Represents the short name shown to users (e.g., "main.rs" not "./src/main.rs").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName(String);

impl FileName {
    /// Create a new FileName
    pub fn new(name: String) -> Self {
        Self(name)
    }

    /// Get the underlying string
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned String
    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for FileName {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for FileName {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl AsRef<str> for FileName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Represents a file or folder reference in the input.
/// Uses NewTypes to make the distinction between full paths and display names explicit.
#[derive(Debug, Clone)]
pub struct FileReference {
    /// Start position in input string
    pub start: usize,
    /// End position in input string
    pub end: usize,
    /// Full path of the file/folder (e.g., "./src/main.rs")
    pub full_path: FilePath,
    /// Display name shown in UI (e.g., "main.rs")
    pub display_name: FileName,
    /// Whether it's a directory
    pub is_dir: bool,
}

impl FileReference {
    /// Create a new FileReference
    pub fn new(
        start: usize,
        end: usize,
        full_path: FilePath,
        display_name: FileName,
        is_dir: bool,
    ) -> Self {
        Self {
            start,
            end,
            full_path,
            display_name,
            is_dir,
        }
    }
}
