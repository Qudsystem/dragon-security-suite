# Design Document — Autonomous Mode (الوضع التلقائي الكامل)

## Overview

The Autonomous Mode feature enables Dragon R1 to execute penetration testing operations from target input to completion without human intervention, while maintaining safety through configurable constraints and comprehensive monitoring. This design builds upon the existing three-phase execution model (Informational → Exploitation → Post-exploitation) and extends it with autonomous decision-making capabilities.

The system supports two operation modes:
1. **Supervised Mode** (default): Requires explicit human approval for exploitation and post-exploitation phases
2. **Full Autonomous Mode**: Operates within predefined constraints without human approval

This design ensures that autonomous operation never compromises safety by implementing multiple layers of protection: constraint templates, real-time monitoring, emergency stop mechanisms, and comprehensive audit trails.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     User Interface Layer                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Mode         │  │ Constraint   │  │ Autonomous   │      │
│  │ Selection UI │  │ Template UI  │  │ Dashboard    │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                   Autonomous Mode Manager                    │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Mode         │  │ Constraint   │  │ Emergency    │      │
│  │ Controller   │  │ Evaluator    │  │ Stop Handler │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Execution Layer                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ LangGraph    │  │ Phase        │  │ MCP Tools    │      │
│  │ Agent        │  │ Manager      │  │ (15 tools)   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Storage & Monitoring                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ Neo4j        │  │ mem0         │  │ Audit Log    │      │
│  │ (Graph)      │  │ (Memory)     │  │ (PostgreSQL) │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
└─────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

1. **Mode Selection**: User selects mode during project/task creation
2. **Constraint Application**: System applies selected constraint template
3. **Autonomous Execution**: LangGraph agent executes with constraint checks
4. **Real-Time Monitoring**: Constraint evaluator monitors all actions
5. **Emergency Control**: User can stop execution at any time
6. **Audit & Learning**: All actions logged and learned from

## Components and Interfaces

### 1. Mode Controller

**Responsibility**: Manages operation mode selection, validation, and transitions.

**Key Methods**:
```python
class ModeController:
    def select_mode(self, task_id: str, mode: OperationMode, user_id: str) -> ModeSelectionResult
    def confirm_autonomous_mode(self, task_id: str, confirmation_phrase: str, user_id: str) -> bool
    def switch_mode(self, task_id: str, new_mode: OperationMode, reason: str, user_id: str) -> ModeTransitionResult
    def get_current_mode(self, task_id: str) -> OperationMode
    def validate_mode_transition(self, task_id: str, from_mode: OperationMode, to_mode: OperationMode) -> ValidationResult
```

**Interfaces**:
- Input: Mode selection requests, confirmation inputs, transition requests
- Output: Mode selection results, validation results, audit log entries
- Storage: Task metadata (mode, template_id, confirmation_timestamp)

### 2. Constraint Template Manager

**Responsibility**: Manages constraint template definitions, validation, and application.

**Key Methods**:
```python
class ConstraintTemplateManager:
    def create_template(self, template: ConstraintTemplate, admin_id: str) -> str
    def validate_template(self, template: ConstraintTemplate) -> ValidationResult
    def apply_template(self, task_id: str, template_id: str) -> ApplicationResult
    def get_predefined_templates(self) -> List[ConstraintTemplate]
    def get_template(self, template_id: str) -> ConstraintTemplate
```

**Constraint Template Structure**:
```python
@dataclass
class ConstraintTemplate:
    id: str
    name: str
    description: str
    scope_limits: ScopeLimits
    tool_restrictions: ToolRestrictions
    time_limits: TimeLimits
    risk_thresholds: RiskThresholds
    created_by: str
    created_at: datetime
    version: int

@dataclass
class ScopeLimits:
    ip_ranges: List[str]  # CIDR notation
    domain_patterns: List[str]  # Regex patterns
    port_ranges: List[Tuple[int, int]]
    protocols: List[str]  # tcp, udp, http, https, etc.

@dataclass
class ToolRestrictions:
    mode: str  # "whitelist" or "blacklist"
    tools: List[str]  # MCP tool names

@dataclass
class TimeLimits:
    max_task_duration: int  # seconds
    max_phase_duration: int  # seconds
    max_tool_duration: int  # seconds

@dataclass
class RiskThresholds:
    max_exploitation_risk: float  # 0.0 to 1.0
    max_post_exploitation_risk: float  # 0.0 to 1.0
```

### 3. Constraint Evaluator

**Responsibility**: Evaluates actions against active constraints in real-time.

**Key Methods**:
```python
class ConstraintEvaluator:
    def evaluate_action(self, task_id: str, action: Action) -> EvaluationResult
    def check_scope_compliance(self, action: Action, scope_limits: ScopeLimits) -> bool
    def check_tool_allowed(self, tool_name: str, restrictions: ToolRestrictions) -> bool
    def check_time_limit(self, task_id: str, phase: str, limits: TimeLimits) -> bool
    def check_risk_threshold(self, action: Action, thresholds: RiskThresholds) -> bool
    def calculate_violation_rate(self, task_id: str, window: int) -> float
```

**Evaluation Result**:
```python
@dataclass
class EvaluationResult:
    compliant: bool
    violations: List[ConstraintViolation]
    near_violations: List[NearViolation]
    decision: str  # "allow", "block", "warn"
    reason: str

@dataclass
class ConstraintViolation:
    constraint_type: str
    constraint_value: Any
    actual_value: Any
    severity: str  # "low", "medium", "high", "critical"
    timestamp: datetime
```

### 4. Autonomous Execution Engine

**Responsibility**: Orchestrates autonomous execution with constraint enforcement.

**Key Methods**:
```python
class AutonomousExecutionEngine:
    def execute_task(self, task_id: str) -> ExecutionResult
    def execute_phase(self, task_id: str, phase: str) -> PhaseResult
    def execute_action(self, task_id: str, action: Action) -> ActionResult
    def handle_constraint_violation(self, task_id: str, violation: ConstraintViolation) -> None
    def check_continue_execution(self, task_id: str) -> bool
```

**Integration with LangGraph**:
```python
# Extended LangGraph state for autonomous mode
@dataclass
class AutonomousAgentState(AgentState):
    operation_mode: OperationMode
    active_constraints: ConstraintTemplate
    violation_count: int
    violation_rate: float
    emergency_stop_requested: bool
```

### 5. Emergency Stop Handler

**Responsibility**: Handles emergency stop requests and graceful shutdown.

**Key Methods**:
```python
class EmergencyStopHandler:
    def request_stop(self, task_id: str, user_id: str, reason: str) -> None
    def execute_stop(self, task_id: str) -> StopResult
    def save_partial_state(self, task_id: str) -> None
    def generate_partial_report(self, task_id: str) -> Report
    def cleanup_resources(self, task_id: str) -> None
```

### 6. Monitoring & Audit System

**Responsibility**: Tracks all autonomous operations and maintains audit trail.

**Key Methods**:
```python
class AutonomousMonitor:
    def log_action(self, task_id: str, action: Action, result: ActionResult, evaluation: EvaluationResult) -> None
    def log_mode_transition(self, task_id: str, from_mode: OperationMode, to_mode: OperationMode, reason: str, user_id: str) -> None
    def log_constraint_evaluation(self, task_id: str, evaluation: EvaluationResult) -> None
    def log_emergency_stop(self, task_id: str, user_id: str, reason: str) -> None
    def query_audit_log(self, filters: AuditFilters) -> List[AuditEntry]
    def calculate_metrics(self, task_id: str) -> AutonomousMetrics
```

### 7. Learning & Adaptation Engine

**Responsibility**: Learns from autonomous executions and improves decision-making.

**Key Methods**:
```python
class LearningEngine:
    def store_execution_pattern(self, task_id: str, pattern: ExecutionPattern) -> None
    def get_tool_success_rate(self, tool_name: str, context: Dict) -> float
    def improve_risk_assessment(self, action: Action, actual_outcome: Outcome) -> None
    def suggest_constraint_adjustments(self, task_id: str) -> List[ConstraintAdjustment]
    def version_knowledge(self, update: KnowledgeUpdate) -> str
```

## Data Models

### Database Schema Extensions

**Tasks Table** (extend existing):
```sql
ALTER TABLE tasks ADD COLUMN operation_mode VARCHAR(20) DEFAULT 'supervised';
ALTER TABLE tasks ADD COLUMN constraint_template_id UUID;
ALTER TABLE tasks ADD COLUMN autonomous_confirmation_at TIMESTAMP;
ALTER TABLE tasks ADD COLUMN autonomous_confirmed_by UUID;
```

**Constraint Templates Table** (new):
```sql
CREATE TABLE constraint_templates (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    scope_limits JSONB NOT NULL,
    tool_restrictions JSONB NOT NULL,
    time_limits JSONB NOT NULL,
    risk_thresholds JSONB NOT NULL,
    is_predefined BOOLEAN DEFAULT FALSE,
    created_by UUID REFERENCES users(id),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    version INTEGER DEFAULT 1
);
```

**Autonomous Audit Log Table** (new):
```sql
CREATE TABLE autonomous_audit_log (
    id UUID PRIMARY KEY,
    task_id UUID REFERENCES tasks(id),
    timestamp TIMESTAMP DEFAULT NOW(),
    event_type VARCHAR(50) NOT NULL,  -- 'action', 'mode_transition', 'constraint_evaluation', 'emergency_stop'
    action_details JSONB,
    constraint_evaluation JSONB,
    user_id UUID REFERENCES users(id),
    result VARCHAR(20),  -- 'allowed', 'blocked', 'completed', 'failed'
    INDEX idx_task_timestamp (task_id, timestamp),
    INDEX idx_event_type (event_type),
    INDEX idx_result (result)
);
```

**Constraint Violations Table** (new):
```sql
CREATE TABLE constraint_violations (
    id UUID PRIMARY KEY,
    task_id UUID REFERENCES tasks(id),
    timestamp TIMESTAMP DEFAULT NOW(),
    constraint_type VARCHAR(50) NOT NULL,
    constraint_value JSONB NOT NULL,
    actual_value JSONB NOT NULL,
    severity VARCHAR(20) NOT NULL,
    action_details JSONB,
    INDEX idx_task_severity (task_id, severity),
    INDEX idx_timestamp (timestamp)
);
```

### Neo4j Graph Extensions

**New Node Types**:
- `ConstraintTemplate`: Represents a constraint template
- `AutonomousExecution`: Represents an autonomous task execution
- `ConstraintViolation`: Represents a constraint violation event

**New Relationships**:
- `(:Task)-[:USES_TEMPLATE]->(:ConstraintTemplate)`
- `(:Task)-[:HAS_EXECUTION]->(:AutonomousExecution)`
- `(:AutonomousExecution)-[:VIOLATED]->(:ConstraintViolation)`
- `(:AutonomousExecution)-[:EXECUTED_ACTION]->(:Action)`

### mem0 Memory Structure

**Autonomous Execution Context**:
```python
{
    "task_id": "uuid",
    "operation_mode": "autonomous",
    "constraint_template": {
        "id": "uuid",
        "name": "Balanced",
        "constraints": {...}
    },
    "execution_history": [
        {
            "phase": "reconnaissance",
            "actions": [...],
            "violations": [...],
            "duration": 120
        }
    ],
    "learned_patterns": {
        "successful_tactics": [...],
        "failed_approaches": [...],
        "constraint_compliance_rate": 0.95
    }
}
```

## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Mode Selection Always Available
*For any* project or task creation flow, the mode selection UI should be displayed and allow selection between Supervised and Full Autonomous modes.
**Validates: Requirements 1.1**

### Property 2: Supervised Mode Requires Approval
*For any* task in Supervised Mode, when an exploitation or post-exploitation action is attempted, an approval workflow should be triggered before execution.
**Validates: Requirements 1.2**

### Property 3: Default Mode is Supervised
*For any* task created without explicit mode selection, retrieving the task should return Supervised as the operation mode.
**Validates: Requirements 1.4**

### Property 4: Mode Persistence
*For any* mode selection, storing and then retrieving the task/project should return the same operation mode.
**Validates: Requirements 1.5**

### Property 5: Double Confirmation Required
*For any* attempt to enable Full Autonomous Mode, both the confirmation phrase entry and the explicit button click must be completed before the mode is activated.
**Validates: Requirements 2.2, 2.3, 2.4**

### Property 6: Incomplete Confirmation Reverts to Supervised
*For any* autonomous mode activation attempt where either confirmation step is not completed, the system should set the mode to Supervised.
**Validates: Requirements 2.5**

### Property 7: Template Selection Required for Autonomous
*For any* task, Full Autonomous Mode should not be activated unless a constraint template has been selected and applied.
**Validates: Requirements 3.5**

### Property 8: Template Application Completeness
*For any* selected constraint template, all constraints defined in that template should be active and enforced for the task.
**Validates: Requirements 3.3**

### Property 9: Template Persistence
*For any* applied constraint template, storing and then retrieving the task should return the same template ID and constraints.
**Validates: Requirements 3.4**

### Property 10: Constraint Validation Before Activation
*For any* constraint template, all constraint definitions should be validated (syntax, references, ranges) before the template can be activated for a task.
**Validates: Requirements 14.1, 14.2, 14.3, 14.4**

### Property 11: Action Constraint Compliance Check
*For any* action in autonomous mode, the action should be checked against all active constraints before execution, and violating actions should be blocked.
**Validates: Requirements 5.2, 5.3**

### Property 12: Autonomous Execution Without Approval
*For any* task in Full Autonomous Mode with valid constraints, all three phases (reconnaissance, exploitation, post-exploitation) should execute without requiring human approval.
**Validates: Requirements 5.1**

### Property 13: Violation Logging Completeness
*For any* constraint violation, the system should log the violation with timestamp, action details, constraint details, and severity.
**Validates: Requirements 5.3, 6.3**

### Property 14: Graceful Phase Completion
*For any* phase where all actions violate constraints, the system should complete the phase and proceed to the next phase or task completion rather than hanging.
**Validates: Requirements 5.4**

### Property 15: Report Includes Violations
*For any* completed autonomous task, the generated report should include a section listing all constraint violations that occurred during execution.
**Validates: Requirements 5.5, 11.1, 11.2**

### Property 16: Real-Time Monitoring
*For any* action in autonomous mode, the constraint evaluator should monitor and evaluate the action in real-time before execution.
**Validates: Requirements 6.1, 6.2**

### Property 17: Violation Rate Threshold Enforcement
*For any* autonomous task, when the violation rate exceeds the configured threshold, the system should pause execution and notify administrators.
**Validates: Requirements 6.4, 6.5**

### Property 18: Emergency Stop Immediacy
*For any* emergency stop request, all ongoing tool executions should halt immediately without completing their current operation.
**Validates: Requirements 7.2**

### Property 19: State Preservation on Stop
*For any* emergency stop, the current state and all partial results should be saved to Neo4j and mem0 before the stop completes.
**Validates: Requirements 7.3**

### Property 20: Partial Report Generation
*For any* emergency stop, a partial report should be generated containing all findings and actions up to the stop point.
**Validates: Requirements 7.4**

### Property 21: Comprehensive Action Logging
*For any* action in autonomous mode, the audit log should contain an entry with timestamp, tool used, target, result, and constraint evaluation.
**Validates: Requirements 8.1, 8.2**

### Property 22: Mode Transition Logging
*For any* mode transition (supervised ↔ autonomous), the audit log should contain an entry with timestamp, reason, and user identity.
**Validates: Requirements 8.3, 10.5**

### Property 23: Audit Log Query Capabilities
*For any* audit log query, the system should support filtering by task, project, time range, tool, and constraint status.
**Validates: Requirements 8.5**

### Property 24: Mode Switch Graceful Transition
*For any* mode switch from autonomous to supervised during execution, the current tool execution should complete before the transition takes effect.
**Validates: Requirements 10.2**

### Property 25: Post-Transition Approval Requirement
*For any* task that transitions from autonomous to supervised mode, the next high-risk action should require approval.
**Validates: Requirements 10.3**

### Property 26: Learning Data Persistence
*For any* completed autonomous task, execution patterns, successful tactics, and constraint compliance should be stored in mem0.
**Validates: Requirements 12.1**

### Property 27: Historical Success Rate Usage
*For any* tool selection in autonomous mode, the system should consider historical success rates from mem0 when choosing between alternative tools.
**Validates: Requirements 12.2**

### Property 28: Knowledge Versioning
*For any* learning update, the system should create a new version of the learned knowledge and maintain an audit trail of the update.
**Validates: Requirements 12.5**

### Property 29: Performance Tracking Granularity
*For any* autonomous task, execution time should be tracked at both phase level and tool level.
**Validates: Requirements 13.1**

### Property 30: Dashboard Completeness
*For any* access to the autonomous mode dashboard, all active autonomous tasks should be displayed with name, phase, progress, compliance status, and elapsed time.
**Validates: Requirements 15.1, 15.2**

## Error Handling

### Constraint Violation Handling
1. **Detection**: Constraint evaluator detects violation before action execution
2. **Blocking**: Action is prevented from executing
3. **Logging**: Violation logged with full context
4. **Notification**: If violation rate exceeds threshold, administrators notified
5. **Continuation**: System continues with next action or phase

### Emergency Stop Handling
1. **Request**: User clicks emergency stop button
2. **Signal**: Stop signal sent to all active tool executions
3. **Halt**: Tools halt immediately (within 1 second)
4. **Save**: Current state saved to Neo4j and mem0
5. **Report**: Partial report generated
6. **Cleanup**: Resources released and connections closed
7. **Audit**: Stop event logged with full context

### Mode Transition Errors
1. **Validation**: Transition validated before execution
2. **Rollback**: If transition fails, mode reverts to previous state
3. **Notification**: User notified of failure with specific reason
4. **Logging**: Failed transition logged for audit

### Template Validation Errors
1. **Syntax Check**: All constraint syntax validated
2. **Reference Check**: Tool names validated against registry
3. **Range Check**: Numeric values validated for reasonableness
4. **Error Display**: Specific validation errors shown to user
5. **Activation Block**: Invalid templates cannot be activated

## Testing Strategy

### Unit Testing
- Mode controller: mode selection, confirmation, transitions
- Constraint evaluator: each constraint type evaluation
- Template manager: template CRUD, validation
- Emergency stop handler: stop execution, state preservation
- Learning engine: pattern storage, success rate calculation

### Property-Based Testing
Each of the 30 correctness properties will be implemented as a property-based test using Hypothesis (Python) or fast-check (TypeScript). Each test will run a minimum of 100 iterations with randomly generated inputs.

**Example Property Test Structure**:
```python
from hypothesis import given, strategies as st
import pytest

@given(
    task_id=st.uuids(),
    mode=st.sampled_from(['supervised', 'autonomous']),
    has_template=st.booleans()
)
def test_property_7_template_required_for_autonomous(task_id, mode, has_template):
    """Property 7: Template Selection Required for Autonomous"""
    # Feature: autonomous-mode, Property 7: Template Selection Required for Autonomous
    
    if mode == 'autonomous' and not has_template:
        with pytest.raises(ValidationError):
            mode_controller.activate_autonomous_mode(task_id)
    elif mode == 'autonomous' and has_template:
        result = mode_controller.activate_autonomous_mode(task_id)
        assert result.success == True
```

### Integration Testing
- End-to-end autonomous task execution
- Mode transitions during active execution
- Emergency stop during different phases
- Constraint violation handling across phases
- Audit log completeness verification

### Security Testing
- Constraint bypass attempts
- Unauthorized mode transitions
- Emergency stop authorization
- Audit log tampering detection
- Template injection attacks

### Performance Testing
- Constraint evaluation overhead (target: < 10ms per action)
- Dashboard load time with 100+ active tasks
- Audit log query performance with 1M+ entries
- Mode transition latency (target: < 100ms)

## Performance Considerations

### Constraint Evaluation Optimization
- Cache compiled regex patterns for domain matching
- Use interval trees for efficient IP range checking
- Batch constraint evaluations where possible
- Pre-compute tool whitelist/blacklist lookups

### Monitoring Overhead
- Asynchronous audit logging to avoid blocking execution
- Batch violation rate calculations (every 10 actions)
- Efficient time-series storage for performance metrics
- Lazy loading of dashboard data

### Memory Management
- Limit in-memory audit log size (rolling window)
- Periodic cleanup of completed task data
- Efficient Neo4j query patterns for graph traversal
- mem0 memory pruning for old execution patterns

## Security Considerations

### Constraint Enforcement
- All constraints evaluated in trusted code (no user-provided constraint logic)
- Constraint templates validated before storage
- Template modifications require admin privileges
- Constraint bypass attempts logged and alerted

### Audit Trail Integrity
- Audit logs append-only (no deletion or modification)
- Cryptographic signatures on critical audit entries
- Tamper detection through log chain verification
- Regular audit log backups

### Emergency Stop Security
- Emergency stop requires authenticated user
- Stop requests rate-limited to prevent DoS
- Stop events logged with full user context
- Partial state encrypted before storage

### Mode Transition Security
- Mode transitions require authentication
- Autonomous mode activation requires double confirmation
- Transition to autonomous requires admin role (configurable)
- All transitions logged and auditable

## Deployment Considerations

### Configuration
- Default mode: Supervised (configured in environment)
- Predefined templates: Loaded from configuration files
- Violation thresholds: Configurable per deployment
- Emergency stop timeout: Configurable (default: 1 second)

### Migration
- Existing tasks default to Supervised mode
- Database schema migrations for new tables
- Neo4j graph schema updates for new node types
- Backward compatibility with existing approval workflows

### Monitoring
- Prometheus metrics for autonomous task counts
- Grafana dashboards for violation rates
- Alert rules for high violation rates
- Performance metrics for constraint evaluation

### Scaling
- Constraint evaluator can be horizontally scaled
- Audit log writes can be distributed
- Dashboard queries optimized with caching
- Learning engine updates batched for efficiency
