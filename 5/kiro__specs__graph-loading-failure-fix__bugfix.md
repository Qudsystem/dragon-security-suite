# Bugfix Requirements Document

## Introduction

The graph visualization page at `/graph` displays an error message "Graph data is unavailable. Failed to load the attack graph right now." when the backend gateway service is not running or unreachable. This prevents users from viewing the attack surface map and understanding the security topology of their targets.

The issue occurs because the frontend attempts to fetch graph data from the backend API endpoint `/api/v1/graph` and establish a WebSocket connection to `ws://localhost:8000/ws/`, but when the gateway is not running, these requests fail without providing clear guidance to the user about the root cause or remediation steps.

## Bug Analysis

### Current Behavior (Defect)

1.1 WHEN the user navigates to the `/graph` page and the backend gateway is not running THEN the system displays a generic error message "Failed to load the attack graph right now" without indicating that the backend service is offline

1.2 WHEN the frontend attempts to fetch graph data via `api.getGraphOverview()` and the backend is unreachable THEN the API call fails with a network error but the error message does not distinguish between backend unavailability and actual graph data issues

1.3 WHEN the WebSocket connection to `ws://localhost:8000/ws/` fails due to the backend being offline THEN the frontend logs multiple retry attempts and displays "Disconnected from Dragon-R1" but the graph page does not reflect this connection state in its error messaging

1.4 WHEN the backend health check endpoint `/api/v1/health` is unreachable THEN the graph page cannot determine the Neo4j service status and defaults to showing "Attack graph service status is unavailable" without clarifying that the entire backend is offline

1.5 WHEN the user accesses the UI on port 3001 instead of the default port 3000 THEN the API requests may fail due to incorrect base URL resolution if the backend is configured for a specific port

### Expected Behavior (Correct)

2.1 WHEN the user navigates to the `/graph` page and the backend gateway is not running THEN the system SHALL display a clear error message indicating "Backend gateway is offline. Please start the Dragon-R1 gateway service." with instructions on how to start the service

2.2 WHEN the frontend attempts to fetch graph data via `api.getGraphOverview()` and the backend is unreachable THEN the system SHALL detect the network error, distinguish it from other error types, and display a user-friendly message indicating backend connectivity issues

2.3 WHEN the WebSocket connection to `ws://localhost:8000/ws/` fails due to the backend being offline THEN the graph page SHALL display the connection status prominently and update the error message to reflect that real-time updates are unavailable due to backend disconnection

2.4 WHEN the backend health check endpoint `/api/v1/health` is unreachable THEN the graph page SHALL recognize this as a backend connectivity issue and display appropriate messaging that distinguishes between "backend offline" and "Neo4j service degraded"

2.5 WHEN the user accesses the UI on a non-default port THEN the API client SHALL correctly resolve the backend URL based on the current window location to ensure requests are routed properly

### Unchanged Behavior (Regression Prevention)

3.1 WHEN the backend gateway is running and Neo4j is healthy THEN the system SHALL CONTINUE TO load and display the attack graph visualization with nodes and edges as expected

3.2 WHEN the backend gateway is running but Neo4j is degraded or unavailable THEN the system SHALL CONTINUE TO display the appropriate service status message "Graph database is offline" and show the empty graph state with retry option

3.3 WHEN the backend gateway is running and returns an empty graph (no nodes/edges) THEN the system SHALL CONTINUE TO display the message "No graph data has been indexed yet. Start or resume a task to populate the attack graph."

3.4 WHEN the user clicks the "Retry" button on the error overlay THEN the system SHALL CONTINUE TO re-fetch graph data and health status to check if services have become available

3.5 WHEN the WebSocket connection is established successfully THEN the system SHALL CONTINUE TO subscribe to graph update channels (`graph_update`, `node_discovery`, `vulnerability_alert`) for real-time updates

3.6 WHEN the user interacts with graph nodes and edges THEN the system SHALL CONTINUE TO display node details in the sidebar panel and support all existing graph visualization features
