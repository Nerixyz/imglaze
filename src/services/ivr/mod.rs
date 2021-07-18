pub mod types;

mod requests;

pub async fn check_mod(channel_login: &str, target_login: &str) -> bool {
    match requests::get_mods(channel_login).await {
        Ok(mods) => mods.iter().any(|m| m.login == target_login),
        Err(_) => false,
    }
}
