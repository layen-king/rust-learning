use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
enum CustomError {
    #[error("unknown type error")]
    UnkowTypeError(String),
}

static MIME_TYPES: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("zip".to_owned(), "application/zip".to_owned());
    m.insert("wps".to_owned(), "application/vnd.ms-works".to_owned());
    m.insert("html".to_owned(), "text/html".to_owned());
    m.insert("txt".to_owned(), "text/plain".to_owned());
    m
});

pub fn get_mime_type(mime_type: &str) -> Result<String> {
    let value = MIME_TYPES.get(mime_type);
    match value {
        Some(value) => Ok(value.to_owned()),
        None => Err(anyhow!(format!("Unknown Type Error: {}", mime_type))),
    }
}
