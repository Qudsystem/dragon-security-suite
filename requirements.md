# Requirements Document — Autonomous Mode (الوضع التلقائي الكامل)

## Introduction

This specification defines the **Full Autonomous Mode** for Dragon R1, enabling the system to execute penetration testing operations from target input to objective completion without human intervention, while maintaining safety through configurable constraints and comprehensive monitoring. This mode is designed to exceed XBOW capabilities by providing intelligent, self-directed security testing with built-in safeguards.

The autonomous mode allows Dragon R1 to operate in two distinct modes:
1. **Supervised Mode** (default): Requires explicit human approval for exploitation and post-exploitation phases
2. **Full Autonomous Mode**: Operates within predefined constraints without human approval, using constraint templates and intensive monitoring

## Glossary

- **Dragon R1**: The ethical penetration testing system
- **Autonomous Mode**: Operation mode where the system executes security testing without human approval for each action
- **Supervised Mode**: Operation mode requiring explicit human approval for high-risk actions
- **Constraint Template**: Predefined set of rules and limits that govern autonomous operation
- **Cooling Period**: Mandatory delay before executing approved high-risk actions
- **Phase Manager**: Component responsible for managing execution phases and approvals
- **LangGraph Agent**: The AI agent orchestrating tool selection and execution
- **MCP Tools**: Model Context Protocol tools (15 tools for security testing)
- **Neo4j**: Graph database storing attack surface and relationships
- **mem0**: Long-term memory system for context and learning
- **Task**: A single security testing objective (e.g., test a specific IP or domain)
- **Project**: A collection of related tasks under one engagement or client

## Requirements

### Requirement 1: Mode Selection at Project/Task Creation

**User Story:** As a penetration tester, I want to select the operation mode when creating a project or task, so that I can choose between supervised and autonomous execution based on the engagement requirements.

#### Acceptance Criteria

1. WHEN a user creates a new project or task THEN the system SHALL display mode selection options (Supervised or Full Autonomous)
2. WHEN a user selects Supervised Mode THEN the system SHALL require explicit approval for exploitation and post-exploitation phases
3. WHEN a user selects Full Autonomous Mode THEN the system SHALL display available constraint templates for selection
4. WHEN no mode is explicitly selected THEN the system SHALL default to Supervised Mode
5. WHEN a mode is selected THEN the system SHALL store the mode choice with the project/task metadata

### Requirement 2: Double Confirmation for Autonomous Mode

**User Story:** As a security-conscious user, I want a double confirmation process when enabling Full Autonomous Mode, so that I cannot accidentally enable unrestricted autonomous operation.

#### Acceptance Criteria

1. WHEN a user selects Full Autonomous Mode THEN the system SHALL display a warning dialog explaining the implications
2. WHEN the warning dialog is displayed THEN the system SHALL require the user to type a confirmation phrase (e.g., "I understand the risks")
3. WHEN the user provides the confirmation phrase THEN the system SHALL require a second explicit confirmation button click
4. WHEN both confirmations are completed THEN the system SHALL enable Full Autonomous Mode for the project/task
5. IF either confirmation is not completed THEN the system SHALL revert to Supervised Mode

### Requirement 3: Constraint Template System

**User Story:** As a penetration tester, I want to select from predefined constraint templates when enabling autonomous mode, so that I can ensure the system operates within acceptable boundaries for my engagement.

#### Acceptance Criteria

1. WHEN a user enables Full Autonomous Mode THEN the system SHALL display available constraint templates
2. WHEN displaying constraint templates THEN the system SHALL show template name, description, and key constraints for each template
3. WHEN a user selects a constraint template THEN the system SHALL apply all constraints defined in that template
4. WHEN a constraint template is applied THEN the system SHALL store the template ID and constraints with the task/project
5. WHEN no template is selected THEN the system SHALL require template selection before allowing autonomous mode activation

### Requirement 4: Constraint Template Definition and Management

**User Story:** As a security administrator, I want to define and manage constraint templates, so that I can create reusable constraint sets for different types of engagements.

#### Acceptance Criteria

1. WHEN an administrator creates a constraint template THEN the system SHALL allow specification of scope limits, tool restrictions, time limits, and risk thresholds
2. WHEN defining scope limits THEN the system SHALL support IP ranges, domain patterns, port ranges, and protocol restrictions
3. WHEN defining tool restrictions THEN the system SHALL allow whitelisting or blacklisting specific MCP tools
4. WHEN defining time limits THEN the system SHALL support maximum execution time per task and per phase
5. WHEN defining risk thresholds THEN the system SHALL support maximum risk score for automatic exploitation

### Requirement 5: Autonomous Execution Within Constraints

**User Story:** As a penetration tester, I want the system to execute all phases autonomously within the defined constraints, so that testing can proceed without constant supervision while remaining safe.

#### Acceptance Criteria

1. WHEN a task runs in Full Autonomous Mode THEN the system SHALL execute reconnaissance, exploitation, and post-exploitation phases without human approval
2. WHEN executing any action THEN the system SHALL verify the action complies with all active constraints before execution
3. WHEN an action violates a constraint THEN the system SHALL skip the action and log the constraint violation
4. WHEN all actions in a phase violate constraints THEN the system SHALL complete the phase and proceed to the next phase or completion
5. WHEN a task completes THEN the system SHALL generate a report including all executed actions and constraint violations

### Requirement 6: Real-Time Constraint Monitoring

**User Story:** As a security administrator, I want real-time monitoring of autonomous operations against constraints, so that I can detect and respond to unexpected behavior immediately.

#### Acceptance Criteria

1. WHEN a task runs in Full Autonomous Mode THEN the system SHALL monitor all actions against active constraints in real-time
2. WHEN monitoring actions THEN the system SHALL track constraint compliance, near-violations, and actual violations
3. WHEN a constraint is violated THEN the system SHALL log the violation with timestamp, action details, and constraint details
4. WHEN multiple violations occur THEN the system SHALL calculate a violation rate and compare against thresholds
5. WHEN violation rate exceeds threshold THEN the system SHALL pause autonomous execution and notify administrators

### Requirement 7: Emergency Stop Mechanism

**User Story:** As a penetration tester, I want an emergency stop button for autonomous tasks, so that I can immediately halt execution if unexpected issues arise.

#### Acceptance Criteria

1. WHEN viewing an autonomous task THEN the system SHALL display a prominent emergency stop button
2. WHEN the emergency stop button is clicked THEN the system SHALL immediately halt all ongoing tool executions
3. WHEN execution is halted THEN the system SHALL save the current state and partial results to Neo4j and mem0
4. WHEN execution is halted THEN the system SHALL generate a partial report with all findings up to the stop point
5. WHEN execution is halted THEN the system SHALL log the emergency stop event with timestamp and user identity

### Requirement 8: Autonomous Mode Audit Trail

**User Story:** As a compliance officer, I want a complete audit trail of all autonomous operations, so that I can verify compliance with policies and investigate incidents.

#### Acceptance Criteria

1. WHEN a task runs in Full Autonomous Mode THEN the system SHALL log every action with timestamp, tool used, target, and result
2. WHEN logging actions THEN the system SHALL include constraint evaluation results (compliant, near-violation, violation)
3. WHEN mode transitions occur THEN the system SHALL log the transition with reason and user identity (if applicable)
4. WHEN constraints are evaluated THEN the system SHALL log the evaluation with constraint details and decision
5. WHEN audit logs are queried THEN the system SHALL support filtering by task, project, time range, tool, and constraint status

### Requirement 9: Constraint Template Library

**User Story:** As a penetration tester, I want a library of predefined constraint templates for common scenarios, so that I can quickly configure autonomous mode for standard engagements.

#### Acceptance Criteria

1. WHEN accessing constraint templates THEN the system SHALL provide at least five predefined templates (Conservative, Balanced, Aggressive, Web-Only, Network-Only)
2. WHEN displaying the Conservative template THEN the system SHALL show constraints limiting to reconnaissance only with no exploitation
3. WHEN displaying the Balanced template THEN the system SHALL show constraints allowing low-risk exploitation with strict scope limits
4. WHEN displaying the Aggressive template THEN the system SHALL show constraints allowing broader exploitation within defined scope
5. WHEN displaying specialized templates (Web-Only, Network-Only) THEN the system SHALL show constraints limiting tools and targets to the specified domain

### Requirement 10: Mode Switching and Transition

**User Story:** As a penetration tester, I want to switch a running task from autonomous to supervised mode, so that I can take manual control if the situation requires it.

#### Acceptance Criteria

1. WHEN viewing a running autonomous task THEN the system SHALL display an option to switch to supervised mode
2. WHEN switching to supervised mode THEN the system SHALL complete the current tool execution before transitioning
3. WHEN the transition completes THEN the system SHALL require approval for the next high-risk action
4. WHEN switching from supervised to autonomous mode THEN the system SHALL require the full double confirmation process
5. WHEN a mode switch occurs THEN the system SHALL log the transition with timestamp, reason, and user identity

### Requirement 11: Constraint Violation Reporting

**User Story:** As a security administrator, I want detailed reports of constraint violations, so that I can refine constraint templates and improve autonomous operation safety.

#### Acceptance Criteria

1. WHEN a task completes THEN the system SHALL include a constraint violations section in the report
2. WHEN reporting violations THEN the system SHALL list each violation with action details, constraint violated, and timestamp
3. WHEN multiple violations of the same constraint occur THEN the system SHALL group them and show count
4. WHEN near-violations occur THEN the system SHALL report them separately with proximity to violation threshold
5. WHEN no violations occur THEN the system SHALL explicitly state that all actions complied with constraints

### Requirement 12: Autonomous Learning and Adaptation

**User Story:** As a penetration tester, I want the system to learn from autonomous executions and improve its decision-making, so that autonomous mode becomes more effective over time.

#### Acceptance Criteria

1. WHEN an autonomous task completes THEN the system SHALL store execution patterns, successful tactics, and constraint compliance in mem0
2. WHEN selecting tools in autonomous mode THEN the system SHALL consider historical success rates from mem0
3. WHEN evaluating risk THEN the system SHALL use learned patterns to improve risk assessment accuracy
4. WHEN constraints are frequently violated THEN the system SHALL suggest constraint template adjustments
5. WHEN learning updates occur THEN the system SHALL version the learned knowledge and maintain audit trail

### Requirement 13: Autonomous Mode Performance Monitoring

**User Story:** As a system administrator, I want performance metrics for autonomous mode operations, so that I can ensure the system operates efficiently and identify bottlenecks.

#### Acceptance Criteria

1. WHEN tasks run in autonomous mode THEN the system SHALL track execution time per phase and per tool
2. WHEN tracking performance THEN the system SHALL measure constraint evaluation overhead
3. WHEN monitoring resources THEN the system SHALL track CPU, memory, and network usage for autonomous tasks
4. WHEN performance degrades THEN the system SHALL log performance warnings and adjust execution strategy
5. WHEN performance metrics are queried THEN the system SHALL provide aggregated statistics across all autonomous tasks

### Requirement 14: Constraint Template Validation

**User Story:** As a security administrator, I want constraint templates to be validated before use, so that I can ensure templates are correctly configured and will function as intended.

#### Acceptance Criteria

1. WHEN a constraint template is created or modified THEN the system SHALL validate all constraint definitions
2. WHEN validating scope limits THEN the system SHALL verify IP ranges, domain patterns, and port ranges are syntactically correct
3. WHEN validating tool restrictions THEN the system SHALL verify all referenced tools exist in the MCP tool registry
4. WHEN validating time limits THEN the system SHALL verify limits are positive and reasonable (e.g., not exceeding 24 hours)
5. WHEN validation fails THEN the system SHALL prevent template activation and display specific validation errors

### Requirement 15: Autonomous Mode Dashboard

**User Story:** As a penetration tester, I want a dashboard showing all autonomous tasks and their status, so that I can monitor multiple autonomous operations simultaneously.

#### Acceptance Criteria

1. WHEN accessing the autonomous mode dashboard THEN the system SHALL display all active autonomous tasks
2. WHEN displaying tasks THEN the system SHALL show task name, current phase, progress, constraint compliance status, and elapsed time
3. WHEN a task violates constraints THEN the system SHALL highlight the task with a warning indicator
4. WHEN a task completes THEN the system SHALL show completion status and link to the report
5. WHEN filtering tasks THEN the system SHALL support filtering by project, constraint template, status, and violation status
