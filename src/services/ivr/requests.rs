use crate::services::ivr::types::{ModVipResponse, Moderator};
use anyhow::Result as AnyResult;
use futures::TryFutureExt;
use lazy_static::lazy_static;
use reqwest::{Client, IntoUrl, Response};
use serde::de::DeserializeOwned;

lazy_static! {
    static ref IVR_CLIENT: Client = Client::builder()
        .user_agent(format!(
            "Imglaze/{} github.com/Nerixyz/imglaze",
            env!("CARGO_PKG_VERSION")
        ))
        .build()
        .unwrap();
}

pub async fn get_mods(login: &str) -> AnyResult<Vec<Moderator>> {
    let modvips: ModVipResponse = ivr_get(format!(
        "https://api.ivr.fi/v2/twitch/modvip/{}",
        urlencoding::encode(login)
    ))
    .await?;
    Ok(modvips.mods)
}

async fn ivr_get<T, U>(url: U) -> AnyResult<T>
where
    T: DeserializeOwned,
    U: IntoUrl,
{
    Ok(IVR_CLIENT.get(url).send().and_then(Response::json).await?)
}
