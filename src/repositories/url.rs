use anyhow::Result;
use redis::AsyncCommands;
use sqlx::{PgPool, Row};

use crate::{
    clients::{postgres::get_db_connection, redis::get_redis_client},
    models::UrlRecord,
};

pub struct UrlRepository {
    pg_pool: PgPool,
    redis_client: redis::Client,
}

impl UrlRepository {
    pub fn new() -> Self {
        UrlRepository {
            pg_pool: get_db_connection().clone(),
            redis_client: get_redis_client().clone(),
        }
    }

    pub async fn save_url(&self, short_code: &str, original_url: &str) -> Result<()> {
        sqlx::query("INSERT INTO urls (short_code, original_url) VALUES ($1, $2)")
            .bind(short_code)
            .bind(original_url)
            .execute(&self.pg_pool)
            .await?;

        let mut redis_conn = self.redis_client.get_async_connection().await?;
        redis_conn
            .set_ex::<_, _, ()>(short_code, original_url, 3600)
            .await?;

        Ok(())
    }

    pub async fn get_url(&self, short_code: &str) -> Result<Option<String>> {
        // Primeiro tenta no Redis
        let mut redis_conn = self.redis_client.get_async_connection().await?;
        let redis_result: Option<String> = redis_conn.get(short_code).await.ok();

        if let Some(url) = redis_result {
            return Ok(Some(url));
        }

        // Se não encontrou no Redis, busca no Postgres
        let row = sqlx::query("SELECT original_url FROM urls WHERE short_code = $1")
            .bind(short_code)
            .fetch_optional(&self.pg_pool)
            .await?;

        if let Some(row) = row {
            let original_url: String = row.get("original_url");

            // Recolocar no Redis
            redis_conn
                .set_ex::<_, _, ()>(short_code, &original_url, 3600)
                .await?;

            Ok(Some(original_url))
        } else {
            Ok(None)
        }
    }

    pub async fn increment_clicks(&self, short_code: &str) -> Result<()> {
        // Incrementar clicks de forma assíncrona
        tokio::spawn({
            let pool = self.pg_pool.clone();
            let short_code = short_code.to_string();
            async move {
                let _ = sqlx::query("UPDATE urls SET clicks = clicks + 1 WHERE short_code = $1")
                    .bind(&short_code)
                    .execute(&pool)
                    .await;
            }
        });

        Ok(())
    }

    pub async fn get_stats(&self, short_code: &str) -> Result<Option<UrlRecord>> {
        let row = sqlx::query_as::<_, UrlRecord>(
            "SELECT id, short_code, original_url, created_at, clicks FROM urls WHERE short_code = $1"
        )
        .bind(short_code)
        .fetch_optional(&self.pg_pool)
        .await?;

        Ok(row)
    }

    pub async fn short_code_exists(&self, short_code: &str) -> Result<bool> {
        let row = sqlx::query("SELECT 1 FROM urls WHERE short_code = $1 LIMIT 1")
            .bind(short_code)
            .fetch_optional(&self.pg_pool)
            .await?;

        Ok(row.is_some())
    }
}
