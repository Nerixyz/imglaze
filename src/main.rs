mod actors;
mod constants;
mod errors;
mod extractors;
mod models;
mod repositories;
mod services;

use crate::{
    actors::{
        db::DbActor,
        irc::{messages::JoinAllMessage, IrcActor},
        overlay::OverlayActor,
        token::TokenRefresher,
    },
    constants::{DATABASE_URL, SERVER_URL, TWITCH_CLIENT_ID, TWITCH_CLIENT_SECRET},
    models::overlay,
    repositories::init_repositories,
};
use actix::Actor;
use actix_cors::Cors;
use actix_web::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    middleware::Logger,
    web, App, HttpResponse, HttpServer,
};
use anyhow::Error as AnyError;
use log::LevelFilter;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::str::FromStr;
use tokio::sync::RwLock;
use twitch_api2::{
    helix::Scope,
    twitch_oauth2::{client::reqwest_http_client, AppAccessToken, ClientId, ClientSecret},
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::builder().format_timestamp(None).init();

    log::info!("Connecting to database");

    let mut pool_options =
        PgConnectOptions::from_str(DATABASE_URL).expect("couldn't read database url");
    pool_options.log_statements(LevelFilter::Debug);
    let pool = PgPool::connect_with(pool_options)
        .await
        .expect("Could not connect to database");

    let overlay_actor = OverlayActor::new(pool.clone()).start();

    log::info!("Creating DB and IRC actors");

    let db_actor = DbActor::new(pool.clone()).start();
    let irc_actor = IrcActor::run(overlay_actor.clone().recipient(), db_actor);

    log::info!("Joining all channels");

    irc_actor
        .send(JoinAllMessage(overlay::all_channels(&pool).await.unwrap()))
        .await
        .unwrap();

    log::info!("Updating app access token");

    let _token_actor = TokenRefresher::new(pool.clone()).start();
    let app_access_token = get_app_access_token()
        .await
        .expect("Could not get app access token");
    let app_access_token = web::Data::new(RwLock::new(app_access_token));

    log::info!("Creating server");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(irc_actor.clone()))
            .app_data(web::Data::new(overlay_actor.clone()))
            .app_data(app_access_token.clone())
            .wrap(create_cors())
            .wrap(Logger::default())
            .service(
                web::scope("/api/v1")
                    .configure(init_repositories)
                    .default_service(web::route().to(HttpResponse::NotFound)),
            )
            .service(actix_files::Files::new("/overlay", "web/packages/overlay/dist").index_file("index.html"))
            .service(actix_files::Files::new("/", "web/packages/dashboard/dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8083")?
    .run()
    .await
}

async fn get_app_access_token() -> Result<AppAccessToken, AnyError> {
    Ok(AppAccessToken::get_app_access_token(
        reqwest_http_client,
        ClientId::new(TWITCH_CLIENT_ID.to_string()),
        ClientSecret::new(TWITCH_CLIENT_SECRET.to_string()),
        vec![Scope::ModerationRead],
    )
    .await?)
}

fn create_cors() -> Cors {
    if cfg!(debug_assertions) {
        Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
    } else {
        Cors::default().allowed_origin(SERVER_URL)
    }
}
