use anyhow::Result;
use brain_rot_daemon::{config::Config, database::Database};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    info!("Setting up Brain Rot Tracker database...");

    let config = Config::load().await?;
    let _database = Database::new(&config.database_path).await?;

    info!("Database setup completed successfully!");
    info!("Database location: {}", config.database_path);

    Ok(())
}