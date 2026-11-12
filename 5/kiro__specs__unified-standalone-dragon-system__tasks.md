# Dragon R1 Chat Command Deck

## Technical Design Update
- [x] Reframe the chat page as a command-deck layout: milestone rail on the left, live conversation in the center, execution graph on the right.
- [x] Keep the live workflow tracker driven by actual `agent_thought` / `agent_action` / `agent_observation` / `agent_message` websocket events instead of a static diagram.
- [x] Add lightweight topic-anchor generation in the gateway with LLM-first classification and heuristic fallback for degraded mode.

## Implementation Tasks
- [x] Add `/api/v1/chat/sessions/{session_id}/topics` to expose session anchors for sidebar navigation.
- [x] Broadcast `chat_topics_updated` websocket snapshots after completed chat/tool responses.
- [x] Build a smart navigation sidebar with smooth-scroll anchors bound to exact chat messages.
- [x] Build a `Chat+` control menu with `New Chat` and `Chat History` session switching.
- [x] Build a React Flow execution tracker that branches by run and updates live.
- [x] Fix chat scroll behavior so auto-follow only happens at the live edge and the down-arrow appears when the operator scrolls away.
- [x] Add Dragon-aligned chat theme variables and shell styling in the global UI theme.

## Verification
- [x] Add backend coverage for topic-anchor classification and deduplication.
- [x] Add frontend coverage for fallback anchor extraction and workflow run grouping.
