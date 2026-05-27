use issue_provider_core::{PageCursor, PageRequest};

pub(crate) fn start_at(page: &Option<PageRequest>) -> u32 {
    page.as_ref()
        .and_then(PageRequest::after)
        .and_then(|cursor| cursor.as_str().parse().ok())
        .unwrap_or(0)
}

pub(crate) fn max_results(page: &Option<PageRequest>) -> u32 {
    page.as_ref().and_then(PageRequest::limit).unwrap_or(50)
}

pub(crate) fn offset_cursor(
    is_last: Option<bool>,
    start: u32,
    count: u32,
    max: u32,
) -> Option<PageCursor> {
    let last = is_last.unwrap_or(count < max);
    if last || count == 0 {
        None
    } else {
        Some(PageCursor::make((start + count).to_string()))
    }
}

pub(crate) fn adf_text(value: &serde_json::Value) -> String {
    let mut out = String::new();
    render_node(value, &mut out);
    out.trim().to_string()
}

fn render_node(node: &serde_json::Value, out: &mut String) {
    match node
        .get("type")
        .and_then(|kind| kind.as_str())
        .unwrap_or("")
    {
        "text" => {
            let text = node
                .get("text")
                .and_then(|text| text.as_str())
                .unwrap_or("");
            out.push_str(&apply_marks(node, text));
        }
        "hardBreak" => out.push('\n'),
        "paragraph" => {
            render_children(node, out);
            out.push_str("\n\n");
        }
        "heading" => {
            let level = node
                .get("attrs")
                .and_then(|attrs| attrs.get("level"))
                .and_then(serde_json::Value::as_u64)
                .unwrap_or(1)
                .clamp(1, 6);
            for _ in 0..level {
                out.push('#');
            }
            out.push(' ');
            render_children(node, out);
            out.push_str("\n\n");
        }
        "codeBlock" => {
            out.push_str("```\n");
            render_children(node, out);
            out.push_str("\n```\n\n");
        }
        "blockquote" => {
            out.push_str("> ");
            render_children(node, out);
            out.push_str("\n\n");
        }
        "rule" => out.push_str("---\n\n"),
        "bulletList" => render_list(node, out, None),
        "orderedList" => render_list(node, out, Some(1)),
        _ => render_children(node, out),
    }
}

fn render_children(node: &serde_json::Value, out: &mut String) {
    if let Some(content) = node.get("content").and_then(|content| content.as_array()) {
        for child in content {
            render_node(child, out);
        }
    }
}

fn render_list(node: &serde_json::Value, out: &mut String, ordered: Option<u32>) {
    if let Some(items) = node.get("content").and_then(|content| content.as_array()) {
        for (index, item) in items.iter().enumerate() {
            match ordered {
                Some(start) => out.push_str(&format!("{}. ", start + index as u32)),
                None => out.push_str("- "),
            }
            let mut inner = String::new();
            render_children(item, &mut inner);
            out.push_str(inner.trim());
            out.push('\n');
        }
        out.push('\n');
    }
}

fn apply_marks(node: &serde_json::Value, text: &str) -> String {
    let mut value = text.to_string();
    let Some(marks) = node.get("marks").and_then(|marks| marks.as_array()) else {
        return value;
    };
    for mark in marks {
        value = match mark
            .get("type")
            .and_then(|kind| kind.as_str())
            .unwrap_or("")
        {
            "code" => format!("`{value}`"),
            "strong" => format!("**{value}**"),
            "em" => format!("*{value}*"),
            "strike" => format!("~~{value}~~"),
            "link" => mark
                .get("attrs")
                .and_then(|attrs| attrs.get("href"))
                .and_then(serde_json::Value::as_str)
                .map(|href| format!("[{value}]({href})"))
                .unwrap_or(value),
            _ => value,
        };
    }
    value
}

pub(crate) fn to_adf(body: &str) -> serde_json::Value {
    serde_json::json!({
        "type": "doc",
        "version": 1,
        "content": [
            { "type": "paragraph", "content": [{ "type": "text", "text": body }] }
        ]
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_cursor_stops_on_last_or_short_page() {
        assert_eq!(offset_cursor(Some(true), 0, 50, 50), None);
        assert_eq!(offset_cursor(None, 0, 10, 50), None);
        assert_eq!(
            offset_cursor(Some(false), 50, 50, 50),
            Some(PageCursor::make("100"))
        );
        assert_eq!(offset_cursor(None, 0, 50, 50), Some(PageCursor::make("50")));
    }

    #[test]
    fn adf_text_extracts_nested_paragraphs() {
        let doc = to_adf("hello world");
        assert_eq!(adf_text(&doc), "hello world");
    }

    #[test]
    fn adf_text_joins_multiple_paragraphs() {
        let doc = serde_json::json!({
            "type": "doc",
            "content": [
                { "type": "paragraph", "content": [{ "type": "text", "text": "first" }] },
                { "type": "paragraph", "content": [{ "type": "text", "text": "second" }] }
            ]
        });
        assert_eq!(adf_text(&doc), "first\n\nsecond");
    }

    #[test]
    fn adf_text_renders_code_block_and_marks() {
        let doc = serde_json::json!({
            "type": "doc",
            "content": [
                { "type": "codeBlock", "content": [{ "type": "text", "text": "SELECT 1" }] },
                { "type": "paragraph", "content": [{ "type": "text", "text": "bold", "marks": [{ "type": "strong" }] }] }
            ]
        });
        assert_eq!(adf_text(&doc), "```\nSELECT 1\n```\n\n**bold**");
    }
}
