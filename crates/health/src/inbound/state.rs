use app_core::config::Config;
use sqlx::PgPool;

pub struct HealthState {
    pub config: Config,
    pub db: PgPool,
}
