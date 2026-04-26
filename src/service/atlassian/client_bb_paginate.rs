use serde_json::Value;

use crate::domain::errors::atlassian::AtlassianResult;

use super::client_bb::BbClient;

const MAX_PAGES: usize = 20;

pub fn get_all_pages(
	client: &BbClient,
	path: &str,
) -> AtlassianResult<String> {
	let first = client.get(path)?;
	let mut combined: Value =
		serde_json::from_str(&first)?;

	let mut pages = 1;
	while let Some(next_url) = extract_next(&combined)
	{
		if pages >= MAX_PAGES {
			break;
		}
		let body = client.fetch_url(&next_url)?;
		let page: Value =
			serde_json::from_str(&body)?;
		merge_values(&mut combined, &page);
		update_next(&mut combined, &page);
		pages += 1;
	}
	remove_pagination_fields(&mut combined);
	Ok(serde_json::to_string(&combined)?)
}

fn extract_next(json: &Value) -> Option<String> {
	json.get("next")
		.and_then(Value::as_str)
		.map(String::from)
}

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

fn update_next(
    combined: &mut Value,
    page: &Value,
) {
    let Some(obj) = combined.as_object_mut()
    else {
        return;
    };
    if let Some(url) = page.get("next") {
        obj.insert("next".into(), url.clone());
    } else {
        obj.remove("next");
    }
}

fn remove_pagination_fields(json: &mut Value) {
	if let Some(obj) = json.as_object_mut() {
		obj.remove("next");
		obj.remove("previous");
		obj.remove("page");
		obj.remove("pagelen");
		if let Some(values) = obj.get("values") {
			let count = values
				.as_array()
				.map(Vec::len)
				.unwrap_or(0);
			obj.insert("size".into(), count.into());
		}
	}
}
