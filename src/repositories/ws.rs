use crate::{
    actors::{overlay::OverlayActor, ws::WsSessionActor},
    errors,
    models::overlay,
};
use actix::Addr;
use actix_web::{web, HttpRequest, HttpResponse, Result};
use actix_web_actors::ws;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
struct WsQuery {
    id: i32,
    secret: String,
}

async fn ws_route(
    query: web::Query<WsQuery>,
    req: HttpRequest,
    stream: web::Payload,
    overlay_actor: web::Data<Addr<OverlayActor>>,
    pool: web::Data<PgPool>,
) -> Result<HttpResponse> {
    let overlay = overlay::by_id(query.id, &pool).await?;
    if overlay.secret != query.secret {
        return Err(errors::ErrorUnauthorized("No, I don't think so"));
    }

    ws::start(
        WsSessionActor::new(overlay.id, overlay_actor.get_ref().clone()),
        &req,
        stream,
    )
}

pub fn init_ws_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("").to(ws_route));
}
