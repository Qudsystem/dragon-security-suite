# Dragon-R1 Product Requirements Document

## Overview
Dragon-R1 is an autonomous offensive security platform providing penetration testing as a service. It provides AI-driven attack pipeline execution, real-time collaboration, and comprehensive reporting.

## Core Features

### 1. Authentication & Authorization
- JWT-based authentication
- RBAC with roles: admin, analyst, viewer
- Secure login/logout endpoints
- Token refresh mechanism

### 2. Attack Pipeline
- Phase-based execution: RECON → ENUMERATE → DISCOVER → EXPLOIT → ESCALATE → EXFILTRATE → REPORT
- Real-time progress tracking via WebSocket
- Human-in-the-loop approval workflow
- Parallel phase execution support

### 3. AI Agent System
- LangGraph-based ReAct autonomous agent (Think → Act → Observe)
- Multi-agent coordination
- LLM provider support (Ollama, OpenAI, Anthropic)
- AI safety guardrails with prompt injection detection

### 4. Real-time Communication
- WebSocket channels: task_updates, agent_chat, approvals
- Live progress streaming
- Real-time finding notifications

### 5. Memory & Knowledge
- mem0 memory management
- RAG pipeline for context retrieval
- Attack graph storage in Neo4j

### 6. Frontend Application
- Chat interface for AI interaction
- Dashboard with attack status
- Task management
- Graph visualization of attack paths
- Report viewer
- Settings management
- Project management

### 7. Reporting
- Automated report generation
- Multiple severity levels (critical/high/medium/low/info)
- Finding management and tracking

## API Endpoints (Backend - port 8000)
- POST /api/v1/auth/login
- POST /api/v1/auth/logout
- GET /api/v1/health
- GET /api/v1/chat/message
- POST /api/v1/tasks/create
- GET /api/v1/tasks/{task_id}
- GET /api/v1/findings
- GET /api/v1/reports
- GET /api/v1/settings
- PUT /api/v1/settings

## Frontend Pages (port 3000)
- /chat - Main chat interface
- /dashboard - Attack dashboard
- /tasks - Task management
- /reports - Report viewer
- /graph - Attack graph
- /admin - Admin panel
- /settings - Settings page
- /projects - Project management
