use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FilePath(pub(crate) PathBuf);

impl FilePath {
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    pub fn file_name(&self) -> Option<String> {
        self.0
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    }
}

impl fmt::Display for FilePath {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "{}", self.0.display())
    }
}
