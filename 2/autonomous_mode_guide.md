# Autonomous Mode Guide

This guide provides comprehensive documentation for the Dragon R1 Autonomous Execution Mode. It details user procedures, constraint template creation, emergency stop usage, and the API endpoints available for managing the autonomous engine.

## 1. User Guide for Autonomous Mode

The Autonomous Mode enables Dragon R1 to execute penetration testing phases without explicit user approval for each step, governed strictly by pre-configured constraint templates.

### 1.1 Mode Selection
By default, all tasks start in **Supervised Mode**, requiring user approval before executing potentially disruptive tools or moving between major phases (Reconnaissance → Vulnerability Scanning → Exploitation).

To enable Autonomous Mode:
1. Select the task to run autonomously.
2. Request a mode switch to Autonomous.
3. Complete the **Double Confirmation** process:
   - Provide a confirmation phrase (e.g., "I understand the risks").
   - Confirm via UI button.
4. Select and apply a **Constraint Template** to define the boundaries of the autonomous execution.
5. Finalize the mode transition.

### 1.2 Monitoring Execution
Once in Autonomous Mode, you can monitor the task via the **Autonomous Dashboard**.
- **Audit Log**: Every action executed by the autonomous engine is recorded with its constraint evaluation status.
- **Violation Tracking**: If the engine attempts an action outside the constraint template, it is blocked and recorded as a violation. A high rate of violations will automatically pause the execution.
- **Performance Metrics**: View detailed statistics around action latency and constraint evaluation overhead.

---

## 2. Constraint Template Creation

Constraint Templates are the safety mechanism that dictates what the Autonomous Engine can and cannot do. A template is composed of several limit categories.

### 2.1 Predefined Templates
We provide 5 predefined templates:
- **Conservative:** Strict limits, no exploitation allowed.
- **Balanced:** Moderate limits, standard safe exploitation.
- **Aggressive:** Loose limits, allows most tools within scope.
- **Web-Only:** Only web-related tools and ports.
- **Network-Only:** Only network-related tools and ports.

### 2.2 Template Structure
Custom templates can be created via API. A constraint template consists of:
- `name`: Unique identifier for the template.
- `description`: Explanatory text.
- `scope_limits`: Whitelisted IP ranges, domains, ports, and protocols.
- `tool_restrictions`: Whitelisted and blacklisted tool names.
- `time_limits`: Maximum duration per task, phase, and individual tool execution.
- `risk_thresholds`: Maximum allowed risk score (1-10) and whether exploitation is permitted.

### 2.3 Creating Custom Templates
Administrators can submit a POST request to `/api/constraint-templates` with a JSON body matching the structure above. Custom templates will then be available for selection during the double confirmation process.

---

## 3. Emergency Stop Procedures

The Emergency Stop is a critical safety feature that instantly halts all autonomous execution and preserves the current state.

### 3.1 Triggering an Emergency Stop
If the autonomous engine exhibits unexpected behavior or if execution must be halted immediately:
1. Trigger the Emergency Stop via the Dashboard button or the API.
2. Provide a reason for the stop (e.g., "Unexpected behavior", "Out of scope action").

### 3.2 System Response
When an Emergency Stop is triggered:
- The system immediately interrupts all active tools running within the autonomous phases.
- The current graph state is preserved in Neo4j and mem0 for future forensic analysis and learning.
- A partial report is generated with findings collected up to the moment of the stop.
- The task is locked and requires manual intervention to resume.

---

## 4. API Endpoints Reference

The following endpoints are available for autonomous mode management. All endpoints return standard JSON responses or JSON-formatted HTTP exception details on failure.

### Mode Management
- **POST `/api/tasks/{task_id}/mode`**
  - **Payload:** `{"mode": "autonomous|supervised", "user_id": "string", "reason": "string"}`
  - **Description:** Initiates a mode change for the task.

- **POST `/api/tasks/{task_id}/mode/confirm`**
  - **Payload:** `{"phrase": "string", "user_id": "string"}`
  - **Description:** Step 1 of the double confirmation (phrase validation).

- **POST `/api/tasks/{task_id}/mode/confirm-button`**
  - **Payload:** `{"user_id": "string"}`
  - **Description:** Step 2 of the double confirmation (button click).

- **POST `/api/tasks/{task_id}/mode/finalize`**
  - **Payload:** `{"user_id": "string"}`
  - **Description:** Finalizes the transition after double confirmation and template assignment.

- **POST `/api/tasks/{task_id}/mode/switch`**
  - **Payload:** `{"target_mode": "string", "user_id": "string", "reason": "string"}`
  - **Description:** Immediately switches mode during ongoing execution.

- **GET `/api/tasks/{task_id}/mode`**
  - **Description:** Retrieves the current active mode.

### Constraint Templates
- **GET `/api/constraint-templates`**
  - **Description:** Lists all available templates.

- **GET `/api/constraint-templates/predefined`**
  - **Description:** Lists predefined system templates.

- **GET `/api/constraint-templates/{template_id}`**
  - **Description:** Retrieves details for a specific template.

- **POST `/api/constraint-templates`**
  - **Payload:** Constraint template JSON structure.
  - **Description:** Creates a custom constraint template (Admin).

- **POST `/api/tasks/{task_id}/constraints/apply`**
  - **Payload:** `{"template_id": "string"}`
  - **Description:** Applies a specific template to the given task.

### Monitoring & Control
- **POST `/api/tasks/{task_id}/emergency-stop`**
  - **Payload:** `{"user_id": "string", "reason": "string"}`
  - **Description:** Triggers the immediate emergency stop sequence.

- **GET `/api/tasks/{task_id}/violations`**
  - **Description:** Fetch constraint violations logged for the task.

- **GET `/api/audit-log`**
  - **Query Params:** Filters (task_id, action_type, status, limit, offset, etc.)
  - **Description:** Queries the comprehensive autonomous audit log.

- **GET `/api/tasks/{task_id}/metrics`**
  - **Description:** Retrieves real-time performance and evaluation metrics for a running task.

- **GET `/api/autonomous/dashboard`**
  - **Description:** Retrieves aggregated status summary of all active autonomous tasks.
