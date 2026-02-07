/// NewType for file/folder display names
///
/// Represents the short name shown to users
/// (e.g., "main.rs" not "./src/main.rs").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileName(pub(crate) String);

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
