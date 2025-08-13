use axum::{Json, Router, http::StatusCode, routing};

use crate::app::state::AppState;

pub fn create_router_app(app_state: AppState) -> Router {
    Router::new()
        .merge(crate::health::inbound::router::create_router(app_state))
        .route(
            "/",
            routing::get(|| async {
                Json(serde_json::json!({"message": "Hello from Rust Starter"}))
            }),
        )
        .fallback(|| async {
            (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"message": "Endpoint not found"})),
            )
        })
        .method_not_allowed_fallback(|| async {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                Json(serde_json::json!({"message": "Method not allowed"})),
            )
        })
}
