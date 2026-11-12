<!-- Original Path: C:\Users\Hossam Galal\Desktop\RED\PHASE-0-DEPENDENCY-GRAPH.md -->

# Dependency Graph - Phase 0 Analysis
**Generated:** February 27, 2026  
**Format:** JSON-compatible dependency mapping

---

## 📊 Dependency Structure

```json
{
  "applications": {
    "webapp": {
      "pages": {
        "admin": {
          "components": ["Navbar", "Sidebar", "DataTable"],
          "apis": ["GET /api/admin/users", "GET /api/admin/settings"],
          "depends_on": ["gateway", "postgres"]
        },
        "tasks": {
          "components": ["TaskCard", "ChatInterface", "GraphView"],
          "apis": ["GET /api/tasks", "POST /api/tasks"],
          "depends_on": ["gateway", "postgres", "redis"]
        },
        "graph": {
          "components": ["GraphView"],
          "apis": ["GET /api/graph"],
          "depends_on": ["gateway", "neo4j"]
        },
        "chat": {
          "components": ["ChatInterface"],
          "apis": ["WebSocket /api/chat"],
          "depends_on": ["gateway", "postgres"]
        }
      }
    },
    "pentagi": {
      "pages": {
        "flows": {
          "components": ["FlowTabs", "FlowForm"],
          "apis": ["GraphQL: getFlows", "GraphQL: createFlow"],
          "depends_on": ["backend-graphql"]
        },
        "settings": {
          "components": ["SettingsForm"],
          "apis": ["GraphQL: getSettings", "GraphQL: updateSettings"],
          "depends_on": ["backend-graphql"]
        }
      }
    },
    "deepcode": {
      "type": "REST API",
      "endpoints": {
        "GET /api/workflows": "List all workflows",
        "POST /api/workflows": "Create new workflow",
        "GET /api/analysis/:id": "Get analysis results",
        "POST /api/workflows/run": "Execute workflow"
      },
      "depends_on": ["database", "python-backend"]
    },
    "mem0": {
      "type": "REST API",
      "endpoints": {
        "GET /api/memories": "List memories",
        "POST /api/memories": "Create memory",
        "GET /api/categories": "List categories",
        "DELETE /api/memories/:id": "Delete memory"
      },
      "depends_on": ["database", "vector-store"]
    }
  }
}
```

---

## 🔗 Critical Dependencies (Blocking Order)

### Tier 1: Foundation (Must migrate first)
```
Database Layer
├─ PostgreSQL (5432)
├─ Neo4j (7687)
├─ Redis (6379)
└─ Ollama (11434)

Gateway & APIs
├─ Gateway API (8000)
├─ LangGraph Agent
├─ Authentication/Authorization
└─ GraphQL endpoints
```

### Tier 2: Core UI Components (Shared)
```
Shared Components
├─ Navbar
├─ Sidebar
├─ UI Library (shadcn/ui, Tailwind)
├─ Form components
├─ Modal/Dialog
└─ Charts/Visualizations
```

### Tier 3: Feature Pages (Can migrate in parallel)
```
webapp pages
├─ admin/* (depends on Tier 1, 2)
├─ tasks/* (depends on Tier 1, 2)
├─ projects/* (depends on Tier 1, 2)
├─ red-team/* (depends on Tier 1, 2)
└─ reports/* (depends on Tier 1, 2)

pentagi pages
├─ flows/* (depends on GraphQL, Tier 2)
└─ settings/* (depends on GraphQL, Tier 2)
```

### Tier 4: AI Services (Independent, can run in parallel)
```
DeepCode
├─ REST API (8100)
├─ Streamlit UI (8501)
└─ Python backend

Mem0
├─ API (8888)
├─ Memory management
└─ Vector store

Streamlit (Nanochat)
└─ WebSocket connections

CAI CLI
└─ Terminal interface
```

---

## 🔄 Migration Order (Optimal Path)

### Phase 0 → Phase 1 (Week 1)
```
1. Shared Components [2 days]
   ├─ Copy UI library
   ├─ Set up Tailwind config
   └─ Test basic components

2. Gateway Integration [1 day]
   ├─ Verify API endpoints
   ├─ Test authentication
   └─ Set up API clients

3. webapp → dragon-ui [3 days]
   ├─ Migrate pages (12)
   ├─ Migrate components (15)
   └─ Test all pages
```

### Phase 1 → Phase 2 (Week 2)
```
4. pentagi → dragon-ui [3 days]
   ├─ Migrate pages (9)
   ├─ Migrate components (54)
   └─ Migrate GraphQL setup

5. AI Services Integration [2 days via APIs]
   ├─ DeepCode → REST client
   ├─ Mem0 → REST client
   ├─ Streamlit → iframe
   └─ CAI → WebSocket terminal
```

---

## ⚠️ Circular Dependencies to Avoid

### Detected Issues:
```
❌ Form components reference each other
   FIX: Create base form component, extend from it

❌ ChatInterface depends on GraphView
   FIX: Extract visualization logic into separate component

❌ Multiple components reference same API
   FIX: Create centralized API client
```

---

## 🎯 API Endpoints Inventory

### Gateway API (http://localhost:8000)
```
Authentication:
  POST   /api/auth/login
  POST   /api/auth/logout
  GET    /api/auth/me

Admin:
  GET    /api/admin/users
  POST   /api/admin/users
  PUT    /api/admin/users/:id
  DELETE /api/admin/users/:id

Tasks:
  GET    /api/tasks
  POST   /api/tasks
  GET    /api/tasks/:id
  PUT    /api/tasks/:id
  DELETE /api/tasks/:id

Projects:
  GET    /api/projects
  POST   /api/projects
  GET    /api/projects/:id
  PUT    /api/projects/:id

Graph:
  GET    /api/graph
  POST   /api/graph/nodes
  GET    /api/graph/nodes/:id
  DELETE /api/graph/nodes/:id
```

### GraphQL Endpoints (from Pentagi)
```
Queries:
  getFlows
  getFlow(id)
  getSettings
  getProviders
  getTools

Mutations:
  createFlow
  updateFlow
  deleteFlow
  updateSettings
  createProvider
  deleteProvider
```

### DeepCode API (http://localhost:8100)
```
Workflows:
  GET    /api/workflows
  POST   /api/workflows
  GET    /api/workflows/:id
  POST   /api/workflows/run

Analysis:
  GET    /api/analysis/:id
  GET    /api/results/:id
```

### Mem0 API (http://localhost:8888)
```
Memories:
  GET    /api/memories
  POST   /api/memories
  GET    /api/memories/:id
  PUT    /api/memories/:id
  DELETE /api/memories/:id

Categories:
  GET    /api/categories
  POST   /api/categories
```

---

## 📦 Package Dependency Analysis

### Critical npm packages (must keep compatible):
```
"@apollo/client": "^3.x"  ← GraphQL client
"react": "^18.x"
"next": "^14.x"
"tailwindcss": "^3.x"
"typescript": "^5.x"
```

### Compatibility Matrix:
```
Feature          | webapp | pentagi | dragon-ui | Conflict?
─────────────────┼────────┼─────────┼───────────┼──────────
React version    | 18.x   | 18.x    | 18.x      | ✅ OK
Next.js version  | 14.x   | -       | 14.x      | ✅ OK
Tailwind         | 3.x    | 3.x     | 3.x       | ✅ OK
TypeScript       | 5.x    | 5.x     | 5.x       | ✅ OK
Apollo Client    | -      | 3.x     | 3.x       | ✅ OK
```

---

## 🔌 External Service Dependencies

```
Service        | Port  | Type      | Required by | Status
───────────────┼───────┼───────────┼─────────────┼─────────
PostgreSQL     | 5432  | Database  | webapp, API | ✅ Running
Neo4j          | 7687  | Graph DB  | Graph UI    | ✅ Running
Redis          | 6379  | Cache     | Sessions    | ✅ Running
Ollama         | 11434 | LLM       | AI agent    | ✅ Running
Prometheus     | 9090  | Metrics   | Monitoring  | ✅ Running
Gateway API    | 8000  | REST API  | UIs         | ✅ Running
GraphQL        | 4000  | GraphQL   | pentagi     | ✅ Running
Streamlit      | 8501  | Web app   | Nanochat    | ⏳ On-demand
DeepCode API   | 8100  | REST API  | DeepCode UI | ⏳ On-demand
Mem0 API       | 8888  | REST API  | Mem0 UI     | ⏳ On-demand
```

---

## 📈 Dependency Graph (ASCII visualization)

```
dragon-ui (main)
├─── Shared Components
│    ├─── Navbar
│    ├─── Sidebar
│    ├─── UI Library (shadcn/ui)
│    └─── Utilities
│
├─── Gateway API (8000)
│    ├─── Authentication
│    ├─── Admin endpoints
│    ├─── Task endpoints
│    ├─── Project endpoints
│    └─── Graph endpoints
│
├─── Database Layer
│    ├─── PostgreSQL (5432)
│    ├─── Neo4j (7687)
│    ├─── Redis (6379)
│    └─── Ollama (11434)
│
├─── Web Pages (from webapp)
│    ├─── admin/* → [Navbar, Sidebar, Admin APIs]
│    ├─── tasks/* → [TaskCard, ChatInterface, Task APIs]
│    ├─── projects/* → [ProjectCard, Project APIs]
│    ├─── reports/* → [ReportViewer, Report APIs]
│    └─── graph/* → [GraphView, Graph APIs]
│
├─── Web Pages (from pentagi)
│    ├─── flows/* → [FlowTabs, GraphQL]
│    └─── settings/* → [SettingsForm, GraphQL]
│
└─── AI Services (via APIs)
     ├─── DeepCode (REST 8100) ← iframe/REST client
     ├─── Mem0 (REST 8888) ← API client
     ├─── Streamlit (8501) ← iframe
     └─── CAI (WebSocket) ← Terminal component
```

---

## ✅ Success Criteria

Before Phase 1, confirm:
- [ ] All 8 applications scanned and documented
- [ ] All pages, components, and APIs listed
- [ ] Dependency graph generated
- [ ] Critical blocking dependencies identified
- [ ] Migration order determined
- [ ] Compatibility matrix reviewed
- [ ] External services verified working

---

**Status:** Phase 0 - Dependency Analysis Complete  
**Next:** Generate detailed inventory report  
**Target Date:** February 27, 2026
