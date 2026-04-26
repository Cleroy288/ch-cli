use std::sync::atomic::Ordering;

use crate::indexer::{
    IndexManager, IndexManagerResult, IndexResult,
};

use super::progress_shared::{
    ProgressState, build_progress_callback,
};

/// Thread handle type for indexing operations
pub(crate) type IndexHandle =
    std::thread::JoinHandle<
        IndexManagerResult<IndexResult>,
    >;

/// Spawn an indexing thread with progress callbacks.
///
/// Builds a fully-configured IndexManager and runs
/// `index_project(".")` in a background thread.
pub(crate) fn spawn_index_thread(
    state: &ProgressState,
) -> IndexHandle {
    let callback = build_progress_callback(state);
    let done_clone = state.done.clone();

    std::thread::spawn(move || {
        let manager = IndexManager::new()
            .with_persistence()
            .with_semantic_analysis()
            .with_reference_extraction()
            .on_progress(callback);

        let result = manager.index_project(".");
        done_clone.store(true, Ordering::SeqCst);
        result
    })
}
