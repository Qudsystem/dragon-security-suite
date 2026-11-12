use super::actions::*;
use std::collections::HashSet;
use thiserror::Error;
use tracing::{info, warn, error};

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("Action not allowed: {0}")]
    Forbidden(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Approval required")]
    ApprovalRequired,
}

/// Response executor with Constitutional Guardian validation
pub struct ResponseExecutor {
    allowed_actions: HashSet<ActionType>,
    audit_log: Vec<AuditLogEntry>,
    dry_run: bool,
}

impl ResponseExecutor {
    pub fn new() -> Self {
        let mut allowed_actions = HashSet::new();
        allowed_actions.insert(ActionType::Alert);
        allowed_actions.insert(ActionType::Log);
        allowed_actions.insert(ActionType::KillProcess);
        allowed_actions.insert(ActionType::BlockNetwork);

        Self {
            allowed_actions,
            audit_log: Vec::new(),
            dry_run: false,
        }
    }

    /// Enable dry-run mode (simulation only)
    pub fn with_dry_run(mut self) -> Self {
        self.dry_run = true;
        self
    }

    /// Execute an action with Constitutional Guardian validation
    pub async fn execute(&mut self, action: Action, context: &ExecutionContext) -> Result<(), ExecutionError> {
        info!("Executing action: {:?}", action.action_type);

        // Constitutional Guardian validation
        if !self.is_action_allowed(&action, context) {
            warn!("Action blocked by Constitutional Guardian: {:?}", action.action_type);
            
            let entry = AuditLogEntry::new(
                action.clone(),
                context.triggering_event_id,
                ValidationStatus::Rejected,
            )
            .mark_failed("Blocked by Constitutional Guardian".to_string());
            
            self.audit_log.push(entry);
            
            return Err(ExecutionError::Forbidden(
                "Action violates business policies".to_string()
            ));
        }

        // Check if approval is required
        if action.requires_approval && action.approved_by.is_none() {
            warn!("Action requires human approval: {:?}", action.action_type);
            
            let entry = AuditLogEntry::new(
                action.clone(),
                context.triggering_event_id,
                ValidationStatus::RequiresHumanApproval,
            );
            
            self.audit_log.push(entry);
            
            return Err(ExecutionError::ApprovalRequired);
        }

        // Create audit log entry before execution
        let mut entry = AuditLogEntry::new(
            action.clone(),
            context.triggering_event_id,
            ValidationStatus::Approved,
        );

        if let Some(operator) = &context.operator_id {
            entry = entry.with_operator(operator.clone());
        }

        // Execute the action
        let result = if self.dry_run {
            info!("DRY RUN: Would execute {:?}", action.action_type);
            Ok(())
        } else {
            self.execute_action(&action).await
        };

        // Update audit log based on result
        match result {
            Ok(()) => {
                info!("Action executed successfully: {:?}", action.action_type);
                entry = entry.mark_success();
                self.audit_log.push(entry);
                Ok(())
            }
            Err(e) => {
                error!("Action execution failed: {:?} - {}", action.action_type, e);
                entry = entry.mark_failed(e.to_string());
                self.audit_log.push(entry);
                Err(e)
            }
        }
    }

    /// Constitutional Guardian validation logic
    fn is_action_allowed(&self, action: &Action, context: &ExecutionContext) -> bool {
        // Check if action type is allowed
        if !self.allowed_actions.contains(&action.action_type) {
            return false;
        }

        // Business-critical system protection
        if context.is_business_critical {
            // During business hours, require approval
            if context.is_business_hours() {
                return action.approved_by.is_some();
            }
        }

        // High-severity threats can execute immediately if pre-approved
        if action.severity as u8 > 90 {
            return true;
        }

        true
    }

    /// Execute the actual action (deterministic, not AI-generated)
    async fn execute_action(&self, action: &Action) -> Result<(), ExecutionError> {
        match action.action_type {
            ActionType::Alert => self.send_alert(action).await,
            ActionType::KillProcess => self.kill_process(action).await,
            ActionType::BlockNetwork => self.block_network(action).await,
            ActionType::IsolateHost => self.isolate_host(action).await,
            ActionType::LockUser => self.lock_user(action).await,
            ActionType::Log => self.log_event(action).await,
        }
    }

    async fn send_alert(&self, action: &Action) -> Result<(), ExecutionError> {
        // TODO: Implement alert sending (email, webhook, etc.)
        info!("Alert sent: {}", action.reason);
        Ok(())
    }

    async fn kill_process(&self, action: &Action) -> Result<(), ExecutionError> {
        if let ActionTarget::Process { pid } = action.target {
            // TODO: Implement process termination using system calls
            info!("Would kill process: {}", pid);
            Ok(())
        } else {
            Err(ExecutionError::ExecutionFailed("Invalid target for kill_process".to_string()))
        }
    }

    async fn block_network(&self, action: &Action) -> Result<(), ExecutionError> {
        if let ActionTarget::Network { ip, port } = &action.target {
            // TODO: Implement network blocking using iptables
            info!("Would block network: {}:{}", ip, port);
            Ok(())
        } else {
            Err(ExecutionError::ExecutionFailed("Invalid target for block_network".to_string()))
        }
    }

    async fn isolate_host(&self, action: &Action) -> Result<(), ExecutionError> {
        if let ActionTarget::Host { hostname } = &action.target {
            // TODO: Implement host isolation
            info!("Would isolate host: {}", hostname);
            Ok(())
        } else {
            Err(ExecutionError::ExecutionFailed("Invalid target for isolate_host".to_string()))
        }
    }

    async fn lock_user(&self, action: &Action) -> Result<(), ExecutionError> {
        if let ActionTarget::User { username } = &action.target {
            // TODO: Implement user account locking using PAM
            info!("Would lock user: {}", username);
            Ok(())
        } else {
            Err(ExecutionError::ExecutionFailed("Invalid target for lock_user".to_string()))
        }
    }

    async fn log_event(&self, action: &Action) -> Result<(), ExecutionError> {
        info!("Event logged: {}", action.reason);
        Ok(())
    }

    /// Get audit log
    pub fn get_audit_log(&self) -> &[AuditLogEntry] {
        &self.audit_log
    }

    /// Get last audit entry
    pub fn get_last_audit_entry(&self) -> Option<&AuditLogEntry> {
        self.audit_log.last()
    }
}

impl Default for ResponseExecutor {
    fn default() -> Self {
        Self::new()
    }
}

/// Execution context for action validation
pub struct ExecutionContext {
    pub triggering_event_id: uuid::Uuid,
    pub is_business_critical: bool,
    pub operator_id: Option<String>,
    current_hour: u32,
}

impl ExecutionContext {
    pub fn new(triggering_event_id: uuid::Uuid) -> Self {
        Self {
            triggering_event_id,
            is_business_critical: false,
            operator_id: None,
            current_hour: chrono::Utc::now().hour(),
        }
    }

    pub fn with_business_critical(mut self) -> Self {
        self.is_business_critical = true;
        self
    }

    pub fn with_operator(mut self, operator_id: String) -> Self {
        self.operator_id = Some(operator_id);
        self
    }

    fn is_business_hours(&self) -> bool {
        self.current_hour >= 9 && self.current_hour < 17
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::models::Severity;

    #[tokio::test]
    async fn test_execute_allowed_action() {
        let mut executor = ResponseExecutor::new().with_dry_run();
        
        let action = Action::new(
            ActionType::Alert,
            ActionTarget::None,
            "Test alert".to_string(),
            Severity::Medium,
        );

        let context = ExecutionContext::new(uuid::Uuid::new_v4());
        
        let result = executor.execute(action, &context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_execute_requires_approval() {
        let mut executor = ResponseExecutor::new().with_dry_run();
        
        let action = Action::new(
            ActionType::IsolateHost,
            ActionTarget::Host { hostname: "server1".to_string() },
            "Compromised host".to_string(),
            Severity::Critical,
        )
        .with_approval_required();

        let context = ExecutionContext::new(uuid::Uuid::new_v4())
            .with_business_critical();
        
        let result = executor.execute(action, &context).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_audit_log() {
        let mut executor = ResponseExecutor::new().with_dry_run();
        
        let action = Action::new(
            ActionType::Log,
            ActionTarget::None,
            "Test log".to_string(),
            Severity::Low,
        );

        let context = ExecutionContext::new(uuid::Uuid::new_v4());
        
        executor.execute(action, &context).await.unwrap();
        
        assert_eq!(executor.get_audit_log().len(), 1);
        assert!(executor.get_last_audit_entry().is_some());
    }
}
