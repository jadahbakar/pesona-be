use std::sync::Arc;

use axum::{Router, routing::get};

use crate::inbound::{health_http::health, state::HealthState};

pub fn create_router(state: HealthState) -> Router {
    // let public_route = Router::new().route("/health", get(health(state)));
    // public_route
    Router::new()
        .route("/health", get(health))
        .with_state(Arc::new(state)) // Gunakan with_state
}
