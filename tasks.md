# Implementation Plan — Autonomous Mode (الوضع التلقائي الكامل)

## Overview
This implementation plan converts the autonomous mode design into actionable coding tasks. Each task builds incrementally on previous work, ensuring no orphaned code. The plan follows a bottom-up approach: data models → core logic → UI → integration → testing.

---

## Tasks

- [x] 1. Set up database schema and data models


  - Create database migrations for new tables (constraint_templates, autonomous_audit_log, constraint_violations)
  - Extend tasks table with autonomous mode columns
  - Create SQLAlchemy/Prisma models for all new tables
  - _Requirements: 1.5, 3.4, 8.1, 11.1_

- [x] 1.1 Write property test for data model persistence

  - **Property 4: Mode Persistence**
  - **Validates: Requirements 1.5**

- [x] 1.2 Write property test for template persistence

  - **Property 9: Template Persistence**
  - **Validates: Requirements 3.4**

- [x] 2. Implement constraint template data structures


  - Create ConstraintTemplate, ScopeLimits, ToolRestrictions, TimeLimits, RiskThresholds dataclasses
  - Implement JSON serialization/deserialization for constraint structures
  - Create validation functions for each constraint type
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_

- [x] 2.1 Write property test for constraint serialization

  - **Property 10: Constraint Validation Before Activation**
  - **Validates: Requirements 14.1, 14.2, 14.3, 14.4**

- [x] 3. Implement ConstraintTemplateManager


  - Create ConstraintTemplateManager class with CRUD operations
  - Implement template validation logic (IP ranges, domain patterns, tool references, time limits)
  - Create five predefined templates (Conservative, Balanced, Aggressive, Web-Only, Network-Only)
  - Implement template storage and retrieval from database
  - _Requirements: 4.1, 9.1, 9.2, 9.3, 9.4, 9.5, 14.1_

- [x] 3.1 Write property test for template validation

  - **Property 10: Constraint Validation Before Activation**
  - **Validates: Requirements 14.1, 14.2, 14.3, 14.4**

- [x] 3.2 Write unit tests for predefined templates

  - Test each predefined template loads correctly
  - Verify template constraints match specifications
  - _Requirements: 9.1, 9.2, 9.3, 9.4, 9.5_

- [x] 4. Implement ConstraintEvaluator




  - Create ConstraintEvaluator class with evaluation methods
  - Implement scope compliance checking (IP ranges, domains, ports, protocols)
  - Implement tool restriction checking (whitelist/blacklist)
  - Implement time limit checking (task, phase, tool durations)
  - Implement risk threshold checking
  - Implement violation rate calculation
  - _Requirements: 5.2, 5.3, 6.1, 6.2, 6.4_

- [x] 4.1 Write property test for constraint compliance


  - **Property 11: Action Constraint Compliance Check**
  - **Validates: Requirements 5.2, 5.3**



- [x] 4.2 Write property test for violation logging

  - **Property 13: Violation Logging Completeness**
  - **Validates: Requirements 5.3, 6.3**



- [x] 4.3 Write unit tests for each constraint type
  - Test IP range matching
  - Test domain pattern matching
  - Test port range checking
  - Test tool whitelist/blacklist
  - Test time limit enforcement

  - _Requirements: 5.2, 5.3_


- [x] 5. Implement ModeController

  - Create ModeController class with mode management methods
  - Implement mode selection logic with default to Supervised
  - Implement double confirmation workflow for autonomous mode
  - Implement mode transition logic with validation
  - Integrate with ConstraintTemplateManager for template application
  - _Requirements: 1.1, 1.2, 1.4, 2.1, 2.2, 2.3, 2.4, 2.5, 10.1, 10.2, 10.3, 10.4_

- [x] 5.1 Write property test for mode selection

  - **Property 1: Mode Selection Always Available**
  - **Validates: Requirements 1.1**


- [x] 5.2 Write property test for supervised approval
  - **Property 2: Supervised Mode Requires Approval**
  - **Validates: Requirements 1.2**


- [x] 5.3 Write property test for default mode
  - **Property 3: Default Mode is Supervised**

  - **Validates: Requirements 1.4**

- [x] 5.4 Write property test for double confirmation

  - **Property 5: Double Confirmation Required**
  - **Validates: Requirements 2.2, 2.3, 2.4**


- [x] 5.5 Write property test for incomplete confirmation
  - **Property 6: Incomplete Confirmation Reverts to Supervised**
  - **Validates: Requirements 2.5**


- [x] 5.6 Write property test for template requirement

  - **Property 7: Template Selection Required for Autonomous**
  - **Validates: Requirements 3.5**

- [x] 5.7 Write property test for mode transition
  - **Property 24: Mode Switch Graceful Transition**
  - **Validates: Requirements 10.2**

- [x] 6. Implement AutonomousMonitor (audit and logging)
  - Create AutonomousMonitor class with logging methods
  - Implement action logging with constraint evaluation results

  - Implement mode transition logging
  - Implement constraint evaluation logging
  - Implement emergency stop logging

  - Implement audit log query with filtering
  - Create database indexes for efficient querying
  - _Requirements: 8.1, 8.2, 8.3, 8.4, 8.5, 10.5_


- [x] 6.1 Write property test for action logging

  - **Property 21: Comprehensive Action Logging**
  - **Validates: Requirements 8.1, 8.2**

- [x] 6.2 Write property test for mode transition logging
  - **Property 22: Mode Transition Logging**
  - **Validates: Requirements 8.3, 10.5**

- [x] 6.3 Write property test for audit query
  - **Property 23: Audit Log Query Capabilities**

  - **Validates: Requirements 8.5**

- [x] 7. Implement EmergencyStopHandler

  - Create EmergencyStopHandler class with stop methods
  - Implement stop signal propagation to active tool executions
  - Implement state preservation to Neo4j and mem0

  - Implement partial report generation
  - Implement resource cleanup
  - Integrate with AutonomousMonitor for stop event logging
  - _Requirements: 7.1, 7.2, 7.3, 7.4, 7.5_

- [x] 7.1 Write property test for emergency stop immediacy
  - **Property 18: Emergency Stop Immediacy**
  - **Validates: Requirements 7.2**


- [x] 7.2 Write property test for state preservation
  - **Property 19: State Preservation on Stop**
  - **Validates: Requirements 7.3**

- [x] 7.3 Write property test for partial report
  - **Property 20: Partial Report Generation**
  - **Validates: Requirements 7.4**

- [x] 8. Extend LangGraph agent state for autonomous mode


  - Add AutonomousAgentState fields to existing AgentState
  - Update agent state initialization to include mode and constraints
  - Modify agent state transitions to track violations and stop requests

  - _Requirements: 5.1, 6.1_

- [x] 9. Implement AutonomousExecutionEngine

  - Create AutonomousExecutionEngine class
  - Implement task execution orchestration with constraint checking
  - Integrate ConstraintEvaluator for pre-action validation
  - Implement phase execution without approval for autonomous mode
  - Implement constraint violation handling and logging
  - Implement violation rate monitoring and execution pause
  - Integrate with EmergencyStopHandler for stop requests
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 6.4, 6.5_

- [x] 9.1 Write property test for autonomous execution
  - **Property 12: Autonomous Execution Without Approval**
  - **Validates: Requirements 5.1**


- [x] 9.2 Write property test for graceful phase completion
  - **Property 14: Graceful Phase Completion**
  - **Validates: Requirements 5.4**

- [x] 9.3 Write property test for violation rate enforcement
  - **Property 17: Violation Rate Threshold Enforcement**
  - **Validates: Requirements 6.4, 6.5**


- [x] 10. Integrate autonomous execution with existing phase manager

  - Modify PhaseManager to check operation mode before requesting approval
  - Add constraint evaluation before each tool execution
  - Update phase transitions to respect autonomous mode

  - Ensure supervised mode still requires approval for exploitation/post-exploitation
  - _Requirements: 1.2, 5.1, 10.2, 10.3_



- [x] 10.1 Write property test for post-transition approval
  - **Property 25: Post-Transition Approval Requirement**
  - **Validates: Requirements 10.3**

- [x] 11. Implement LearningEngine for autonomous adaptation
  - Create LearningEngine class with learning methods
  - Implement execution pattern storage to mem0
  - Implement tool success rate tracking and retrieval
  - Implement risk assessment improvement based on outcomes
  - Implement constraint adjustment suggestions
  - Implement knowledge versioning with audit trail
  - _Requirements: 12.1, 12.2, 12.3, 12.4, 12.5_

- [x] 11.1 Write property test for learning persistence
  - **Property 26: Learning Data Persistence**
  - **Validates: Requirements 12.1**

- [x] 11.2 Write property test for success rate usage
  - **Property 27: Historical Success Rate Usage**
  - **Validates: Requirements 12.2**

- [x] 11.3 Write property test for knowledge versioning
  - **Property 28: Knowledge Versioning**
  - **Validates: Requirements 12.5**


- [x] 12. Extend Neo4j graph schema for autonomous mode
  - Create ConstraintTemplate, AutonomousExecution, ConstraintViolation node types
  - Create relationships: USES_TEMPLATE, HAS_EXECUTION, VIOLATED, EXECUTED_ACTION
  - Implement Cypher queries for autonomous execution tracking

  - Implement graph traversal for execution pattern analysis
  - _Requirements: 5.5, 8.1, 12.1_

- [x] 13. Implement report generation with constraint violations
  - Extend existing report generator to include autonomous mode section
  - Add constraint violations section with grouping and counts
  - Add near-violations section
  - Add compliance summary statistics
  - Ensure report includes all executed actions with constraint status

  - _Requirements: 5.5, 11.1, 11.2, 11.3, 11.4, 11.5_

- [x] 13.1 Write property test for report violations
  - **Property 15: Report Includes Violations**
  - **Validates: Requirements 5.5, 11.1, 11.2**

- [x] 14. Implement performance monitoring for autonomous mode
  - Create performance tracking for execution time per phase and tool
  - Implement constraint evaluation overhead measurement
  - Implement resource usage tracking (CPU, memory, network)
  - Implement performance degradation detection and logging

  - Create aggregated statistics calculation
  - _Requirements: 13.1, 13.2, 13.3, 13.4, 13.5_

- [x] 14.1 Write property test for performance tracking
  - **Property 29: Performance Tracking Granularity**
  - **Validates: Requirements 13.1**

- [x] 15. Create API endpoints for mode management

  - POST /api/tasks/{id}/mode - Select operation mode
  - POST /api/tasks/{id}/mode/confirm - Confirm autonomous mode
  - POST /api/tasks/{id}/mode/switch - Switch mode during execution
  - GET /api/tasks/{id}/mode - Get current mode
  - _Requirements: 1.1, 2.1, 10.1_


- [x] 16. Create API endpoints for constraint templates
  - GET /api/constraint-templates - List all templates
  - GET /api/constraint-templates/predefined - Get predefined templates
  - GET /api/constraint-templates/{id} - Get specific template
  - POST /api/constraint-templates - Create custom template (admin)
  - PUT /api/constraint-templates/{id} - Update template (admin)
  - POST /api/tasks/{id}/constraints/apply - Apply template to task
  - _Requirements: 3.1, 3.2, 4.1, 9.1_


- [x] 17. Create API endpoints for monitoring and control
  - POST /api/tasks/{id}/emergency-stop - Trigger emergency stop
  - GET /api/tasks/{id}/violations - Get constraint violations
  - GET /api/audit-log - Query audit log with filters
  - GET /api/tasks/{id}/metrics - Get performance metrics
  - _Requirements: 7.1, 8.5, 11.1, 13.5_


- [x] 18. Create mode selection UI component
  - Create ModeSelector component with radio buttons (Supervised/Autonomous)
  - Add mode selection to project creation form
  - Add mode selection to task creation form
  - Display mode description and implications
  - Set default to Supervised mode
  - _Requirements: 1.1, 1.4_

- [x] 19. Create double confirmation UI component
  - Create AutonomousModeConfirmation modal component
  - Display warning dialog with implications
  - Add confirmation phrase input field
  - Add explicit confirmation button
  - Implement two-step confirmation flow
  - Show revert to Supervised on incomplete confirmation
  - _Requirements: 2.1, 2.2, 2.3, 2.4, 2.5_

- [x] 20. Create constraint template selection UI
  - Create TemplateSelector component
  - Display available templates with name, description, and key constraints
  - Implement template selection and preview
  - Show template details in expandable sections
  - Integrate with mode selection flow
  - _Requirements: 3.1, 3.2, 9.1, 9.2, 9.3, 9.4, 9.5_

- [x] 21. Create autonomous mode dashboard
  - Create AutonomousDashboard page component
  - Display all active autonomous tasks in table/grid
  - Show task name, current phase, progress, compliance status, elapsed time
  - Implement warning indicators for constraint violations
  - Add filtering by project, template, status, violation status
  - Add real-time updates via WebSocket
  - _Requirements: 15.1, 15.2, 15.3, 15.4, 15.5_


- [x] 21.1 Write property test for dashboard completeness
  - **Property 30: Dashboard Completeness**
  - **Validates: Requirements 15.1, 15.2**

- [x] 22. Create emergency stop UI component
  - Add prominent emergency stop button to task detail page
  - Create emergency stop confirmation dialog
  - Implement stop request with reason input
  - Show stop progress and completion status
  - Display partial report link after stop
  - _Requirements: 7.1, 7.2, 7.4_

- [x] 23. Create mode switching UI
  - Add mode switch button to running task detail page
  - Create mode switch confirmation dialog
  - Show current mode and target mode
  - Require reason input for mode switch
  - Display transition progress
  - _Requirements: 10.1, 10.4_

- [x] 24. Create constraint violations display component

  - Create ViolationsPanel component for task detail page
  - Display violations grouped by constraint type
  - Show violation count, severity, and details
  - Display near-violations separately
  - Add timeline view of violations
  - _Requirements: 11.1, 11.2, 11.3, 11.4_


- [x] 25. Create constraint template management UI (admin)
  - Create TemplateManagement page for administrators
  - Display all templates (predefined and custom)
  - Implement template creation form with all constraint types
  - Implement template editing with validation
  - Show template usage statistics
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 4.5_


- [x] 26. Integrate autonomous mode with existing task execution flow
  - Update task start logic to check operation mode
  - Route autonomous tasks to AutonomousExecutionEngine
  - Route supervised tasks to existing approval workflow
  - Ensure seamless transition between modes
  - Update task status tracking for autonomous execution
  - _Requirements: 1.2, 5.1, 10.2_

- [x] 27. Implement WebSocket events for real-time updates
  - Add autonomous_action_executed event
  - Add constraint_violation event
  - Add mode_transition event
  - Add emergency_stop event
  - Update dashboard and task detail pages to listen for events
  - _Requirements: 13.5, 15.3, 10.1, 15.1_

- [x] 28. Add autonomous mode to existing reports

  - Extend PDF/HTML report generation with autonomous section

  - Include mode information in report header
  - Add constraint template details
  - Add violations section with statistics
  - Add compliance summary
  - _Requirements: 5.5, 11.1, 11.2, 11.3, 11.4, 11.5_

- [x] 28.1 Write property test for template application

  - **Property 8: Template Application Completeness**
  - **Validates: Requirements 3.3**

- [x] 29. Implement caching and optimization
  - Cache compiled regex patterns for domain matching
  - Implement interval trees for IP range checking
  - Add Redis caching for template lookups

  - Optimize audit log queries with proper indexes
  - Implement lazy loading for dashboard data
  - _Requirements: 6.1, 13.2, 15.1_

- [x] 30. Add Prometheus metrics for autonomous mode
  - Add autonomous_tasks_active gauge
  - Add constraint_violations_total counter

  - Add constraint_evaluation_duration histogram
  - Add mode_transitions_total counter
  - Add emergency_stops_total counter
  - _Requirements: 13.1, 13.2_

- [x] 31. Create Grafana dashboard for autonomous monitoring
  - Create dashboard with autonomous task metrics

  - Add violation rate graphs
  - Add performance metrics (evaluation time, resource usage)
  - Add mode transition tracking
  - Add alert rules for high violation rates
  - _Requirements: 6.4, 13.1, 13.2, 13.3, 13.4_

- [x] 32. Checkpoint - Ensure all tests pass

  - Run all unit tests and property-based tests
  - Verify all 30 correctness properties pass
  - Check test coverage meets requirements
  - Fix any failing tests
  - Ask user if questions arise

- [x] 33. Write integration tests for end-to-end flows

  - Test complete autonomous task execution
  - Test mode transition during execution
  - Test emergency stop at different phases
  - Test constraint violation handling
  - Test audit log completeness
  - _Requirements: 5.1, 7.1, 8.1, 10.1_


- [x] 34. Write security tests
  - Test constraint bypass attempts
  - Test unauthorized mode transitions
  - Test emergency stop authorization
  - Test audit log tampering detection
  - Test template injection attacks
  - _Requirements: 2.1, 7.1, 8.1_

- [x] 35. Write performance tests
  - Test constraint evaluation overhead (< 10ms target)
  - Test dashboard load with 100+ active tasks
  - Test audit log query with 1M+ entries
  - Test mode transition latency (< 100ms target)
  - _Requirements: 13.1, 13.2, 15.1_

- [x] 36. Create documentation
  - Write user guide for autonomous mode
  - Document constraint template creation
  - Document emergency stop procedures
  - Create API documentation for new endpoints
  - Add inline code documentation
  - _Requirements: All_


- [x] 37. Final checkpoint - Complete system verification

  - Ensure all tests pass
  - Verify all 15 requirements are implemented
  - Check all 30 correctness properties are validated
  - Perform end-to-end testing
  - Ask user if questions arise

---

## Notes

- All tasks are required for comprehensive implementation
- Each property-based test should run minimum 100 iterations
- All property tests must reference the design document property number
- Integration, security, and performance tests are required before release
- Documentation must be complete before considering the feature done