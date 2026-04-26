mod detected_language;
mod discovery;
mod language;
mod types;

// Re-export all public types for backward compatibility
pub use detected_language::DetectedLanguage;
pub use discovery::Crawler;
pub use language::Language;
pub use types::{CrawlStats, CrawlerConfig, FileResult};
