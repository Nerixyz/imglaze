use std::{ffi::OsStr, path::Path};
use url::Url;

pub fn check_image_url(url: &str) -> anyhow::Result<()> {
    let url = Url::parse(url)?;
    let path = Path::new(url.path());
    match path.extension().and_then(|s| s.to_str()) {
        Some("png") | Some("jpg") | Some("jpeg") | Some("webp") | Some("avif") | Some("jxl")
        | Some("bmp") | Some("svg") => Ok(()),
        _ => Err(anyhow::Error::msg("Extension not found or whitelisted")),
    }
}
