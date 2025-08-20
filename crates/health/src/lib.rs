use std::sync::Arc;

use app_core::config::Config;
pub mod inbound;

pub struct Dependency {
    pub db: Arc<Config>,
}
