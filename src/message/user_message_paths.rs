//! Path extraction methods for UserMessage.

use crate::message::MessageSegment;

use super::UserMessage;

impl UserMessage {
	/// Extract paths based on a predicate function.
	///
	/// This is a generic helper that eliminates code
	/// duplication between file_paths() and folder_paths().
	fn extract_paths<F>(&self, predicate: F) -> Vec<String>
	where
		F: Fn(&MessageSegment) -> Option<String>,
	{
		self.segments.iter().filter_map(predicate).collect()
	}

	/// Get all file paths referenced in this message
	pub fn file_paths(&self) -> Vec<String> {
		self.extract_paths(|s| {
			if let MessageSegment::FileReference {
				full_path,
				..
			} = s
			{
				Some(full_path.clone())
			} else {
				None
			}
		})
	}

	/// Get all folder paths referenced in this message
	pub fn folder_paths(&self) -> Vec<String> {
		self.extract_paths(|s| {
			if let MessageSegment::FolderReference {
				full_path,
				..
			} = s
			{
				Some(full_path.clone())
			} else {
				None
			}
		})
	}
}
