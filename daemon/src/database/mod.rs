use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{sqlite::SqlitePool, Row};
use std::path::Path;
use uuid::Uuid;

use crate::types::{Badge, Rule, UsageEvent, DailyStats, CategoryBreakdown, AppBreakdown};

#[derive(Debug, Clone)]
pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self> {
        // Ensure parent directory exists
        if let Some(parent) = Path::new(database_path).parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        let database_url = format!("sqlite:{}", database_path);
        let pool = SqlitePool::connect(&database_url).await?;

        let db = Self { pool };
        db.migrate().await?;
        db.seed_default_data().await?;

        Ok(db)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                app_or_site TEXT NOT NULL,
                window_title TEXT,
                duration_sec INTEGER NOT NULL,
                rot_points REAL NOT NULL,
                category TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS rules (
                id TEXT PRIMARY KEY,
                pattern TEXT NOT NULL UNIQUE,
                multiplier REAL NOT NULL,
                category TEXT NOT NULL,
                unless_title_contains TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS badges (
                id TEXT PRIMARY KEY,
                badge_id TEXT NOT NULL UNIQUE,
                name TEXT NOT NULL,
                description TEXT NOT NULL,
                unlock_condition TEXT NOT NULL,
                unlocked_at TEXT,
                created_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_events_timestamp ON events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_events_app_or_site ON events(app_or_site);
            CREATE INDEX IF NOT EXISTS idx_events_category ON events(category);
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn seed_default_data(&self) -> Result<()> {
        // Check if we already have rules
        let rule_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM rules")
            .fetch_one(&self.pool)
            .await?;

        if rule_count == 0 {
            self.seed_default_rules().await?;
        }

        // Check if we already have badges
        let badge_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM badges")
            .fetch_one(&self.pool)
            .await?;

        if badge_count == 0 {
            self.seed_default_badges().await?;
        }

        Ok(())
    }

    async fn seed_default_rules(&self) -> Result<()> {
        let default_rules = vec![
            ("TikTok*", 12.0, "junk"),
            ("*YouTube*", 4.0, "junk"),
            ("*Twitter*", 6.0, "junk"),
            ("*Instagram*", 5.0, "junk"),
            ("*Facebook*", 5.0, "junk"),
            ("*Reddit*", 3.0, "junk"),
            ("*Twitch*", 4.0, "junk"),
            ("*Netflix*", 2.0, "junk"),
            ("*Kindle*", -3.0, "healthy"),
            ("*Duolingo*", -5.0, "healthy"),
            ("*Anki*", -4.0, "healthy"),
            ("*Code*", -2.0, "healthy"),
            ("*Terminal*", -1.0, "healthy"),
        ];

        for (pattern, multiplier, category) in default_rules {
            let id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();

            sqlx::query(
                r#"
                INSERT OR IGNORE INTO rules (id, pattern, multiplier, category, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&id)
            .bind(pattern)
            .bind(multiplier)
            .bind(category)
            .bind(&now)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    async fn seed_default_badges(&self) -> Result<()> {
        let default_badges = vec![
            ("first_day", "First Day", "Complete your first day of tracking", "events_count >= 1"),
            ("week_warrior", "Week Warrior", "Track for 7 consecutive days", "streak_days >= 7"),
            ("month_master", "Month Master", "Track for 30 consecutive days", "streak_days >= 30"),
            ("low_rot_day", "Low Rot Day", "Keep daily rot under 100 points", "daily_rot < 100"),
            ("touch_grass", "Touch Grass", "Use the Touch Grass feature 10 times", "touch_grass_count >= 10"),
            ("healthy_habits", "Healthy Habits", "Accumulate 100 negative rot points", "negative_rot >= 100"),
        ];

        for (badge_id, name, description, condition) in default_badges {
            let id = Uuid::new_v4().to_string();
            let now = Utc::now().to_rfc3339();

            sqlx::query(
                r#"
                INSERT OR IGNORE INTO badges (id, badge_id, name, description, unlock_condition, created_at)
                VALUES (?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(&id)
            .bind(badge_id)
            .bind(name)
            .bind(description)
            .bind(condition)
            .bind(&now)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn insert_event(&self, event: &UsageEvent) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO events (id, timestamp, app_or_site, window_title, duration_sec, rot_points, category)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(event.id.to_string())
        .bind(event.timestamp.to_rfc3339())
        .bind(&event.app_or_site)
        .bind(&event.window_title)
        .bind(event.duration_sec)
        .bind(event.rot_points)
        .bind(&event.category)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_daily_stats(&self, date: &str, timezone: Option<&str>) -> Result<DailyStats> {
        let start_date = format!("{}T00:00:00Z", date);
        let end_date = format!("{}T23:59:59Z", date);

        // Get total stats
        let total_row = sqlx::query(
            r#"
            SELECT 
                COALESCE(SUM(rot_points), 0) as total_rot_points,
                COALESCE(SUM(duration_sec), 0) / 60 as total_duration_minutes
            FROM events 
            WHERE timestamp >= ? AND timestamp <= ?
            "#,
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_one(&self.pool)
        .await?;

        let total_rot_points: f64 = total_row.get("total_rot_points");
        let total_duration_minutes: i64 = total_row.get("total_duration_minutes");

        // Get category breakdown
        let category_rows = sqlx::query(
            r#"
            SELECT 
                category,
                SUM(rot_points) as rot_points,
                SUM(duration_sec) / 60 as duration_minutes
            FROM events 
            WHERE timestamp >= ? AND timestamp <= ?
            GROUP BY category
            ORDER BY rot_points DESC
            "#,
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(&self.pool)
        .await?;

        let breakdown: Vec<CategoryBreakdown> = category_rows
            .into_iter()
            .map(|row| {
                let rot_points: f64 = row.get("rot_points");
                let duration_minutes: i64 = row.get("duration_minutes");
                let percentage = if total_rot_points > 0.0 {
                    (rot_points / total_rot_points) * 100.0
                } else {
                    0.0
                };

                CategoryBreakdown {
                    category: row.get("category"),
                    rot_points,
                    duration_minutes,
                    percentage,
                }
            })
            .collect();

        // Get top offenders
        let app_rows = sqlx::query(
            r#"
            SELECT 
                app_or_site,
                category,
                SUM(rot_points) as rot_points,
                SUM(duration_sec) / 60 as duration_minutes
            FROM events 
            WHERE timestamp >= ? AND timestamp <= ?
            GROUP BY app_or_site, category
            ORDER BY rot_points DESC
            LIMIT 10
            "#,
        )
        .bind(&start_date)
        .bind(&end_date)
        .fetch_all(&self.pool)
        .await?;

        let top_offenders: Vec<AppBreakdown> = app_rows
            .into_iter()
            .map(|row| AppBreakdown {
                app_or_site: row.get("app_or_site"),
                category: row.get("category"),
                rot_points: row.get("rot_points"),
                duration_minutes: row.get("duration_minutes"),
            })
            .collect();

        Ok(DailyStats {
            date: date.to_string(),
            total_rot_points,
            total_duration_minutes,
            breakdown,
            top_offenders,
        })
    }

    pub async fn get_rules(&self) -> Result<Vec<Rule>> {
        let rows = sqlx::query_as::<_, Rule>(
            r#"
            SELECT 
                id, pattern, multiplier, category, unless_title_contains,
                created_at, updated_at
            FROM rules 
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn upsert_rule(&self, rule: &Rule) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR REPLACE INTO rules 
            (id, pattern, multiplier, category, unless_title_contains, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(rule.id.to_string())
        .bind(&rule.pattern)
        .bind(rule.multiplier)
        .bind(&rule.category)
        .bind(rule.unless_title_contains.as_ref().map(|v| serde_json::to_string(v).unwrap()))
        .bind(rule.created_at.to_rfc3339())
        .bind(rule.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_badges(&self) -> Result<Vec<Badge>> {
        let rows = sqlx::query_as::<_, Badge>(
            "SELECT id, badge_id, name, description, unlock_condition, unlocked_at, created_at FROM badges ORDER BY created_at"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    pub async fn unlock_badge(&self, badge_id: &str) -> Result<()> {
        let now = Utc::now().to_rfc3339();
        sqlx::query("UPDATE badges SET unlocked_at = ? WHERE badge_id = ? AND unlocked_at IS NULL")
            .bind(&now)
            .bind(badge_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    pub async fn get_all_events(&self) -> Result<Vec<UsageEvent>> {
        let rows = sqlx::query_as::<_, UsageEvent>(
            "SELECT id, timestamp, app_or_site, window_title, duration_sec, rot_points, category FROM events ORDER BY timestamp DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }
}