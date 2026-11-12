use super::policy::{PolicyRule, ValidationResult, Action};
use crate::parser::models::{Event, Severity};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Timelike};

/// Constitutional Guardian - Policy validator
pub struct PolicyValidator {
    rules: Vec<PolicyRule>,
    cache: HashMap<String, ValidationResult>,
    business_critical_systems: Vec<String>,
    restricted_hours: (u32, u32), // (start_hour, end_hour)
}

impl PolicyValidator {
    pub fn new(rules: Vec<PolicyRule>) -> Self {
        Self {
            rules,
            cache: HashMap::new(),
            business_critical_systems: vec![
                "postgres".to_string(),
                "mysql".to_string(),
                "payment_processor".to_string(),
            ],
            restricted_hours: (9, 17), // 9 AM to 5 PM
        }
    }

    /// Validate event against all rules
    pub fn validate(&self, event: &Event) -> ValidationResult {
        // Check cache first
        let cache_key = format!("{:?}_{}", event.event_type, event.process.name);
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }

        // Evaluate rules
        for rule in &self.rules {
            if !rule.enabled {
                continue;
            }

            if rule.condition.matches(event) {
                let mut action = rule.action.clone();
                
                // Apply Constitutional Guardian checks
                if self.is_business_critical(&event.process.name) {
                    action.requires_approval = true;
                }

                if self.is_restricted_time() && action.requires_approval {
                    // During business hours, require approval for critical systems
                    action.requires_approval = true;
                }

                return ValidationResult {
                    matched: true,
                    rule_id: Some(rule.id.clone()),
                    severity: rule.severity,
                    action: Some(action),
                };
            }
        }

        ValidationResult::default()
    }

    /// Check if process is business-critical
    fn is_business_critical(&self, process_name: &str) -> bool {
        self.business_critical_systems.iter()
            .any(|sys| process_name.contains(sys))
    }

    /// Check if current time is during restricted hours
    fn is_restricted_time(&self) -> bool {
        let now = Utc::now();
        let hour = now.hour();
        hour >= self.restricted_hours.0 && hour < self.restricted_hours.1
    }

    /// Add a new rule
    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
        self.cache.clear(); // Invalidate cache
    }

    /// Remove a rule by ID
    pub fn remove_rule(&mut self, rule_id: &str) -> bool {
        let initial_len = self.rules.len();
        self.rules.retain(|r| r.id != rule_id);
        let removed = self.rules.len() < initial_len;
        
        if removed {
            self.cache.clear(); // Invalidate cache
        }
        
        removed
    }

    /// Get all rules
    pub fn get_rules(&self) -> &[PolicyRule] {
        &self.rules
    }

    /// Clear validation cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::*;
    use crate::validator::policy::*;
    use std::path::PathBuf;

    fn create_test_rule() -> PolicyRule {
        PolicyRule {
            id: "rule_001".to_string(),
            name: "Detect suspicious binary".to_string(),
            description: "Alert on suspicious binary execution".to_string(),
            severity: Severity::High,
            enabled: true,
            condition: Condition {
                field: "process.name".to_string(),
                operator: Operator::Contains,
                value: serde_json::json!("suspicious"),
                logical_op: None,
                sub_conditions: vec![],
            },
            action: Action {
                action_type: ActionType::Alert,
                target: None,
                reason: "Suspicious binary detected".to_string(),
                requires_approval: false,
            },
            metadata: RuleMetadata {
                created_at: Utc::now(),
                updated_at: Utc::now(),
                author: "admin".to_string(),
                tags: vec!["malware".to_string()],
            },
        }
    }

    fn create_test_event(process_name: &str) -> Event {
        Event {
            id: uuid::Uuid::new_v4(),
            timestamp: Utc::now(),
            event_type: EventType::ProcessExecution,
            severity: Severity::Medium,
            process: ProcessInfo {
                pid: 1234,
                name: process_name.to_string(),
                exe_path: PathBuf::from(format!("/tmp/{}", process_name)),
                parent_pid: 1000,
                parent_name: "bash".to_string(),
                user: "www-data".to_string(),
                uid: 33,
                command_line: format!("/tmp/{}", process_name),
                start_time: Utc::now(),
            },
            syscall: None,
            network: None,
            file: None,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_validator_matches_rule() {
        let rule = create_test_rule();
        let validator = PolicyValidator::new(vec![rule]);
        
        let event = create_test_event("suspicious_binary");
        let result = validator.validate(&event);
        
        assert!(result.matched);
        assert_eq!(result.rule_id, Some("rule_001".to_string()));
        assert_eq!(result.severity, Severity::High);
    }

    #[test]
    fn test_validator_no_match() {
        let rule = create_test_rule();
        let validator = PolicyValidator::new(vec![rule]);
        
        let event = create_test_event("normal_process");
        let result = validator.validate(&event);
        
        assert!(!result.matched);
        assert_eq!(result.rule_id, None);
    }

    #[test]
    fn test_business_critical_requires_approval() {
        let rule = create_test_rule();
        let validator = PolicyValidator::new(vec![rule]);
        
        let event = create_test_event("suspicious_postgres");
        let result = validator.validate(&event);
        
        assert!(result.matched);
        if let Some(action) = result.action {
            assert!(action.requires_approval);
        }
    }

    #[test]
    fn test_add_remove_rule() {
        let mut validator = PolicyValidator::new(vec![]);
        
        let rule = create_test_rule();
        validator.add_rule(rule);
        
        assert_eq!(validator.get_rules().len(), 1);
        
        let removed = validator.remove_rule("rule_001");
        assert!(removed);
        assert_eq!(validator.get_rules().len(), 0);
    }
}
