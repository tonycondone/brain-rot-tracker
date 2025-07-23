use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    Json as JsonBody,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    database::Database,
    types::{Badge, DailyStats, ExportData, FocusMode, ManualActivity, Rule, UsageEvent},
};

#[derive(Deserialize)]
pub struct DailyStatsQuery {
    pub date: Option<String>,
    pub tz: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
        }
    }
}

pub async fn get_daily_stats(
    Query(params): Query<DailyStatsQuery>,
    State(database): State<Database>,
) -> Result<Json<ApiResponse<DailyStats>>, StatusCode> {
    let date = params.date.unwrap_or_else(|| {
        Utc::now().format("%Y-%m-%d").to_string()
    });

    match database.get_daily_stats(&date, params.tz.as_deref()).await {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(e) => {
            tracing::error!("Failed to get daily stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn add_manual_activity(
    State(database): State<Database>,
    JsonBody(activity): JsonBody<ManualActivity>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let event = UsageEvent {
        id: Uuid::new_v4(),
        timestamp: Utc::now(),
        app_or_site: activity.activity.clone(),
        window_title: activity.notes.clone(),
        duration_sec: activity.duration_minutes * 60,
        rot_points: match activity.category.as_str() {
            "healthy" => -activity.duration_minutes as f64 * 0.5,
            "junk" => activity.duration_minutes as f64 * 2.0,
            _ => 0.0,
        },
        category: activity.category,
    };

    match database.insert_event(&event).await {
        Ok(_) => Ok(Json(ApiResponse::success("Activity recorded successfully".to_string()))),
        Err(e) => {
            tracing::error!("Failed to record manual activity: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_rules(
    State(database): State<Database>,
) -> Result<Json<ApiResponse<Vec<Rule>>>, StatusCode> {
    match database.get_rules().await {
        Ok(rules) => Ok(Json(ApiResponse::success(rules))),
        Err(e) => {
            tracing::error!("Failed to get rules: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn update_rules(
    State(database): State<Database>,
    JsonBody(rules): JsonBody<Vec<Rule>>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    for rule in rules {
        if let Err(e) = database.upsert_rule(&rule).await {
            tracing::error!("Failed to update rule: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    Ok(Json(ApiResponse::success("Rules updated successfully".to_string())))
}

pub async fn get_badges(
    State(database): State<Database>,
) -> Result<Json<ApiResponse<Vec<Badge>>>, StatusCode> {
    match database.get_badges().await {
        Ok(badges) => Ok(Json(ApiResponse::success(badges))),
        Err(e) => {
            tracing::error!("Failed to get badges: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn toggle_focus_mode(
    JsonBody(focus_mode): JsonBody<FocusMode>,
) -> Result<Json<ApiResponse<String>>, StatusCode> {
    // For now, just return success - focus mode implementation would go here
    let status = if focus_mode.enabled { "enabled" } else { "disabled" };
    Ok(Json(ApiResponse::success(format!("Focus mode {}", status))))
}

#[derive(Deserialize)]
pub struct ExportQuery {
    pub format: Option<String>,
}

pub async fn export_data(
    Query(params): Query<ExportQuery>,
    State(database): State<Database>,
) -> Result<Json<Value>, StatusCode> {
    let format = params.format.unwrap_or_else(|| "json".to_string());

    match (database.get_all_events().await, database.get_rules().await, database.get_badges().await) {
        (Ok(events), Ok(rules), Ok(badges)) => {
            let export_data = ExportData {
                events,
                rules,
                badges,
                exported_at: Utc::now(),
            };

            match format.as_str() {
                "csv" => {
                    // For CSV, we'll just export events for now
                    let mut csv_data = String::from("timestamp,app_or_site,window_title,duration_sec,rot_points,category\n");
                    for event in &export_data.events {
                        csv_data.push_str(&format!(
                            "{},{},{},{},{},{}\n",
                            event.timestamp.to_rfc3339(),
                            event.app_or_site,
                            event.window_title.as_deref().unwrap_or(""),
                            event.duration_sec,
                            event.rot_points,
                            event.category
                        ));
                    }
                    Ok(Json(json!({ "data": csv_data, "format": "csv" })))
                }
                _ => Ok(Json(serde_json::to_value(export_data).unwrap())),
            }
        }
        _ => {
            tracing::error!("Failed to export data");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "timestamp": Utc::now().to_rfc3339(),
        "version": env!("CARGO_PKG_VERSION")
    }))
}