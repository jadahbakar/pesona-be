use axum::{Router, routing::get};

use crate::{
    app::state::AppState,
    health::inbound::health_http::{self, health},
};

pub fn create_router(state: AppState) -> Router {
    let public_route = Router::new().route("/health", get(health));
    public_route
}
