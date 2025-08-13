use crate::utils::config::Config;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db: PgPool,
}
