/// Domain types module
///
/// This module contains NewTypes, domain-specific types,
/// and error types that make illegal states
/// unrepresentable.
///
/// # Modules
/// - `constants`: All magic values as typed constants
/// - `cursor`: CursorPosition newtype
/// - `errors`: Per-domain error enums
/// - `file_ref`: FilePath, FileName, and FileReference
pub mod constants;
pub mod cursor;
mod cursor_conversions;
mod cursor_ops;
pub mod errors;
pub mod file_name;
mod file_name_conversions;
pub mod file_path;
mod file_path_conversions;
pub mod file_ref;

// Re-export commonly used types for convenience
pub use constants::*;
pub use cursor::CursorPosition;
pub use errors::*;
pub use file_name::FileName;
pub use file_path::FilePath;
pub use file_ref::FileReference;
