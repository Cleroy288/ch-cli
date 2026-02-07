use std::path::PathBuf;

/// NewType for file paths
///
/// Distinguishes file paths from arbitrary strings and enforces
/// valid path representations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePath(pub(crate) PathBuf);

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
