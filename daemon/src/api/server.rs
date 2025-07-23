use anyhow::Result;
use axum::{
    extract::DefaultBodyLimit,
    http::{header, Method},
    routing::{get, post, put},
    Router,
};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::info;

use crate::{config::Config, database::Database};

use super::handlers;

pub struct ApiServer {
    database: Database,
    config: Config,
}

impl ApiServer {
    pub fn new(database: Database, config: Config) -> Self {
        Self { database, config }
    }

    pub async fn start(self) -> Result<()> {
        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
            .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
            .allow_origin(Any);

        let api_routes = Router::new()
            .route("/rot/daily", get(handlers::get_daily_stats))
            .route("/rot/manual", post(handlers::add_manual_activity))
            .route("/rules", get(handlers::get_rules))
            .route("/rules", put(handlers::update_rules))
            .route("/badges", get(handlers::get_badges))
            .route("/focus-mode", post(handlers::toggle_focus_mode))
            .route("/export", get(handlers::export_data))
            .route("/health", get(handlers::health_check))
            .with_state(self.database.clone());

        let dashboard_service = ServeDir::new(&self.config.dashboard_path)
            .append_index_html_on_directories(true);

        let app = Router::new()
            .nest("/api", api_routes)
            .fallback_service(dashboard_service)
            .layer(
                ServiceBuilder::new()
                    .layer(cors)
                    .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB
            );

        let addr = SocketAddr::from(([127, 0, 0, 1], self.config.api_port));
        info!("API server starting on {}", addr);

        let listener = tokio::net::TcpListener::bind(addr).await?;
        axum::serve(listener, app).await?;

        Ok(())
    }
}