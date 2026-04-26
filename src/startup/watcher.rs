use std::path::Path;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use crate::fs::FileCache;
use crate::indexer::FileWatcher;

pub use super::watcher_handle::WatcherHandle;

pub(super) const POLL_MS: u64 = 100;

/// Shared watcher message for TUI display
pub type WatcherMsg = Arc<Mutex<Option<String>>>;

/// Spawn background watcher thread
pub fn spawn_watcher(
    project_path: &Path,
    file_cache: FileCache,
    msg: WatcherMsg,
) -> WatcherHandle {
    let stop_flag =
        Arc::new(AtomicBool::new(false));
    let stop_clone = stop_flag.clone();
    let path = project_path.to_path_buf();

    let handle = std::thread::spawn(move || {
        watcher_loop(
            &path, &stop_clone,
            &file_cache, &msg,
        );
    });

    WatcherHandle::new(stop_flag, handle)
}

/// Empty watcher message slot
pub fn new_watcher_msg() -> WatcherMsg {
    Arc::new(Mutex::new(None))
}

pub(super) fn set_watcher_msg(
    msg_slot: &WatcherMsg,
    text: &str,
) {
    if let Ok(mut guard) = msg_slot.lock() {
        *guard = Some(text.to_string());
    }
}

fn init_watcher(
    project_path: &Path,
    msg: &WatcherMsg,
) -> Option<FileWatcher> {
    let mut watcher =
        match FileWatcher::new(project_path) {
            Ok(w) => w,
            Err(err) => {
                set_watcher_msg(msg, &format!(
                    "Watcher error: {}", err
                ));
                return None;
            }
        };

    if let Err(err) = watcher.start() {
        set_watcher_msg(msg, &format!(
            "Watcher start failed: {}", err
        ));
        return None;
    }
    Some(watcher)
}

/// Main watcher loop: init, watch, stop
fn watcher_loop(
    project_path: &Path,
    stop_flag: &Arc<AtomicBool>,
    file_cache: &FileCache,
    msg: &WatcherMsg,
) {
    let Some(mut watcher) =
        init_watcher(project_path, msg)
    else {
        return;
    };

    set_watcher_msg(msg, &format!(
        "Watching {}",
        project_path.display()
    ));

    super::watcher_loop::run_watch_loop(
        &mut watcher, project_path,
        stop_flag, file_cache, msg,
    );

    let _ = watcher.stop();
}
