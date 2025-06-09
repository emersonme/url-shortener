use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use std::sync::Arc;

use crate::{
    models::ShortenRequest, services::shorten::build_shorten_url, utils::is_valid_url, AppState,
};

async fn shorten_url(Json(payload): Json<ShortenRequest>) -> Result<impl IntoResponse, StatusCode> {
    if !is_valid_url(&payload.url) {
        return Err(StatusCode::BAD_REQUEST);
    }

    match build_shorten_url(&payload.url).await {
        Ok(shorten_response) => Ok(Json(shorten_response)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn shorten_routes() -> Router<Arc<AppState>> {
    Router::new().route("/shorten", post(shorten_url))
}
