# Design Document: Graph Loading Failure Fix

## Overview

This design addresses the issue where the graph visualization page displays a generic error message when the backend gateway is offline, without providing clear guidance to users about the root cause or remediation steps.

## Bug Condition Specification

### C(X): Bug Condition - Identifies Buggy Inputs

The bug occurs when:

```pseudocode
isBugCondition(request) = 
  (request.target = "/graph" page) AND
  (backend_gateway.status = "offline" OR backend_gateway.unreachable = true)
```

**Concrete failing cases:**
- User navigates to `/graph` page when gateway is not running
- API call to `/api/v1/graph` fails with network error (ECONNREFUSED, timeout)
- WebSocket connection to `ws://localhost:8000/ws/` fails to establish
- Health check endpoint `/api/v1/health` is unreachable

**Current defective behavior:**
- Displays generic error: "Failed to load the attack graph right now"
- Does not distinguish between backend offline vs. Neo4j service issues
- No actionable guidance for users to resolve the issue
- Multiple retry attempts logged without clear user feedback

## Expected Behavior Properties

### P(result): Expected Behavior for Bug Condition

When the bug condition is satisfied (backend is offline), the system should exhibit:

```pseudocode
expectedBehavior(result) =
  (result.error_message = "Backend gateway is offline. Please start the Dragon-R1 gateway service.") AND
  (result.includes_instructions = true) AND
  (result.distinguishes_backend_vs_neo4j = true) AND
  (result.connection_status_visible = true)
```

**Specific expected behaviors:**

1. **Clear error messaging** (from requirement 2.1):
   - Display: "Backend gateway is offline. Please start the Dragon-R1 gateway service."
   - Include instructions: "Run `START-DRAGON-R1.bat` (Windows) or `./start-dragon-r1.sh` (Linux/Mac)"

2. **Network error detection** (from requirement 2.2):
   - Detect network errors (ECONNREFUSED, ETIMEDOUT, fetch failures)
   - Distinguish from other error types (4xx, 5xx responses)
   - Display user-friendly message indicating backend connectivity issues

3. **WebSocket status reflection** (from requirement 2.3):
   - Display connection status prominently on graph page
   - Update error message when WebSocket fails: "Real-time updates unavailable - backend disconnected"
   - Show reconnection attempts in UI

4. **Health check interpretation** (from requirement 2.4):
   - When `/api/v1/health` unreachable → "Backend offline"
   - When health check succeeds but Neo4j degraded → "Graph database is offline"
   - Clear distinction between backend connectivity and service health

5. **Port-agnostic API resolution** (from requirement 2.5):
   - API client resolves backend URL from `window.location`
   - Works correctly on non-default ports (e.g., 3001)
   - Maintains proper routing regardless of frontend port

## Preservation Requirements

### ¬C(X): Non-Buggy Inputs - Behavior to Preserve

The fix must NOT change behavior when:

```pseudocode
isNonBugCondition(request) =
  (backend_gateway.status = "online" AND backend_gateway.reachable = true)
```

**Scenarios that must remain unchanged:**

1. **Healthy backend + healthy Neo4j** (requirement 3.1):
   - MUST continue to load and display attack graph visualization
   - Nodes and edges render correctly
   - All graph interactions work as before

2. **Healthy backend + degraded Neo4j** (requirement 3.2):
   - MUST continue to display: "Graph database is offline"
   - Show empty graph state with retry option
   - Distinguish this from backend offline scenario

3. **Healthy backend + empty graph** (requirement 3.3):
   - MUST continue to display: "No graph data has been indexed yet. Start or resume a task to populate the attack graph."
   - Empty state UI remains unchanged

4. **Retry functionality** (requirement 3.4):
   - MUST continue to re-fetch graph data on retry button click
   - Re-check health status
   - Attempt to reconnect WebSocket

5. **WebSocket real-time updates** (requirement 3.5):
   - MUST continue to subscribe to channels: `graph_update`, `node_discovery`, `vulnerability_alert`
   - Real-time updates work when backend is online

6. **Graph interactions** (requirement 3.6):
   - MUST continue to display node details in sidebar
   - All existing visualization features work unchanged
   - Node selection, zoom, pan, layout controls preserved

## Implementation Strategy

### Files to Modify

1. **Frontend API Client** (`Cerberus/dragon-ui/src/lib/api.ts`):
   - Add network error detection logic
   - Distinguish between connection failures and HTTP errors
   - Implement port-agnostic base URL resolution

2. **Graph Page Component** (`Cerberus/dragon-ui/src/app/graph/page.tsx`):
   - Update error messaging based on error type
   - Display connection status from WebSocket
   - Add backend startup instructions

3. **WebSocket Manager** (`Cerberus/dragon-ui/src/lib/websocket.ts`):
   - Expose connection state to components
   - Provide clear disconnection reasons

4. **Zustand Store** (`Cerberus/dragon-ui/src/lib/store.ts`):
   - Add backend connectivity state
   - Track error types (network vs. service)

### Error Classification

```typescript
enum ErrorType {
  BACKEND_OFFLINE = "backend_offline",      // Network error, connection refused
  NEO4J_DEGRADED = "neo4j_degraded",        // Backend online, Neo4j unhealthy
  EMPTY_GRAPH = "empty_graph",              // Backend online, no data
  UNKNOWN = "unknown"                        // Other errors
}
```

### Detection Logic

```pseudocode
function classifyError(error, healthStatus):
  if error.type in [ECONNREFUSED, ETIMEDOUT, NetworkError]:
    return BACKEND_OFFLINE
  
  if healthStatus.reachable and not healthStatus.neo4j_healthy:
    return NEO4J_DEGRADED
  
  if healthStatus.reachable and graphData.nodes.length == 0:
    return EMPTY_GRAPH
  
  return UNKNOWN
```

## Testing Strategy

### Bug Condition Exploration Test (Property 1)

**Test before fix (should FAIL):**
```typescript
// Property-based test scoped to concrete failing case
test("Bug Condition: Backend offline shows generic error", async () => {
  // Arrange: Backend is offline (mock network failure)
  mockBackendOffline();
  
  // Act: Navigate to /graph page
  render(<GraphPage />);
  await waitFor(() => screen.getByText(/failed to load/i));
  
  // Assert: Should show specific backend offline message (will FAIL on unfixed code)
  expect(screen.getByText(/backend gateway is offline/i)).toBeInTheDocument();
  expect(screen.getByText(/start the dragon-r1 gateway service/i)).toBeInTheDocument();
});
```

**Expected outcome on unfixed code:** Test FAILS - confirms bug exists (generic error shown instead)

### Preservation Tests (Property 2)

**Test before fix (should PASS):**
```typescript
// Observe behavior on unfixed code for non-buggy inputs
test("Preservation: Healthy backend loads graph correctly", async () => {
  // Arrange: Backend is online, Neo4j healthy
  mockBackendOnline();
  mockGraphData({ nodes: [node1, node2], edges: [edge1] });
  
  // Act: Navigate to /graph page
  render(<GraphPage />);
  await waitFor(() => screen.getByTestId("graph-canvas"));
  
  // Assert: Graph renders (should PASS on unfixed code)
  expect(screen.getByTestId("graph-canvas")).toBeInTheDocument();
  expect(screen.queryByText(/failed to load/i)).not.toBeInTheDocument();
});

test("Preservation: Neo4j degraded shows correct message", async () => {
  // Arrange: Backend online, Neo4j offline
  mockBackendOnline();
  mockHealthStatus({ neo4j: false });
  
  // Act: Navigate to /graph page
  render(<GraphPage />);
  await waitFor(() => screen.getByText(/graph database is offline/i));
  
  // Assert: Shows Neo4j-specific message (should PASS on unfixed code)
  expect(screen.getByText(/graph database is offline/i)).toBeInTheDocument();
  expect(screen.queryByText(/backend gateway is offline/i)).not.toBeInTheDocument();
});
```

**Expected outcome on unfixed code:** Tests PASS - confirms baseline behavior to preserve

## Acceptance Criteria Mapping

| Requirement | Bug Condition | Expected Behavior | Preservation |
|-------------|---------------|-------------------|--------------|
| 1.1 | ✓ Generic error when backend offline | Clear backend offline message | - |
| 1.2 | ✓ API call fails without distinction | Detect and classify network errors | - |
| 1.3 | ✓ WebSocket fails without UI feedback | Display connection status | - |
| 1.4 | ✓ Health check unreachable | Distinguish backend vs Neo4j issues | - |
| 1.5 | ✓ Port resolution issues | Port-agnostic URL resolution | - |
| 2.1-2.5 | - | ✓ All expected behaviors | - |
| 3.1-3.6 | - | - | ✓ All preservation requirements |

## Risk Assessment

**Low risk areas:**
- Error message updates (UI only)
- Connection status display (additive)

**Medium risk areas:**
- API error classification logic (could affect error handling)
- WebSocket state exposure (needs careful state management)

**Mitigation:**
- Comprehensive preservation tests before implementation
- Incremental changes with test validation at each step
- Fallback to existing behavior if classification fails
