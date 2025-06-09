use once_cell::sync::OnceCell;

use sqlx::postgres::PgPool;

pub static DB_CONNECTION: OnceCell<PgPool> = OnceCell::new();

pub async fn initialize_db_connection(database_url: &str) {
    let pool = PgPool::connect(database_url).await.unwrap();
    DB_CONNECTION.set(pool).expect("DB already initialized");
}

pub fn get_db_connection() -> &'static PgPool {
    DB_CONNECTION.get().expect("DB not initialized")
}
