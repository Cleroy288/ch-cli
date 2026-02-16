use std::path::PathBuf;

/// The mode of the picker.
///
/// Represents the current state of the picker:
/// browsing files, symbols, tools, or docs.
#[derive(Debug, Clone, PartialEq)]
pub enum PickerMode {
    /// Not in picker mode
    Inactive,
    /// Browsing filesystem at a directory
    Browse {
        /// The directory being browsed
        dir: PathBuf,
    },
    /// Browsing symbols in a file
    Symbols {
        /// The file whose symbols are shown
        file_path: PathBuf,
        /// Current parent symbol (None = top-level)
        parent: Option<String>,
    },
    /// Browsing available tools
    Tools,
    /// Browsing generated documentation
    DocBrowser,
}
