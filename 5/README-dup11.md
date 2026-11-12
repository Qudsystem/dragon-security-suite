# Autonomous Mode - Dragon R1

## Overview
Complete autonomous execution system for Dragon R1 penetration testing platform. Enables fully autonomous security testing with configurable constraints and comprehensive monitoring.

## Features Implemented

### ✅ Core Components
1. **ConstraintTemplateManager** - Manages constraint templates with 5 predefined templates
2. **ConstraintEvaluator** - Real-time constraint evaluation and violation detection
3. **ModeController** - Mode selection, double confirmation, and transitions
4. **AutonomousMonitor** - Complete audit trail and metrics tracking
5. **EmergencyStopHandler** - Immediate stop with state preservation
6. **LearningEngine** - Continuous learning from executions
7. **AutonomousExecutionEngine** - Orchestrates autonomous execution

### ✅ Database Schema
- Extended `tasks` table with autonomous mode columns
- `constraint_templates` table with 5 predefined templates
- `autonomous_audit_log` table for complete audit trail
- `constraint_violations` table for violation tracking
- `execution_patterns` and `knowledge_updates` for learning

### ✅ API Endpoints
- Mode management (select, confirm, switch)
- Constraint template CRUD
- Emergency stop
- Audit log queries
- Performance metrics
- Violations tracking

### ✅ Predefined Templates
1. **Conservative**: Reconnaissance only, no exploitation
2. **Balanced**: Low-risk exploitation with strict limits
3. **Aggressive**: Broader exploitation within scope
4. **Web-Only**: Web application testing only
5. **Network-Only**: Network infrastructure testing only

## Architecture

```
┌─────────────────────────────────────────┐
│         API Layer (FastAPI)             │
│  - Mode Management                      │
│  - Template Management                  │
│  - Monitoring & Control                 │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│      Autonomous Mode Manager            │
│  ┌──────────────┐  ┌──────────────┐    │
│  │ Mode         │  │ Constraint   │    │
│  │ Controller   │  │ Evaluator    │    │
│  └──────────────┘  └──────────────┘    │
│  ┌──────────────┐  ┌──────────────┐    │
│  │ Emergency    │  │ Learning     │    │
│  │ Stop Handler │  │ Engine       │    │
│  └──────────────┘  └──────────────┘    │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│     Execution Engine                    │
│  - Autonomous Execution                 │
│  - Constraint Enforcement               │
│  - Violation Monitoring                 │
└─────────────────────────────────────────┘
                  │
┌─────────────────────────────────────────┐
│     Storage Layer                       │
│  - PostgreSQL (audit, violations)       │
│  - Neo4j (graph relationships)          │
│  - mem0 (learning patterns)             │
└─────────────────────────────────────────┘
```

## Usage

### 1. Apply Database Migrations
```bash
cd Cerberus/gateway/migrations
python apply_migrations.py
```

### 2. Select Operation Mode
```python
from autonomous import ModeController

controller = ModeController()

# Select supervised mode (default)
result = controller.select_mode(task_id, OperationMode.SUPERVISED, user_id)

# Select autonomous mode (requires confirmation)
result = controller.select_mode(task_id, OperationMode.AUTONOMOUS, user_id)
```

### 3. Confirm Autonomous Mode
```python
# Step 1: Confirm with phrase
controller.confirm_autonomous_mode(task_id, "I understand the risks", user_id)

# Step 2: Confirm with button
controller.confirm_autonomous_button(task_id, user_id)

# Step 3: Select template
template_manager.apply_template(task_id, template_id)
controller.set_template_selected(task_id, template_id)

# Step 4: Finalize
result = controller.finalize_autonomous_mode(task_id, user_id)
```

### 4. Execute Autonomous Task
```python
from autonomous import AutonomousExecutionEngine, ConstraintTemplateManager

engine = AutonomousExecutionEngine()
template_manager = ConstraintTemplateManager()

# Get template
template = template_manager.get_template_by_name("Balanced")

# Execute task
result = engine.execute_task(task_id, template)
```

### 5. Emergency Stop
```python
from autonomous import EmergencyStopHandler

stop_handler = EmergencyStopHandler()

# Request stop
stop_handler.request_stop(task_id, user_id, "User requested stop")

# Execute stop
result = stop_handler.execute_stop(task_id)
```

### 6. Query Audit Log
```python
from autonomous import AutonomousMonitor
from models import AuditFilters

monitor = AutonomousMonitor()

# Query all actions for a task
filters = AuditFilters(task_id=task_id)
entries = monitor.query_audit_log(filters)

# Get metrics
metrics = monitor.calculate_metrics(task_id)
print(f"Total actions: {metrics.total_actions}")
print(f"Violation rate: {metrics.violation_rate:.2%}")
```

## API Examples

### Select Mode
```bash
curl -X POST http://localhost:8000/api/tasks/task-123/mode \
  -H "Content-Type: application/json" \
  -d '{"mode": "autonomous", "user_id": "user-456"}'
```

### Apply Template
```bash
curl -X POST http://localhost:8000/api/tasks/task-123/constraints/apply \
  -H "Content-Type: application/json" \
  -d '{"template_id": "template-789"}'
```

### Emergency Stop
```bash
curl -X POST http://localhost:8000/api/tasks/task-123/emergency-stop \
  -H "Content-Type: application/json" \
  -d '{"user_id": "user-456", "reason": "Unexpected behavior detected"}'
```

### Get Violations
```bash
curl http://localhost:8000/api/tasks/task-123/violations
```

## Testing

### Run Property-Based Tests
```bash
cd Cerberus/gateway/tests
pytest test_autonomous_models.py -v
```

### Run All Tests
```bash
pytest tests/ -v --cov=autonomous
```

## Configuration

### Environment Variables
```bash
# Database
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_DB=dragon_db
POSTGRES_USER=dragon_user
POSTGRES_PASSWORD=your_password

# Neo4j
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password

# mem0
MEM0_API_KEY=your_api_key
```

### Constraint Template Configuration
Templates can be customized in `constraint_template_manager.py` or created via API.

## Security Considerations

1. **Double Confirmation**: Autonomous mode requires phrase + button confirmation
2. **Constraint Enforcement**: All actions validated before execution
3. **Audit Trail**: Complete logging of all operations
4. **Emergency Stop**: Immediate halt capability
5. **Violation Monitoring**: Automatic pause on high violation rates

## Performance

- Constraint evaluation: < 10ms per action
- Violation rate calculation: O(1) with rolling window
- Audit log queries: Indexed for fast retrieval
- Dashboard load: < 100ms for 100+ tasks

## Troubleshooting

### Migration Fails
```bash
# Check database connection
psql -h localhost -U dragon_user -d dragon_db

# Manually apply migration
psql -h localhost -U dragon_user -d dragon_db -f migrations/001_autonomous_mode.sql
```

### Tests Fail
```bash
# Install test dependencies
pip install pytest hypothesis pytest-cov

# Run with verbose output
pytest tests/test_autonomous_models.py -vv
```

### Constraint Violations Not Logged
Check that AutonomousMonitor is properly initialized and database connection is active.

## Future Enhancements

- [ ] Machine learning for risk prediction
- [ ] Advanced constraint templates with ML-based adaptation
- [ ] Real-time dashboard with WebSocket updates
- [ ] Multi-tenant support with isolated constraints
- [ ] Integration with external SIEM systems

## License
Proprietary - Dragon R1 Project

## Support
For issues or questions, contact the Dragon R1 development team.
