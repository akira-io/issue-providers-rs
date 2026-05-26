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
    collect_text(value, &mut out);
    out.trim().to_string()
}

fn collect_text(value: &serde_json::Value, out: &mut String) {
    if let Some(text) = value.get("text").and_then(|text| text.as_str()) {
        out.push_str(text);
    }
    if let Some(content) = value.get("content").and_then(|content| content.as_array()) {
        for child in content {
            collect_text(child, out);
        }
        if value.get("type").and_then(|kind| kind.as_str()) == Some("paragraph") {
            out.push('\n');
        }
    }
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
        assert_eq!(offset_cursor(Some(false), 50, 50, 50), Some(PageCursor::make("100")));
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
        assert_eq!(adf_text(&doc), "first\nsecond");
    }
}
