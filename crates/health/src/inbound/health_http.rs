use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;
use std::sync::Arc;

use crate::inbound::state::HealthState;

pub async fn health(State(state): State<Arc<HealthState>>) -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "version": env!("CARGO_PKG_VERSION"),
            "db_connected": !state.db.is_closed(),
        })),
    )
}
