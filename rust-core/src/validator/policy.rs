use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::parser::models::{Event, Severity};

/// Policy rule for event validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub id: String,
    pub name: String,
    pub description: String,
    pub severity: Severity,
    pub enabled: bool,
    pub condition: Condition,
    pub action: Action,
    pub metadata: RuleMetadata,
}

/// Rule metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub author: String,
    pub tags: Vec<String>,
}

/// Condition for rule matching
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub field: String,
    pub operator: Operator,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logical_op: Option<LogicalOperator>,
    #[serde(default)]
    pub sub_conditions: Vec<Condition>,
}

impl Condition {
    /// Check if event matches this condition
    pub fn matches(&self, event: &Event) -> bool {
        let field_value = self.extract_field_value(event);
        
        let current_match = match &field_value {
            Some(value) => self.compare_values(value, &self.value),
            None => false,
        };

        // Handle sub-conditions with logical operators
        if self.sub_conditions.is_empty() {
            return current_match;
        }

        match self.logical_op {
            Some(LogicalOperator::And) => {
                current_match && self.sub_conditions.iter().all(|c| c.matches(event))
            }
            Some(LogicalOperator::Or) => {
                current_match || self.sub_conditions.iter().any(|c| c.matches(event))
            }
            Some(LogicalOperator::Not) => {
                current_match && !self.sub_conditions.iter().any(|c| c.matches(event))
            }
            None => current_match,
        }
    }

    fn extract_field_value(&self, event: &Event) -> Option<serde_json::Value> {
        // Extract field value from event based on field path
        match self.field.as_str() {
            "process.name" => Some(serde_json::json!(event.process.name)),
            "process.pid" => Some(serde_json::json!(event.process.pid)),
            "process.user" => Some(serde_json::json!(event.process.user)),
            "severity" => Some(serde_json::json!(event.severity)),
            "event_type" => Some(serde_json::json!(event.event_type)),
            _ => None,
        }
    }

    fn compare_values(&self, field_value: &serde_json::Value, rule_value: &serde_json::Value) -> bool {
        match self.operator {
            Operator::Equals => field_value == rule_value,
            Operator::NotEquals => field_value != rule_value,
            Operator::Contains => {
                if let (Some(field_str), Some(rule_str)) = (field_value.as_str(), rule_value.as_str()) {
                    field_str.contains(rule_str)
                } else {
                    false
                }
            }
            Operator::Matches => {
                // TODO: Implement regex matching
                false
            }
            Operator::GreaterThan => {
                if let (Some(field_num), Some(rule_num)) = (field_value.as_f64(), rule_value.as_f64()) {
                    field_num > rule_num
                } else {
                    false
                }
            }
            Operator::LessThan => {
                if let (Some(field_num), Some(rule_num)) = (field_value.as_f64(), rule_value.as_f64()) {
                    field_num < rule_num
                } else {
                    false
                }
            }
        }
    }
}

/// Comparison operators
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Operator {
    Equals,
    NotEquals,
    Contains,
    Matches,  // Regex
    GreaterThan,
    LessThan,
}

/// Logical operators for combining conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// Action to take when rule matches
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub target: Option<String>,
    pub reason: String,
    pub requires_approval: bool,
}

/// Types of actions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    Alert,
    KillProcess,
    BlockNetwork,
    IsolateHost,
    LockUser,
    Log,
}

/// Validation result
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub matched: bool,
    pub rule_id: Option<String>,
    pub severity: Severity,
    pub action: Option<Action>,
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            matched: false,
            rule_id: None,
            severity: Severity::Low,
            action: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::*;
    use std::path::PathBuf;

    fn create_test_event() -> Event {
        Event {
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            event_type: EventType::Syscall,
            severity: Severity::High,
            process: ProcessInfo {
                pid: 1234,
                name: "suspicious_binary".to_string(),
                exe_path: PathBuf::from("/tmp/malware"),
                parent_pid: 1000,
                parent_name: "bash".to_string(),
                user: "www-data".to_string(),
                uid: 33,
                command_line: "/tmp/malware --evil".to_string(),
                start_time: chrono::Utc::now(),
            },
            syscall: None,
            network: None,
            file: None,
            metadata: HashMap::new(),
        }
    }

    #[test]
    fn test_condition_equals() {
        let condition = Condition {
            field: "process.name".to_string(),
            operator: Operator::Equals,
            value: serde_json::json!("suspicious_binary"),
            logical_op: None,
            sub_conditions: vec![],
        };

        let event = create_test_event();
        assert!(condition.matches(&event));
    }

    #[test]
    fn test_condition_contains() {
        let condition = Condition {
            field: "process.name".to_string(),
            operator: Operator::Contains,
            value: serde_json::json!("suspicious"),
            logical_op: None,
            sub_conditions: vec![],
        };

        let event = create_test_event();
        assert!(condition.matches(&event));
    }

    #[test]
    fn test_condition_and() {
        let condition = Condition {
            field: "process.name".to_string(),
            operator: Operator::Contains,
            value: serde_json::json!("suspicious"),
            logical_op: Some(LogicalOperator::And),
            sub_conditions: vec![
                Condition {
                    field: "process.user".to_string(),
                    operator: Operator::Equals,
                    value: serde_json::json!("www-data"),
                    logical_op: None,
                    sub_conditions: vec![],
                }
            ],
        };

        let event = create_test_event();
        assert!(condition.matches(&event));
    }
}
