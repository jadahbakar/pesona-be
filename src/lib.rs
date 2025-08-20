use tokio::{signal, sync::broadcast};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::RequestDecompressionLayer,
    timeout::TimeoutLayer,
};

use crate::{
    app::{router, state::AppState},
    utils::config::Config,
};
use sqlx::postgres::PgPoolOptions;
use std::{error::Error, time::Duration};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
pub mod app;

pub async fn run() -> Result<(), Box<dyn Error>> {
    // Create a broadcast channel to signal shutdown to all application components.
    // Spawn a task to listen for shutdown signals (Ctrl+C and SIGTERM).
    let (shutdown_tx, _) = broadcast::channel(1);
    spawn_shutdown_listener(shutdown_tx.clone());

    let config = Config::new("./config/config.yaml")?;
    // Initialize database pooling
    let db_pool = PgPoolOptions::new()
        .max_connections(config.get::<u32>("database.max_connection")?)
        .connect(&config.get::<String>("database.url")?)
        .await?;

    // Assemble the final AppState from the shared resources and module states.
    let app_state = AppState {
        config: config.clone(),
        db: db_pool,
    };
    let timeout_secs =
        Duration::from_secs(app_state.config.get::<u64>("application.timeout_secs")?);

    let app = router::create_router_app(app_state.clone()).layer(
        ServiceBuilder::new()
            // .layer(middleware::from_fn(request_response_logger))
            .layer(CookieManagerLayer::new())
            .layer(CorsLayer::new().allow_origin(Any)) // Enables CORS for all origins
            .layer(RequestDecompressionLayer::new()) // Enables request compression
            .layer(CompressionLayer::new()) // Enables response compression
            .layer(TimeoutLayer::new(timeout_secs)), // Adds a request timeout
    );
    let server_address = app_state.config.get::<String>("application.address")?;
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    let log_level = app_state.config.get::<String>("log_level")?;
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .json()
                .with_target(true) // Include the target (module path)
                .with_file(true) // Include file names
                .with_line_number(true) // Include line numbers
                .with_span_events(fmt::format::FmtSpan::CLOSE),
        )
        .init();

    tracing::info!("🚀 listening on {}", listener.local_addr()?);

    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            shutdown_tx.subscribe().recv().await.ok();
            tracing::info!("🛑 Server is shutting down gracefully...");
        })
        .await?;

    Ok(())
}

/// Spawns a background task to listen for system shutdown signals.
fn spawn_shutdown_listener(shutdown_tx: broadcast::Sender<()>) {
    tokio::spawn(async move {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("Failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("Failed to install SIGTERM handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => { tracing::info!("🔻 Received SIGINT (Ctrl+C)")},
            _ = terminate => { tracing::info!("🔻 Received SIGTERM")},
        }

        // Send the shutdown signal to all parts of the application.
        if shutdown_tx.send(()).is_err() {
            tracing::error!("Failed to send shutdown signal");
        }
    });
}
