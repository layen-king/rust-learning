use once_cell::sync::Lazy;
use std::collections::HashMap;

static MIME_TYPES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("zip".to_owned(), "application/zip".to_owned());
    m.insert("wps".to_owned(), "application/vnd.ms-works".to_owned());
    m.insert("html".to_owned(), "text/html".to_owned());
    m
});

/// ## 获取mime值
/// ### [mime_type] mime_type的键名
pub fn get_mime_type(mime_type: &str) -> String {
    let value = MIME_TYPES.get(mime_type);
    if let Some(value) = value {
        value.to_owned()
    } else {
        String::from("")
    }
}
