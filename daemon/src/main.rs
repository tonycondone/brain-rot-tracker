use anyhow::Result;
use brain_rot_daemon::{
    api::server::ApiServer,
    config::Config,
    database::Database,
    tracker::UsageTracker,
};
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("brain_rot_daemon=debug,info")
        .init();

    info!("Starting Brain Rot Tracker Daemon v{}", env!("CARGO_PKG_VERSION"));

    // Load configuration
    let config = Config::load().await?;
    info!("Configuration loaded successfully");

    // Initialize database
    let database = Database::new(&config.database_path).await?;
    info!("Database initialized at: {}", config.database_path);

    // Start usage tracker
    let tracker = UsageTracker::new(database.clone(), config.clone());
    let tracker_handle = tokio::spawn(async move {
        if let Err(e) = tracker.start().await {
            error!("Usage tracker error: {}", e);
        }
    });

    // Start API server
    let api_server = ApiServer::new(database, config.clone());
    let server_handle = tokio::spawn(async move {
        if let Err(e) = api_server.start().await {
            error!("API server error: {}", e);
        }
    });

    info!("Brain Rot Tracker Daemon started successfully");
    info!("API server running on: http://localhost:{}", config.api_port);
    info!("Dashboard available at: http://localhost:{}/dashboard", config.api_port);

    // Wait for both tasks
    tokio::select! {
        _ = tracker_handle => {
            error!("Usage tracker stopped unexpectedly");
        }
        _ = server_handle => {
            error!("API server stopped unexpectedly");
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    info!("Brain Rot Tracker Daemon shutting down");
    Ok(())
}