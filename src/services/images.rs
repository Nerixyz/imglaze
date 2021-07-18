use url::Url;

pub fn check_image_url(url: &str) -> anyhow::Result<bool> {
    let url = Url::parse(url)?;
    Ok(match url.domain() {
        Some("i.imgur.com") | Some("i.nuuls.com") => true,
        _ => false,
    })
}
