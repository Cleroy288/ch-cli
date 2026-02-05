/// Domain types module
///
/// This module contains NewTypes and domain-specific types that make illegal states
/// unrepresentable. By using NewTypes, we enforce type safety and make the code's
/// intent clearer.
///
/// # Modules
/// - `constants`: All magic values as typed constants
/// - `cursor`: CursorPosition newtype
/// - `file_ref`: FilePath, FileName, and FileReference newtypes
pub mod constants;
pub mod cursor;
pub mod file_ref;

// Re-export commonly used types for convenience
pub use constants::*;
pub use cursor::CursorPosition;
pub use file_ref::{FileName, FilePath, FileReference};
