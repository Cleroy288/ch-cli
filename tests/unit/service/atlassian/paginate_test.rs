//! Tests for Bitbucket pagination merge logic.
//!
//! Tests the pure functions that merge paginated
//! Bitbucket API responses.

use serde_json::{json, Value};

/// Simulate merge_values by extending values array
fn merge_values(combined: &mut Value, page: &Value) {
	let Some(target) = combined
		.get_mut("values")
		.and_then(Value::as_array_mut)
	else {
		return;
	};
	let Some(source) =
		page.get("values").and_then(Value::as_array)
	else {
		return;
	};
	target.extend(source.iter().cloned());
}

/// Merging two pages combines values arrays
#[test]
fn merge_combines_values() {
	// Arrange
	let mut page1 = json!({
		"values": [{"name": "a"}, {"name": "b"}],
		"next": "http://example.com?page=2",
	});
	let page2 = json!({
		"values": [{"name": "c"}],
	});

	// Act
	merge_values(&mut page1, &page2);

	// Assert
	let values = page1["values"].as_array().unwrap();
	assert_eq!(values.len(), 3);
	assert_eq!(values[2]["name"], "c");
}

/// Merging with empty second page keeps original
#[test]
fn merge_empty_page_keeps_original() {
	// Arrange
	let mut page1 = json!({
		"values": [{"name": "a"}],
	});
	let page2 = json!({"values": []});

	// Act
	merge_values(&mut page1, &page2);

	// Assert
	let values = page1["values"].as_array().unwrap();
	assert_eq!(values.len(), 1);
}

/// Next URL extraction from BB response
#[test]
fn extract_next_url_from_response() {
	// Arrange
	let resp = json!({
		"values": [],
		"next": "https://api.bitbucket.org/2.0/x?page=2",
	});

	// Assert
	let next = resp
		.get("next")
		.and_then(Value::as_str);
	assert_eq!(
		next,
		Some(
			"https://api.bitbucket.org/2.0/x?page=2"
		),
	);
}

/// Missing next field means last page
#[test]
fn no_next_means_last_page() {
	// Arrange
	let resp = json!({"values": []});

	// Assert
	let next = resp
		.get("next")
		.and_then(Value::as_str);
	assert!(next.is_none());
}
