use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use std::sync::Arc;

use crate::{models::StatsResponse, repositories::url::UrlRepository, AppState};

async fn redirect_url(Path(short_code): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let url_repository = UrlRepository::new();

    match url_repository.get_url(&short_code).await {
        Ok(Some(original_url)) => {
            let _ = url_repository.increment_clicks(&short_code).await;
            Ok(Redirect::to(&original_url))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_stats(Path(short_code): Path<String>) -> Result<impl IntoResponse, StatusCode> {
    let url_repository = UrlRepository::new();

    match url_repository.get_stats(&short_code).await {
        Ok(Some(record)) => {
            let response = StatsResponse {
                short_code: record.short_code,
                original_url: record.original_url,
                clicks: record.clicks,
                created_at: record.created_at,
            };
            Ok(Json(response))
        }
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub fn redirect_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/:short_code", get(redirect_url))
        .route("/:short_code/stats", get(get_stats))
}
