use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use super::language::Language;
use super::types::CrawlerConfig;

pub struct Crawler {
	config: CrawlerConfig,
}

impl Crawler {
	pub fn new() -> Self {
		Self {
			config: CrawlerConfig::default(),
		}
	}

	pub fn with_config(config: CrawlerConfig) -> Self {
		Self { config }
	}

	pub fn config(&self) -> &CrawlerConfig {
		&self.config
	}

	pub fn discover_files<P: AsRef<Path>>(
		&self,
		root: P,
	) -> Vec<PathBuf> {
		let root = root.as_ref();
		let builder = self.build_walker(root);

		builder
			.build()
			.flatten()
			.filter_map(|entry| {
				let path = entry.path();
				if path.is_dir() { return None; }
				Language::from_path(path)?;
				if self.exceeds_max_size(path) { return None; }
				Some(path.to_path_buf())
			})
			.collect()
	}

	fn build_walker(&self, root: &Path) -> WalkBuilder {
		let mut builder = WalkBuilder::new(root);
		builder
			.hidden(true)
			.git_ignore(self.config.respect_gitignore)
			.git_global(self.config.respect_gitignore)
			.git_exclude(self.config.respect_gitignore)
			.follow_links(self.config.follow_symlinks);

		for pattern in &self.config.ignore_patterns {
			let _ = builder.add_ignore(Path::new(pattern));
		}
		builder
	}

	fn exceeds_max_size(
		&self,
		path: &Path,
	) -> bool {
		let Some(max) = self.config.max_file_size
		else {
			return false;
		};
		path.metadata()
			.map(|meta| meta.len() > max)
			.unwrap_or(false)
	}
}

impl Default for Crawler {
	fn default() -> Self {
		Self::new()
	}
}
