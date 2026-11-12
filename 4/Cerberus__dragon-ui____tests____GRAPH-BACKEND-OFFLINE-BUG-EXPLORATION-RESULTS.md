# Graph Backend Offline Bug Exploration Results

## Test Execution Date
March 30, 2026

## Test File
`__tests__/graph-backend-offline-bug-exploration.property.test.ts`

## Test Results on Unfixed Code

### Summary
✅ **Test executed successfully** - All tests FAILED as expected, confirming the bug exists

**Total Tests**: 7
**Failed**: 6 (expected failures - bug confirmed)
**Passed**: 1 (port-agnostic URL resolution already exists)

## Bugs Detected

### 1. Missing ErrorType Enum ❌
**Status**: CONFIRMED BUG
**Test**: `should detect missing ErrorType enum in API module`
**Finding**: No ErrorType enum exists in api.ts for error classification
**Expected**: Enum with values: BACKEND_OFFLINE, NEO4J_DEGRADED, EMPTY_GRAPH, UNKNOWN
**Impact**: Cannot classify errors for proper user messaging

### 2. Missing classifyError Function ❌
**Status**: CONFIRMED BUG
**Test**: `should detect missing classifyError function in API module`
**Finding**: No classifyError(error, healthStatus) function exists
**Expected**: Function to detect network errors and classify them
**Impact**: Cannot distinguish between different error scenarios

### 3. Network Errors Lack User-Friendly Messages ❌
**Status**: CONFIRMED BUG
**Test**: `should detect that network errors are not enhanced with user-friendly messages`
**Finding**: Raw network errors (ECONNREFUSED, ETIMEDOUT) shown without enhancement
**Expected**: Errors should include:
  - errorType: "BACKEND_OFFLINE"
  - userMessage: "Backend gateway is offline. Please start the Dragon-R1 gateway service."
  - instructions: "Run START-DRAGON-R1.bat (Windows) or ./start-dragon-r1.sh (Linux/Mac)"
**Impact**: Users see technical error codes instead of actionable guidance

### 4. Cannot Distinguish Backend Offline from Neo4j Degraded ❌
**Status**: CONFIRMED BUG
**Test**: `should detect lack of distinction between backend offline and Neo4j degraded`
**Finding**: No classification logic to distinguish error types
**Expected**: classifyError should check:
  - If error.code in [ECONNREFUSED, ETIMEDOUT, NetworkError] → BACKEND_OFFLINE
  - If healthStatus.reachable && !healthStatus.neo4j_healthy → NEO4J_DEGRADED
  - If healthStatus.reachable && graphData.nodes.length === 0 → EMPTY_GRAPH
**Impact**: Users cannot identify which service needs attention

### 5. Port-Agnostic Base URL Resolution ✅
**Status**: ALREADY IMPLEMENTED
**Test**: `should detect missing port-agnostic base URL resolution`
**Finding**: normalizeLocalApiBaseUrl function exists and works correctly
**Note**: This is not a bug - the function already handles non-default ports

### 6. Graph Page Shows Generic Error ❌
**Status**: CONFIRMED BUG
**Test**: `should detect that graph page shows generic error for backend offline`
**Finding**: Graph page shows "Failed to load the attack graph right now." for all errors
**Current Behavior** (page.tsx ~line 150):
  - Generic error message for all failure scenarios
  - No distinction between backend offline vs Neo4j degraded
  - No startup instructions provided
**Expected Behavior**:
  - Check error type from classifyError()
  - Show "Backend gateway is offline" for BACKEND_OFFLINE
  - Show "Graph database is offline" for NEO4J_DEGRADED
  - Include startup instructions for BACKEND_OFFLINE
**Impact**: Users cannot understand root cause or how to fix it

### 7. Missing WebSocket Connection Status in Error Display ❌
**Status**: CONFIRMED BUG
**Test**: `should detect missing WebSocket connection status in error display`
**Finding**: Error message doesn't reflect WebSocket connection status
**Expected**: When backend is offline:
  - Display WebSocket connection status prominently
  - Show "Real-time updates unavailable - backend disconnected"
  - Update error message based on connection state
**Impact**: Users don't know real-time features are unavailable

## Counterexamples Found

The test successfully identified the following counterexamples that demonstrate the bug:

1. **Missing Error Classification Infrastructure**:
   - No ErrorType enum in api.ts
   - No classifyError function in api.ts
   - Network errors not enhanced with user-friendly messages

2. **Generic Error Messaging**:
   - Graph page shows "Failed to load the attack graph right now." for all errors
   - No distinction between different failure scenarios
   - No actionable guidance for users

3. **Missing Connection Status**:
   - WebSocket disconnection not reflected in error messaging
   - Users unaware of real-time feature unavailability

## Next Steps

These test failures confirm the bug exists and provide clear requirements for the fix:

1. **Task 3.1**: Add error classification logic to API client
   - Create ErrorType enum
   - Implement classifyError function
   - Detect network errors (ECONNREFUSED, ETIMEDOUT)
   - Distinguish from HTTP errors (4xx, 5xx)

2. **Task 3.2**: Update graph page error messaging
   - Use error classification from API
   - Show specific messages based on ErrorType
   - Include startup instructions for BACKEND_OFFLINE

3. **Task 3.3**: Expose WebSocket connection state
   - Make connection status visible to components
   - Provide disconnection reason

4. **Task 3.4**: Update Zustand store
   - Add backendConnectivity state
   - Add errorType state
   - Track connection status changes

## Test Command
```bash
cd Cerberus/dragon-ui
npm test -- graph-backend-offline-bug-exploration.property.test.ts
```

## Expected Outcome After Fix
After implementing Tasks 3.1-3.4, this same test should PASS, confirming:
- Error classification infrastructure exists
- Network errors are properly classified
- User-friendly messages and instructions are provided
- Graph page shows specific error messages
- WebSocket status is reflected in error display
