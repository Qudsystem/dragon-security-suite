use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Action to be executed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub id: Uuid,
    pub action_type: ActionType,
    pub target: ActionTarget,
    pub reason: String,
    pub severity: crate::parser::models::Severity,
    pub requires_approval: bool,
    pub approved_by: Option<String>,
    pub executed: bool,
    pub execution_time: Option<DateTime<Utc>>,
}

impl Action {
    pub fn new(
        action_type: ActionType,
        target: ActionTarget,
        reason: String,
        severity: crate::parser::models::Severity,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            action_type,
            target,
            reason,
            severity,
            requires_approval: false,
            approved_by: None,
            executed: false,
            execution_time: None,
        }
    }

    pub fn with_approval_required(mut self) -> Self {
        self.requires_approval = true;
        self
    }

    pub fn approve(mut self, approver: String) -> Self {
        self.approved_by = Some(approver);
        self
    }

    pub fn mark_executed(mut self) -> Self {
        self.executed = true;
        self.execution_time = Some(Utc::now());
        self
    }
}

/// Types of actions that can be executed
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionType {
    /// Send alert notification
    Alert,
    /// Terminate a process
    KillProcess,
    /// Block network connection
    BlockNetwork,
    /// Isolate entire host from network
    IsolateHost,
    /// Lock user account
    LockUser,
    /// Log event only
    Log,
}

/// Target of an action
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ActionTarget {
    Process { pid: u32 },
    Network { ip: String, port: u16 },
    Host { hostname: String },
    User { username: String },
    None,
}

/// Audit log entry for executed actions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub action: Action,
    pub triggering_event_id: Uuid,
    pub ai_recommendation: Option<String>,
    pub guardian_validation: ValidationStatus,
    pub execution_status: ExecutionStatus,
    pub operator_id: Option<String>,
    pub error_message: Option<String>,
}

impl AuditLogEntry {
    pub fn new(
        action: Action,
        triggering_event_id: Uuid,
        guardian_validation: ValidationStatus,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            action,
            triggering_event_id,
            ai_recommendation: None,
            guardian_validation,
            execution_status: ExecutionStatus::Pending,
            operator_id: None,
            error_message: None,
        }
    }

    pub fn with_ai_recommendation(mut self, recommendation: String) -> Self {
        self.ai_recommendation = Some(recommendation);
        self
    }

    pub fn with_operator(mut self, operator_id: String) -> Self {
        self.operator_id = Some(operator_id);
        self
    }

    pub fn mark_success(mut self) -> Self {
        self.execution_status = ExecutionStatus::Success;
        self
    }

    pub fn mark_failed(mut self, error: String) -> Self {
        self.execution_status = ExecutionStatus::Failed;
        self.error_message = Some(error);
        self
    }
}

/// Validation status from Constitutional Guardian
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidationStatus {
    Approved,
    Rejected,
    RequiresHumanApproval,
}

/// Execution status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionStatus {
    Pending,
    Success,
    Failed,
    Cancelled,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::Severity;

    #[test]
    fn test_action_creation() {
        let action = Action::new(
            ActionType::KillProcess,
            ActionTarget::Process { pid: 1234 },
            "Malicious process detected".to_string(),
            Severity::Critical,
        );

        assert_eq!(action.action_type, ActionType::KillProcess);
        assert!(!action.requires_approval);
        assert!(!action.executed);
    }

    #[test]
    fn test_action_approval_flow() {
        let action = Action::new(
            ActionType::IsolateHost,
            ActionTarget::Host { hostname: "server1".to_string() },
            "Compromised host".to_string(),
            Severity::Critical,
        )
        .with_approval_required()
        .approve("admin@example.com".to_string());

        assert!(action.requires_approval);
        assert_eq!(action.approved_by, Some("admin@example.com".to_string()));
    }

    #[test]
    fn test_audit_log_entry() {
        let action = Action::new(
            ActionType::Alert,
            ActionTarget::None,
            "Test alert".to_string(),
            Severity::Low,
        );

        let entry = AuditLogEntry::new(
            action,
            Uuid::new_v4(),
            ValidationStatus::Approved,
        )
        .with_ai_recommendation("Block this process".to_string())
        .with_operator("operator1".to_string())
        .mark_success();

        assert_eq!(entry.execution_status, ExecutionStatus::Success);
        assert!(entry.ai_recommendation.is_some());
        assert!(entry.operator_id.is_some());
    }
}
