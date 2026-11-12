# Bugfix Requirements Document

## Introduction

The Dragon-R1 chat interface at http://localhost:3001/chat is not receiving responses from the LLM model when users send messages, preventing task execution and autonomous operations. While Ollama is running and the deepseek-r1:1.5b model is downloaded, the chat flow fails to deliver responses back to the frontend. This issue manifests as:

- Heartbeat timeouts causing unstable WebSocket connections
- Empty validation rules array in the frontend
- "LLM backend is not reachable" error messages
- Messages sent but no assistant responses received

The root cause appears to be a combination of:
1. WebSocket connection instability between frontend and backend
2. LLM adapter failing to reach Ollama (likely due to Podman container networking)
3. Missing error propagation when LLM calls fail
4. No fallback response mechanism when the LLM is unreachable

## Bug Analysis

### Current Behavior (Defect)

1.1 WHEN a user sends a chat message through the WebSocket connection THEN the system experiences heartbeat timeouts and frequent disconnections

1.2 WHEN the backend attempts to call Ollama at http://127.0.0.1:11434 from within a Podman container THEN the connection fails because localhost resolves to the container's localhost, not the host machine

1.3 WHEN the LLM adapter fails to reach Ollama THEN the chat_service._generate_response() catches the exception but returns a generic "LLM backend is not reachable" message without logging the actual error details

1.4 WHEN chat_service.process_message() completes with the fallback message THEN the WebSocket handler broadcasts the response, but the frontend may have already disconnected due to heartbeat timeout

1.5 WHEN the WebSocket connection is unstable THEN the frontend shows "Unstable Connection" and messages may not be delivered or received properly

1.6 WHEN the ValidationManager receives an empty rules array THEN the frontend cannot properly validate or process chat interactions

### Expected Behavior (Correct)

2.1 WHEN a user sends a chat message through the WebSocket connection THEN the system SHALL maintain a stable connection with successful heartbeat responses

2.2 WHEN the backend attempts to call Ollama from within a Podman container THEN the system SHALL use the correct host reference (host.containers.internal or host network mode) to reach the host machine's Ollama instance

2.3 WHEN the LLM adapter fails to reach Ollama THEN the system SHALL log detailed error information (connection refused, timeout, DNS resolution failure) and attempt fallback strategies before returning a user-facing error

2.4 WHEN chat_service.process_message() completes THEN the WebSocket handler SHALL successfully broadcast the response to the connected client with proper message structure

2.5 WHEN the WebSocket connection experiences issues THEN the system SHALL implement exponential backoff reconnection with clear status updates to the user

2.6 WHEN the ValidationManager initializes THEN the system SHALL receive and load the proper validation rules configuration

2.7 WHEN the LLM is temporarily unreachable THEN the system SHALL provide a helpful response indicating the specific issue (e.g., "Ollama connection failed at http://127.0.0.1:11434 - verify service is running") rather than a generic error

### Unchanged Behavior (Regression Prevention)

3.1 WHEN a user sends a tool execution command (e.g., "/run nmap target") THEN the system SHALL CONTINUE TO parse and execute the command with proper authorization checks

3.2 WHEN a user sends a "pensec" command (e.g., "pensec tasks") THEN the system SHALL CONTINUE TO handle it with the hardcoded command responses

3.3 WHEN the LLM successfully responds to a chat message THEN the system SHALL CONTINUE TO broadcast the response through WebSocket with proper message structure

3.4 WHEN chat sessions are created, listed, or deleted THEN the system SHALL CONTINUE TO persist them to the file-backed storage at gateway/data/chat_sessions.json

3.5 WHEN the WebSocket connection is established THEN the system SHALL CONTINUE TO authenticate, subscribe to channels (agent_chat, approvals, task_updates), and start the heartbeat mechanism

3.6 WHEN the frontend receives agent_thought, agent_action, or agent_observation events THEN the system SHALL CONTINUE TO update the workflow steps and mission progress indicators

3.7 WHEN a chat message times out after 60 seconds THEN the system SHALL CONTINUE TO display the timeout error in the UI with appropriate workflow status updates
