mod change_detection;
mod file_state;
mod paths;
mod persistence;
pub mod ref_persistence;
pub(crate) mod ref_persistence_helpers;
mod ref_persistence_io;
mod types;
mod types_default;
mod version;

pub use version::INDEX_VERSION;

pub use types::ChangeSet;
pub use types::FileChange;
pub use types::FileState;
pub use types::IndexState;
