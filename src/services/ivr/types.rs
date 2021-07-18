use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct Moderator {
    pub id: String,
    pub login: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct ModVipResponse {
    pub mods: Vec<Moderator>,
}
