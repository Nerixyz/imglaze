use actix_web::web;

mod auth;
mod overlay;
mod ws;

pub fn init_repositories(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/auth").configure(auth::init_auth_routes))
        .service(web::scope("/overlays").configure(overlay::init_overlay_routes))
        .service(web::scope("/overlay-ws").configure(ws::init_ws_routes));
}
