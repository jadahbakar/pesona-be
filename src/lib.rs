use crate::utils::config::Config;
use std::error::Error;

pub mod app;
pub mod health;
pub mod utils;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let config = Config::new("./config/config.yaml")?;
    // Initialize database pooling
    let db_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.get::<u32>("database.max_connection")?)
        .connect(&config.get::<String>("database.url")?)
        .await?;

    Ok(())
}
