//! Tests for retrieval::daemon::protocol

use rustean::retrieval::daemon::protocol::{
	deserialize_request, deserialize_response,
	serialize_request, serialize_response,
	CachedSearchResult, DaemonRequest, DaemonResponse,
};

#[test]
fn test_request_serialization() {
	let req = DaemonRequest::Embed {
		texts: vec!["hello".to_string()],
	};
	let bytes = serialize_request(&req).unwrap();
	let parsed: DaemonRequest =
		deserialize_request(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonRequest::Embed { texts } => {
			assert_eq!(texts, vec!["hello"]);
		}
		_ => panic!("wrong variant"),
	}
}

#[test]
fn test_response_serialization() {
	let resp =
		DaemonResponse::Embeddings(vec![vec![0.1, 0.2, 0.3]]);
	let bytes = serialize_response(&resp).unwrap();
	let parsed: DaemonResponse =
		deserialize_response(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonResponse::Embeddings(vecs) => {
			assert_eq!(vecs.len(), 1);
			assert_eq!(vecs[0], vec![0.1, 0.2, 0.3]);
		}
		_ => panic!("wrong variant"),
	}
}

#[test]
fn test_index_project_request_serialization() {
	let req = DaemonRequest::IndexProject {
		project_path: "/home/user/project".to_string(),
		force: false,
	};
	let bytes = serialize_request(&req).unwrap();
	let parsed: DaemonRequest =
		deserialize_request(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonRequest::IndexProject {
			project_path,
			force,
		} => {
			assert_eq!(project_path, "/home/user/project");
			assert!(!force);
		}
		_ => panic!("wrong variant"),
	}
}

#[test]
fn test_project_indexed_response() {
	let resp = DaemonResponse::ProjectIndexed {
		symbol_count: 100,
		cached: true,
		index_time_ms: 0,
	};
	let bytes = serialize_response(&resp).unwrap();
	let parsed: DaemonResponse =
		deserialize_response(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonResponse::ProjectIndexed {
			symbol_count,
			cached,
			index_time_ms,
		} => {
			assert_eq!(symbol_count, 100);
			assert!(cached);
			assert_eq!(index_time_ms, 0);
		}
		_ => panic!("wrong variant"),
	}
}

#[test]
fn test_cached_search_result_serialization() {
	let result = CachedSearchResult {
		symbol_name: "test_function".to_string(),
		symbol_kind: "function".to_string(),
		file_path: "src/main.rs".to_string(),
		line: 42,
		score: 0.95,
		rerank_score: Some(0.87),
	};
	let resp = DaemonResponse::SearchResults(vec![result]);
	let bytes = serialize_response(&resp).unwrap();
	let parsed: DaemonResponse =
		deserialize_response(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonResponse::SearchResults(results) => {
			assert_eq!(results.len(), 1);
			assert_eq!(
				results[0].symbol_name,
				"test_function"
			);
			assert_eq!(results[0].line, 42);
		}
		_ => panic!("wrong variant"),
	}
}

#[test]
fn test_project_cache_status_serialization() {
	let resp = DaemonResponse::ProjectCacheStatus {
		cached: true,
		symbol_count: 500,
		last_indexed: 1706745600,
	};
	let bytes = serialize_response(&resp).unwrap();
	let parsed: DaemonResponse =
		deserialize_response(&bytes[..bytes.len() - 1])
			.unwrap();

	match parsed {
		DaemonResponse::ProjectCacheStatus {
			cached,
			symbol_count,
			last_indexed,
		} => {
			assert!(cached);
			assert_eq!(symbol_count, 500);
			assert_eq!(last_indexed, 1706745600);
		}
		_ => panic!("wrong variant"),
	}
}
