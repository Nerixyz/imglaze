use crate::constants::IMAGE_PROXY;
use std::path::Path;
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

pub fn wrap_proxy(url: &str) -> String {
    format!("{}/?url={}", IMAGE_PROXY, urlencoding::encode(url))
}
