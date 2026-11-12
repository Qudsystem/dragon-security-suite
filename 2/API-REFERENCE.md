# 📋 Dragon-R1 Chat - API Reference

## Frontend Routes (Next.js)

### Chat API
```
GET  /api/chat/history
  - Fetch chat history
  - Query params: task_id, limit
  - Response: { messages: [...], task_id: string }

POST /api/chat/send
  - Send message to agent
  - Body: { message: string, context?: object, task_id?: string }
  - Response: { message: string, task_id: string, suggestions: [...] }

POST /api/chat/upload
  - Upload file for analysis
  - Body: FormData { file, type }
  - Response: { success: boolean, filename: string, analysis: object }

POST /api/chat/process-file
  - Process uploaded file
  - Body: { filename: string, analysis_type: string, task_id?: string }
  - Response: { success: boolean, analysis: object, processing_time: number }
```

---

## Backend Routes (Gateway @ localhost:8000)

### Health & Status
```
GET /api/v1/health
  - Health check
  - Response: { status: "healthy", timestamp: ISO8601, version: string }
```

### Tasks
```
GET  /api/v1/tasks
  - List all tasks
  - Query: skip, limit, status
  - Response: { tasks: [...], total: number }

POST /api/v1/tasks
  - Create new task
  - Body: { target, target_type, objective, description }
  - Response: { task_id, status, created_at }

GET  /api/v1/tasks/{task_id}
  - Get task details with agent messages
  - Response: { 
      task_id, 
      status, 
      progress_percent, 
      agent_messages: [...],
      current_phase, 
      findings: [...] 
    }

PATCH /api/v1/tasks/{task_id}
  - Update task
  - Body: { ...task fields to update }
  - Response: { updated_task }

DELETE /api/v1/tasks/{task_id}
  - Delete task
  - Response: { success: boolean }
```

### Chat
```
POST /api/v1/chat/message
  - Send chat message to agent
  - Body: { message, context, session_id }
  - Response: { message, task_id, suggestions }

GET  /api/v1/chat/history
  - Get chat history
  - Query: limit, task_id, session_id
  - Response: { agent_messages: [...], task_id }
```

### Reports
```
GET  /api/v1/tasks/{task_id}/report
  - Get report for task
  - Query: format (json, pdf, html)
  - Response: File or JSON

GET  /api/v1/reports
  - List all reports
  - Query: skip, limit, task_id, status
  - Response: { reports: [...], total: number }
```

### WebSocket
```
WS   /ws/{client_id}
  - Real-time bidirectional communication
  - Message Types:
    {
      type: "agent_message" | "agent_typing" | "task_progress" | "error" | "notification",
      data: {...},
      timestamp: ISO8601
    }
  
  Subscribe to channels:
    - "notifications"
    - "approvals"
    - "tasks"
    - "agent_chat"
    - "security_update"
    - "security_alert"
```

---

## Message Types (WebSocket)

### Agent Message
```json
{
  "type": "agent_message",
  "id": "msg_123",
  "role": "agent",
  "content": "Scanning target...",
  "timestamp": "2025-03-10T10:30:00Z",
  "metadata": {}
}
```

### Agent Typing Indicator
```json
{
  "type": "agent_typing",
  "typing": true
}
```

### Task Progress
```json
{
  "type": "task_progress",
  "task_id": "task_123",
  "status": "running",
  "progress_percent": 45,
  "current_phase": "exploitation",
  "message": "Found 3 open ports"
}
```

### Task Completed
```json
{
  "type": "task_completed",
  "task_id": "task_123",
  "report_id": "report_123",
  "findings_count": 5,
  "severity_breakdown": {
    "critical": 1,
    "high": 2,
    "medium": 2
  }
}
```

### Error
```json
{
  "type": "error",
  "message": "Error message here",
  "error_code": "ERROR_CODE",
  "task_id": "task_123"
}
```

### Notification
```json
{
  "type": "notification",
  "id": "notif_123",
  "message": "Notification message",
  "level": "info" | "warning" | "error" | "success",
  "timestamp": "2025-03-10T10:30:00Z"
}
```

---

## Authentication

### Header
```
Authorization: Bearer <JWT_TOKEN>
```

### Login
```
POST /api/v1/auth/login
Body: { username, password }
Response: { access_token, refresh_token, expires_in }
```

### Refresh Token
```
POST /api/v1/auth/refresh
Header: { Authorization: Bearer <refresh_token> }
Response: { access_token, expires_in }
```

---

## Ports & Services

| Service | Port | URL | Status |
|---------|------|-----|--------|
| Frontend (Next.js) | 3000 | http://localhost:3000 | Running? |
| Backend Gateway | 8000 | http://localhost:8000 | Running? |
| WebSocket | 8000 | ws://localhost:8000/ws/{client_id} | Running? |
| Streamlit (optional) | 8501 | http://localhost:8501 | Optional |
| Docker (optional) | varies | - | Optional |

---

## Status Codes

| Code | Meaning | Action |
|------|---------|--------|
| 200 | OK | Success ✅ |
| 201 | Created | Resource created ✅ |
| 400 | Bad Request | Check request body |
| 401 | Unauthorized | Check authentication token |
| 403 | Forbidden | Check permissions/role |
| 404 | Not Found | Check resource exists |
| 500 | Server Error | Check backend logs |

---

## Error Response Format

```json
{
  "error": "Error message",
  "detail": "Detailed error description",
  "status_code": 400,
  "timestamp": "2025-03-10T10:30:00Z"
}
```

---

## Example: Complete Chat Flow

### 1. Create Task
```bash
curl -X POST http://localhost:8000/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "target": "example.com",
    "target_type": "domain",
    "objective": "Security assessment"
  }'

# Response:
# { "task_id": "task_123", "status": "pending" }
```

### 2. Connect WebSocket
```javascript
const ws = new WebSocket('ws://localhost:8000/ws/ui-123');
ws.onmessage = (e) => {
  const msg = JSON.parse(e.data);
  console.log('Agent:', msg.content);
};
```

### 3. Monitor Progress
```bash
# In another terminal:
curl http://localhost:8000/api/v1/tasks/task_123

# Response includes:
# {
#   "status": "running",
#   "progress_percent": 45,
#   "agent_messages": [
#     { "type": "agent_message", "content": "Scanning..." }
#   ]
# }
```

### 4. Get Report
```bash
curl "http://localhost:8000/api/v1/tasks/task_123/report?format=json"

# Response: Complete report JSON
```

---

## Testing Commands

### Health Check
```bash
curl http://localhost:8000/api/v1/health
```

### List Tasks
```bash
curl http://localhost:8000/api/v1/tasks?limit=10
```

### Create Task (via Frontend)
```bash
curl -X POST http://localhost:3000/api/chat/send \
  -H "Content-Type: application/json" \
  -d '{ "message": "scan example.com" }'
```

### WebSocket Test (JavaScript)
```javascript
const ws = new WebSocket('ws://localhost:8000/ws/test-client');
ws.onopen = () => console.log('✅ Connected');
ws.onmessage = (e) => console.log('📨', e.data);
ws.onerror = () => console.log('❌ Error');
ws.onclose = () => console.log('🔌 Closed');
```

---

**Last Updated:** 2025-03-10  
**Version:** 1.0.0
