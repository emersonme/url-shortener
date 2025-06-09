use env_logger::{Builder, Env};
use std::env;

#[derive(Clone)]
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub server_address: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let rust_env = env::var("RUST_ENV")
            .unwrap_or("dev".to_string())
            .to_lowercase();

        let env_filename = format!(".env.{}", rust_env);

        dotenv::from_filename(env_filename).ok();

        let env = Env::default().default_filter_or("debug");
        Builder::from_env(env).init();

        Ok(Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            redis_url: env::var("REDIS_URL").expect("REDIS_URL must be set"),
            server_address: env::var("SERVER_ADDRESS").expect("SERVER_ADDRESS must be set"),
        })
    }
}
