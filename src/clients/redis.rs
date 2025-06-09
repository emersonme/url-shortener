use once_cell::sync::OnceCell;
use redis::Client;

pub static REDIS_POOL: OnceCell<Client> = OnceCell::new();

pub async fn initialize_redis_pool(redis_url: &str) {
    let client = redis::Client::open(redis_url).unwrap();
    let _ = REDIS_POOL.set(client);
}

pub fn get_redis_client() -> &'static Client {
    REDIS_POOL.get().expect("Redis not initialized")
}
