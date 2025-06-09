mod clients;
mod config;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

use log::info;
use std::sync::Arc;

use axum::Router;
use tower_http::cors::CorsLayer;

use crate::{
    clients::{postgres::initialize_db_connection, redis::initialize_redis_pool},
    config::Config,
    routes::{redirect::redirect_routes, shorten::shorten_routes},
};

pub struct AppState {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    tokio::join!(
        initialize_db_connection(&config.database_url),
        initialize_redis_pool(&config.redis_url)
    );

    let app_state = Arc::new(AppState {});

    let app = Router::new()
        .merge(shorten_routes())
        .merge(redirect_routes())
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind(&config.server_address)
        .await
        .unwrap();

    info!("🚀 server running on {}", config.server_address);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
