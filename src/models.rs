use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenResponse {
    pub short_code: String,
    pub short_url: String,
    pub original_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatsResponse {
    pub short_code: String,
    pub original_url: String,
    pub clicks: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
pub struct UrlRecord {
    // pub id: i32,
    pub short_code: String,
    pub original_url: String,
    pub created_at: DateTime<Utc>,
    pub clicks: i64,
} 