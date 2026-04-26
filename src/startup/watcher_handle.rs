use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Handle to the background watcher thread
pub struct WatcherHandle {
    /// flag to signal the watcher to stop
    stop_flag: Arc<AtomicBool>,
    /// handle to the watcher thread
    thread_handle:
        Option<std::thread::JoinHandle<()>>,
}

impl WatcherHandle {
    pub(super) fn new(
        stop_flag: Arc<AtomicBool>,
        thread_handle: std::thread::JoinHandle<()>,
    ) -> Self {
        Self {
            stop_flag,
            thread_handle: Some(thread_handle),
        }
    }

    pub fn stop(mut self) {
        self.shutdown();
    }

    /// Signal stop and join the thread
    fn shutdown(&mut self) {
        self.stop_flag
            .store(true, Ordering::SeqCst);
        if let Some(h) =
            self.thread_handle.take()
        {
            let _ = h.join();
        }
    }
}

impl Drop for WatcherHandle {
    fn drop(&mut self) {
        self.shutdown();
    }
}
