use crate::{
    actors::{
        irc::{messages::JoinMessage, IrcActor},
        overlay::{messages::DeleteOverlay, OverlayActor},
    },
    models::overlay,
    services::{ivr::check_mod, jwt::JwtClaims},
};
use actix::Addr;
use actix_web::{delete, get, patch, put, web, HttpResponse, Result};
use errors::{json_error::JsonError, sql::SqlReason};
use rand::{distributions::Alphanumeric, Rng};
use serde::Deserialize;
use sqlx::PgPool;
use twitch_api2::twitch_oauth2::UserToken;

#[get("")]
async fn get_all(claims: JwtClaims, pool: web::Data<PgPool>) -> Result<HttpResponse> {
    let overlays = overlay::all_from_user(claims.user_id(), &pool).await?;

    Ok(HttpResponse::Ok().json(overlays))
}

#[derive(Deserialize)]
struct OverlayCreateBody {
    for_user: String,
}

#[put("")]
async fn create(
    claims: JwtClaims,
    body: web::Json<OverlayCreateBody>,
    pool: web::Data<PgPool>,
    irc: web::Data<Addr<IrcActor>>,
) -> Result<HttpResponse> {
    let user: UserToken = claims.get_user(&pool).await?.into();
    let for_user = body.into_inner().for_user.to_lowercase();
    if user.login != for_user && !check_mod(&for_user, &user.login).await {
        return Err(errors::ErrorForbidden("You aren't a moderator."));
    }

    let secret = generate_secret();
    let overlay = match overlay::create(&user.user_id, &for_user, &secret, &pool).await {
        Ok(overlay) => overlay,
        Err(JsonError {
            error: SqlReason::Conflict(ref constraint),
            ..
        }) if constraint == "overlays_created_by_for_user_uindex" => {
            return Err(match overlay::creator_for(&for_user, &pool).await {
                Ok(creator) => errors::ErrorConflict(format!(
                    "This channel already exists. It's managed by {}",
                    creator
                )),
                Err(_) => errors::ErrorConflict(
                    "This channel already exists but there's no further info.",
                ),
            });
        }
        _ => return Err(errors::ErrorBadRequest("This channel already exists")),
    };

    irc.do_send(JoinMessage(for_user));

    Ok(HttpResponse::Ok().json(overlay))
}

#[patch("/{id}")]
async fn regenerate_secret(
    claims: JwtClaims,
    id: web::Path<String>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let id = id
        .parse::<i32>()
        .map_err(|_| errors::ErrorBadRequest("bad id"))?;
    let mut overlay = overlay::by_id(id, &pool).await?;
    if overlay.created_by != claims.user_id() {
        return Err(errors::ErrorForbidden(
            "arnoldHalt you are not the creator!",
        ));
    }
    overlay.secret = generate_secret();
    overlay.patch_secret(&pool).await?;

    Ok(HttpResponse::Ok().json(overlay))
}

#[delete("/{id}")]
async fn delete(
    claims: JwtClaims,
    id: web::Path<String>,
    pool: web::Data<PgPool>,
    overlay_actor: web::Data<Addr<OverlayActor>>,
) -> Result<HttpResponse> {
    let id = id
        .parse::<i32>()
        .map_err(|_| errors::ErrorBadRequest("bad id"))?;
    let overlay = overlay::by_id(id, &pool).await?;
    if overlay.created_by != claims.user_id() {
        return Err(errors::ErrorForbidden(
            "arnoldHalt you are not the creator!",
        ));
    }
    overlay.delete(&pool).await?;
    overlay_actor.do_send(DeleteOverlay(overlay.id));

    Ok(HttpResponse::NoContent().finish())
}

fn generate_secret() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

pub fn init_overlay_routes(config: &mut web::ServiceConfig) {
    config
        .service(get_all)
        .service(regenerate_secret)
        .service(create)
        .service(delete);
}
