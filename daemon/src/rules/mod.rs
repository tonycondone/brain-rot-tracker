use anyhow::Result;
use glob::Pattern;
use tracing::{debug, warn};

use crate::{database::Database, types::Rule};

pub struct RuleEngine {
    rules: Vec<Rule>,
}

impl RuleEngine {
    pub fn new() -> Self {
        Self { rules: Vec::new() }
    }

    pub async fn load_from_database(&mut self, database: &Database) -> Result<()> {
        self.rules = database.get_rules().await?;
        debug!("Loaded {} rules from database", self.rules.len());
        Ok(())
    }

    pub fn calculate_rot_points(&self, app_name: &str, window_title: Option<&str>) -> (f64, String) {
        for rule in &self.rules {
            if let Ok(pattern) = Pattern::new(&rule.pattern) {
                if pattern.matches(app_name) {
                    // Check unless_title_contains conditions
                    if let (Some(title), Some(ref unless_conditions)) = (window_title, &rule.unless_title_contains) {
                        let title_lower = title.to_lowercase();
                        if unless_conditions.iter().any(|condition| title_lower.contains(&condition.to_lowercase())) {
                            debug!("Rule '{}' matched but excluded by title condition", rule.pattern);
                            continue;
                        }
                    }

                    let rot_points = rule.multiplier * (5.0 / 60.0); // 5 seconds converted to minutes
                    debug!(
                        "Applied rule '{}' to '{}': {} points (category: {})",
                        rule.pattern, app_name, rot_points, rule.category
                    );
                    return (rot_points, rule.category.clone());
                }
            } else {
                warn!("Invalid pattern in rule: {}", rule.pattern);
            }
        }

        // Default: neutral activity
        debug!("No rule matched for '{}', using default", app_name);
        (0.0, "neutral".to_string())
    }

    pub fn get_rules(&self) -> &[Rule] {
        &self.rules
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn update_rule(&mut self, updated_rule: Rule) {
        if let Some(rule) = self.rules.iter_mut().find(|r| r.id == updated_rule.id) {
            *rule = updated_rule;
        }
    }

    pub fn remove_rule(&mut self, rule_id: &uuid::Uuid) {
        self.rules.retain(|r| r.id != *rule_id);
    }
}

impl Default for RuleEngine {
    fn default() -> Self {
        Self::new()
    }
}