# 🔥 Dragon UI - Critical Fixes Plan
**Created**: 2026-04-11 21:15
**Status**: EXECUTION READY

---

## 📊 Problem Analysis Summary

### 🚨 6 Critical Blockers

1. **WebSocket 404 Error**
   - **Issue**: `/ws` endpoint not configured in Next.js
   - **Impact**: Real-time updates fail, fallback to HTTP only
   - **Root Cause**: Missing proxy configuration in `next.config.mjs`

2. **Build EPERM Errors**
   - **Issue**: File permission errors during build
   - **Impact**: Build failures, deployment blocked
   - **Root Cause**: Windows file locking, turbopack issues

3. **Hero Page vs Runtime Chat Conflict**
   - **Issue**: `/chat` page has marketing hero + runtime chat mixed
   - **Impact**: Components use mock data instead of real data
   - **Root Cause**: Architectural confusion between landing page and app

4. **Ollama 503 Errors**
   - **Issue**: Ollama service unavailable or not started
   - **Impact**: No LLM responses, chat broken
   - **Root Cause**: Ollama not running or wrong port

5. **Dual Upload System**
   - **Issue**: Two different file upload implementations
   - **Impact**: Confusion, inconsistent behavior
   - **Root Cause**: Legacy code + new implementation

6. **Code Overflow Issues**
   - **Issue**: Long code blocks break layout
   - **Impact**: UI breaks, unreadable content
   - **Root Cause**: Missing overflow handling in CodeBlock component

### 🔧 12 Partially Implemented Features

1. WorkflowGraph - Created but not connected to real data
2. ReasoningPanel - Created but not connected to real data
3. StatusBar - Not created yet
4. AttackPipeline - Not created yet
5. Zustand store slices - Not created yet
6. WebSocket subscriptions - Partially implemented
7. Gateway API integration - Partially implemented
8. DeepThink component - Uses mock workflow data
9. ChatBubble - Not connected to real message metadata
10. TopicNavigator - Uses fallback data only
11. OllamaSettings - Not connected to real stats
12. LiveTaskList - Not connected to real tasks

---

## 🎯 Execution Strategy

### Phase 1: Infrastructure Fixes (CRITICAL)
**Priority**: P0 - Must fix first
**Time**: 30 minutes

1. **Fix WebSocket Proxy**
   - Add `/api/ws` proxy to `next.config.mjs`
   - Configure rewrites for WebSocket upgrade
   - Test WebSocket connection

2. **Fix Build Issues**
   - Disable turbopack (causing EPERM)
   - Add proper file handling
   - Configure standalone output correctly

3. **Fix Ollama Integration**
   - Add Ollama health check
   - Add auto-start script
   - Add fallback error handling

### Phase 2: Architecture Cleanup (HIGH)
**Priority**: P1 - Critical for functionality
**Time**: 45 minutes

1. **Separate Hero from Chat**
   - Move hero content to `/` (home page)
   - Make `/chat` pure runtime interface
   - Remove marketing content from chat page

2. **Fix File Upload**
   - Consolidate to single upload system
   - Use existing API integration
   - Remove duplicate code

3. **Fix Code Overflow**
   - Add proper overflow handling
   - Add horizontal scroll
   - Add expand/collapse for long blocks

### Phase 3: Data Integration (MEDIUM)
**Priority**: P2 - Complete functionality
**Time**: 60 minutes

1. **Connect WorkflowGraph to Real Data**
   - Use WebSocket task_updates
   - Update graph nodes in real-time
   - Show actual workflow status

2. **Connect ReasoningPanel to Real Data**
   - Use WebSocket agent_thought events
   - Show actual reasoning steps
   - Display real timing data

3. **Create Missing Components**
   - StatusBar with real progress
   - AttackPipeline with real phases
   - Connect to Zustand store

4. **Implement Zustand Store Slices**
   - sidebarSlice
   - chatSlice
   - deepThinkSlice
   - inputSlice

5. **Complete WebSocket Integration**
   - Subscribe to all channels
   - Handle all message types
   - Update UI in real-time

6. **Complete Gateway API Integration**
   - Load initial data
   - Handle errors gracefully
   - Show loading states

### Phase 4: Polish & Testing (LOW)
**Priority**: P3 - Nice to have
**Time**: 30 minutes

1. **Test All Features**
   - WebSocket real-time updates
   - File uploads
   - Model switching
   - Session management

2. **Fix UI Issues**
   - Animations
   - Responsive design
   - RTL support

3. **Add Error Handling**
   - Graceful degradation
   - User-friendly messages
   - Retry mechanisms

---

## 🚀 Implementation Plan

### Step 1: Fix next.config.mjs (WebSocket + Build)
```javascript
/** @type {import('next').NextConfig} */
const nextConfig = {
  typescript: {
    ignoreBuildErrors: false,
  },
  // REMOVE turbopack - causes EPERM errors
  // turbopack: { root: __dirname },
  
  // Add WebSocket proxy
  async rewrites() {
    return [
      {
        source: '/api/ws/:path*',
        destination: 'http://localhost:8000/ws/:path*',
      },
    ];
  },
  
  // Configure for production
  output: 'standalone',
  productionBrowserSourceMaps: false,
  
  // Optimize images
  images: {
    remotePatterns: [
      { protocol: 'http', hostname: 'localhost' },
      { protocol: 'http', hostname: '127.0.0.1' },
    ],
    formats: ['image/webp', 'image/avif'],
  },
};
```

### Step 2: Fix WebSocket Connection
```typescript
// Update websocket.ts to use correct endpoint
const socketUrl = `ws://localhost:8000/ws/${this.client_id}`;
// OR use proxy:
const socketUrl = `/api/ws/${this.client_id}`;
```

### Step 3: Add Ollama Health Check
```typescript
// Add to api.ts
async checkOllamaHealth(): Promise<boolean> {
  try {
    const response = await fetch('http://localhost:11434/api/tags');
    return response.ok;
  } catch {
    return false;
  }
}
```

### Step 4: Create Missing Components
- StatusBar.tsx
- AttackPipeline.tsx
- Store slices (4 files)

### Step 5: Connect Components to Real Data
- Update WorkflowGraph
- Update ReasoningPanel
- Update DeepThink
- Update all other components

---

## 📝 Files to Modify

### Critical Files (Phase 1)
1. `Cerberus/dragon-ui/next.config.mjs` - Add WebSocket proxy, remove turbopack
2. `Cerberus/dragon-ui/src/lib/websocket.ts` - Fix endpoint URL
3. `Cerberus/dragon-ui/src/lib/api.ts` - Add Ollama health check

### Architecture Files (Phase 2)
4. `Cerberus/dragon-ui/src/app/chat/page.tsx` - Remove hero content
5. `Cerberus/dragon-ui/src/components/ChatInterface.tsx` - Fix file upload
6. `Cerberus/dragon-ui/src/components/Content/CodeBlock.tsx` - Fix overflow

### New Files (Phase 3)
7. `Cerberus/dragon-ui/src/components/Status/StatusBar.tsx` - CREATE
8. `Cerberus/dragon-ui/src/components/Pipeline/AttackPipeline.tsx` - CREATE
9. `Cerberus/dragon-ui/src/lib/store/slices/sidebarSlice.ts` - CREATE
10. `Cerberus/dragon-ui/src/lib/store/slices/chatSlice.ts` - CREATE
11. `Cerberus/dragon-ui/src/lib/store/slices/deepThinkSlice.ts` - CREATE
12. `Cerberus/dragon-ui/src/lib/store/slices/inputSlice.ts` - CREATE

### Integration Files (Phase 3)
13. `Cerberus/dragon-ui/src/components/Workflow/WorkflowGraph.tsx` - Connect to real data
14. `Cerberus/dragon-ui/src/components/Reasoning/ReasoningPanel.tsx` - Connect to real data
15. `Cerberus/dragon-ui/src/components/Chat/DeepThink.tsx` - Connect to real data

---

## ✅ Success Criteria

### Phase 1 Complete When:
- [ ] WebSocket connects successfully (no 404)
- [ ] Build completes without EPERM errors
- [ ] Ollama health check works

### Phase 2 Complete When:
- [ ] `/chat` page is pure runtime (no hero)
- [ ] File upload works consistently
- [ ] Code blocks don't overflow

### Phase 3 Complete When:
- [ ] WorkflowGraph shows real workflow data
- [ ] ReasoningPanel shows real reasoning steps
- [ ] StatusBar shows real progress
- [ ] AttackPipeline shows real phases
- [ ] All components connected to Zustand store
- [ ] WebSocket updates all components in real-time
- [ ] Gateway API loads initial data

### Phase 4 Complete When:
- [ ] All features tested and working
- [ ] UI polished and responsive
- [ ] Error handling graceful
- [ ] No console errors

---

## 🎯 Execution Order

1. **IMMEDIATE** (Phase 1): Fix infrastructure - WebSocket, Build, Ollama
2. **NEXT** (Phase 2): Clean architecture - Hero/Chat separation, File upload, Code overflow
3. **THEN** (Phase 3): Connect data - All components to real data sources
4. **FINALLY** (Phase 4): Polish and test

---

## 📊 Progress Tracking

- [ ] Phase 1: Infrastructure Fixes (0/3)
- [ ] Phase 2: Architecture Cleanup (0/3)
- [ ] Phase 3: Data Integration (0/6)
- [ ] Phase 4: Polish & Testing (0/3)

**Total**: 0/15 tasks complete

---

**Next Action**: Start Phase 1 - Fix next.config.mjs
