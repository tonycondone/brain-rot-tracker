use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UsageEvent {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub app_or_site: String,
    pub window_title: Option<String>,
    pub duration_sec: i64,
    pub rot_points: f64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Rule {
    pub id: Uuid,
    pub pattern: String,
    pub multiplier: f64,
    pub category: String,
    pub unless_title_contains: Option<Vec<String>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Badge {
    pub id: Uuid,
    pub badge_id: String,
    pub name: String,
    pub description: String,
    pub unlock_condition: String,
    pub unlocked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyStats {
    pub date: String,
    pub total_rot_points: f64,
    pub total_duration_minutes: i64,
    pub breakdown: Vec<CategoryBreakdown>,
    pub top_offenders: Vec<AppBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryBreakdown {
    pub category: String,
    pub rot_points: f64,
    pub duration_minutes: i64,
    pub percentage: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBreakdown {
    pub app_or_site: String,
    pub rot_points: f64,
    pub duration_minutes: i64,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualActivity {
    pub activity: String,
    pub duration_minutes: i64,
    pub category: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusMode {
    pub enabled: bool,
    pub daily_budget_minutes: Option<i64>,
    pub blocked_apps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportData {
    pub events: Vec<UsageEvent>,
    pub rules: Vec<Rule>,
    pub badges: Vec<Badge>,
    pub exported_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWindow {
    pub app_name: String,
    pub window_title: String,
    pub process_id: u32,
}