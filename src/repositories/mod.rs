use actix_metrics::Metrics;
use actix_web::{get, web, Responder};
use metrics_exporter_prometheus::PrometheusHandle;
use std::future::{ready, Ready};

mod auth;
mod overlay;
mod ws;

#[get("/metrics")]
fn metrics_render(handle: web::Data<PrometheusHandle>) -> Ready<impl Responder> {
    ready(handle.render())
}

pub fn init_repositories(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/auth")
                .wrap(Metrics::new("auth"))
                .configure(auth::init_auth_routes),
        )
        .service(
            web::scope("/overlays")
                .wrap(Metrics::new("overlay"))
                .configure(overlay::init_overlay_routes),
        )
        .service(web::scope("/overlay-ws").configure(ws::init_ws_routes))
        .service(metrics_render);
}
