use crate::{
    actors::irc::{
        messages::{JoinMessage, PartMessage},
        IrcActor,
    },
    constants::{SERVER_URL, TWITCH_CLIENT_ID, TWITCH_CLIENT_SECRET},
    models::{users, users::User},
    services::jwt::{encode_jwt, JwtClaims},
};
use actix::Addr;
use actix_web::{cookie::CookieBuilder, delete, get, web, HttpResponse, Result};
use errors::redirect_error::RedirectError;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use time::{Duration, OffsetDateTime};
use twitch_api2::twitch_oauth2::{
    client::reqwest_http_client, tokens::UserTokenBuilder, ClientId, ClientSecret, CsrfToken,
    RedirectUrl, TwitchToken, UserToken,
};

#[derive(Deserialize)]
#[non_exhaustive]
struct TwitchCallbackQuery {
    code: Option<String>,
    scope: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

#[get("/twitch-callback")]
async fn twitch_callback(
    pool: web::Data<PgPool>,
    irc: web::Data<Addr<IrcActor>>,
    query: web::Query<TwitchCallbackQuery>,
) -> Result<HttpResponse> {
    let query = query.into_inner();
    let (code, scope) = match (query.code, query.scope) {
        (Some(code), Some(scope)) => (code, scope),
        _ => {
            log::info!("{:?} {:?}", query.error, query.error_description);
            return Err(RedirectError::new(
                "/failed-auth",
                query.error_description.or(query.error),
            )
            .into());
        }
    };

    let mut builder = UserTokenBuilder::new(
        ClientId::new(TWITCH_CLIENT_ID.to_string()),
        ClientSecret::new(TWITCH_CLIENT_SECRET.to_string()),
        RedirectUrl::new(format!("{}/api/v1/auth/twitch-callback", SERVER_URL))
            .expect("Invalid redirect-url"),
    )
    .expect("Invalid url");

    builder.set_csrf(CsrfToken::new("".to_string()));

    let user_token = builder
        .get_user_token(reqwest_http_client, "", &code)
        .await
        .map_err(|_| RedirectError::new("/failed-auth", Some("Could not get token")))?;

    let refresh_token = user_token
        .refresh_token
        .ok_or_else(|| RedirectError::<&str, &str>::simple("/failed-auth"))?;

    let user = User {
        id: user_token.user_id.clone(),
        refresh_token: refresh_token.secret().clone(),
        access_token: user_token.access_token.secret().clone(),
        scopes: scope,
        name: user_token.login.clone(),
    };

    user.create(&pool)
        .await
        .map_err(|_| RedirectError::new("/failed-auth", Some("Could not create user")))?;

    log::info!("AUTH: Registered {}", user.name);

    // join the user's channel
    irc.do_send(JoinMessage(user.name));

    let token = encode_jwt(&JwtClaims::new(user_token.user_id.clone()))
        .map_err(|_| RedirectError::new("/failed-auth", Some("Could not encode")))?;
    Ok(HttpResponse::Found()
        .append_header(("location", "/"))
        .cookie(
            CookieBuilder::new("auth_token", token)
                .expires(Some(OffsetDateTime::now_utc() + Duration::days(365)))
                .path("/")
                .finish(),
        )
        .finish())
}

#[derive(Serialize)]
struct TwitchOAuthParams {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: String,
}

#[derive(Serialize)]
struct TwitchAuthUrlResponse {
    url: String,
}

#[get("/twitch-auth")]
fn redirect_to_twitch_auth() -> HttpResponse {
    let params = TwitchOAuthParams {
        client_id: TWITCH_CLIENT_ID.to_string(),
        redirect_uri: format!("{}/api/v1/auth/twitch-callback", SERVER_URL),
        response_type: "code".to_string(),
        scope: "".to_string(),
    };
    let url = format!(
        "https://id.twitch.tv/oauth2/authorize?{}",
        serde_qs::to_string(&params).expect("Failed to serialize")
    );

    HttpResponse::Found()
        .append_header(("location", url))
        .finish()
}

#[delete("")]
async fn revoke(
    claims: JwtClaims,
    pool: web::Data<PgPool>,
    irc: web::Data<Addr<IrcActor>>,
) -> Result<HttpResponse> {
    let user = claims.get_user(&pool).await?;
    let user_name = user.name.clone();
    let token: UserToken = user.into();

    if let Err(e) = token.revoke_token(reqwest_http_client).await {
        // we don't return the error, so me make sure everything is cleaned up
        log::warn!("Revoke token error: {}", e);
    }

    log::info!("AUTH: Revoked {}", user_name);

    irc.do_send(PartMessage(user_name));

    // here we can return the error as there's no work afterwards
    users::delete(claims.user_id(), &pool).await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn init_auth_routes(config: &mut web::ServiceConfig) {
    config
        .service(redirect_to_twitch_auth)
        .service(twitch_callback)
        .service(revoke);
}
