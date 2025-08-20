use std::sync::Arc;

use axum::{Router, routing::get};

use crate::{app::state::AppState, health::inbound::health_http::health};

pub fn create_router(state: AppState) -> Router {
    // let public_route = Router::new().route("/health", get(health(state)));
    // public_route
    Router::new()
        .route("/health", get(health))
        .with_state(Arc::new(state)) // Gunakan with_state
}
