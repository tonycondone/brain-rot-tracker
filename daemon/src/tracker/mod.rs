use anyhow::Result;
use chrono::{DateTime, Utc};
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use crate::{
    config::Config,
    database::Database,
    rules::RuleEngine,
    types::{ActiveWindow, UsageEvent},
};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "linux")]
mod linux;

pub struct UsageTracker {
    database: Database,
    config: Config,
    rule_engine: RuleEngine,
    last_window: Option<ActiveWindow>,
    last_check: DateTime<Utc>,
}

impl UsageTracker {
    pub fn new(database: Database, config: Config) -> Self {
        let rule_engine = RuleEngine::new();
        
        Self {
            database,
            config,
            rule_engine,
            last_window: None,
            last_check: Utc::now(),
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting usage tracker");
        
        // Load rules from database
        self.rule_engine.load_from_database(&self.database).await?;
        
        let mut interval = time::interval(Duration::from_secs(self.config.tracking_interval_seconds));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.track_current_usage().await {
                error!("Error tracking usage: {}", e);
            }
        }
    }

    async fn track_current_usage(&mut self) -> Result<()> {
        let current_window = self.get_active_window().await?;
        let now = Utc::now();
        
        // If we have a previous window and it's different from current, record the usage
        if let Some(ref last_window) = self.last_window {
            if last_window.app_name != current_window.app_name 
                || last_window.window_title != current_window.window_title {
                
                let duration = (now - self.last_check).num_seconds();
                
                if duration > 0 {
                    self.record_usage_event(last_window, duration).await?;
                }
            }
        }
        
        self.last_window = Some(current_window);
        self.last_check = now;
        
        Ok(())
    }

    async fn record_usage_event(&self, window: &ActiveWindow, duration_sec: i64) -> Result<()> {
        let (rot_points, category) = self.rule_engine.calculate_rot_points(
            &window.app_name,
            Some(&window.window_title),
        );

        let event = UsageEvent {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            app_or_site: window.app_name.clone(),
            window_title: Some(window.window_title.clone()),
            duration_sec,
            rot_points,
            category,
        };

        debug!(
            "Recording usage: {} for {}s ({}pts, {})",
            event.app_or_site, event.duration_sec, event.rot_points, event.category
        );

        self.database.insert_event(&event).await?;
        
        Ok(())
    }

    async fn get_active_window(&self) -> Result<ActiveWindow> {
        #[cfg(target_os = "windows")]
        return windows::get_active_window().await;
        
        #[cfg(target_os = "macos")]
        return macos::get_active_window().await;
        
        #[cfg(target_os = "linux")]
        return linux::get_active_window().await;
        
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            warn!("Platform not supported for window tracking");
            Ok(ActiveWindow {
                app_name: "Unknown".to_string(),
                window_title: "Unknown".to_string(),
                process_id: 0,
            })
        }
    }
}