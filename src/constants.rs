use dotenv_codegen::dotenv;

pub const DATABASE_URL: &str = dotenv!("DATABASE_URL");

pub const SERVER_URL: &str = dotenv!("SERVER_URL");
pub const JWT_BASE64_SECRET: &str = dotenv!("JWT_BASE64_SECRET");

pub const TWITCH_CLIENT_ID: &str = dotenv!("TWITCH_CLIENT_ID");
pub const TWITCH_CLIENT_SECRET: &str = dotenv!("TWITCH_CLIENT_SECRET");
pub const TWITCH_CLIENT_USER_LOGIN: &str = dotenv!("TWITCH_CLIENT_USER_LOGIN");

pub const IMAGE_PROXY: &str = dotenv!("IMAGE_PROXY");
