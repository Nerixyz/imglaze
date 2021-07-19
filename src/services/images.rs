use url::Url;

pub fn check_image_url(url: &str) -> anyhow::Result<()> {
    let url = Url::parse(url)?;
    match url.domain() {
        Some("i.imgur.com") | Some("i.nuuls.com") => Ok(()),
        _ => Err(anyhow::Error::msg("Domain not whitelisted")),
    }
}
