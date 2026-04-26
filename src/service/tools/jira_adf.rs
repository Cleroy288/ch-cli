use serde_json::Value;

pub fn adf_to_text(adf: &Value) -> String {
    let mut buf = String::new();
    collect_text(adf, &mut buf);
    buf.trim().to_string()
}

fn collect_text(
    node: &Value,
    buf: &mut String,
) {
    let node_type = node
        .get("type")
        .and_then(Value::as_str)
        .unwrap_or("");
    if node_type == "hardBreak" {
        buf.push('\n');
        return;
    }
    append_text_value(node, buf);
    visit_children(node, buf);
    if is_block_node(node_type) {
        buf.push('\n');
    }
}

fn append_text_value(
    node: &Value,
    buf: &mut String,
) {
    if let Some(text) =
        node.get("text").and_then(Value::as_str)
    {
        buf.push_str(text);
    }
}

fn visit_children(
    node: &Value,
    buf: &mut String,
) {
    let Some(arr) = node
        .get("content")
        .and_then(Value::as_array)
    else {
        return;
    };
    for child in arr {
        collect_text(child, buf);
    }
}

fn is_block_node(node_type: &str) -> bool {
    matches!(
        node_type,
        "paragraph"
            | "heading"
            | "listItem"
            | "blockquote"
            | "codeBlock"
            | "rule"
    )
}
