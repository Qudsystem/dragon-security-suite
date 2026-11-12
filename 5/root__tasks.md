# Dragon R1 Chat Interface - Implementation Tasks

## Technical Design Update (2026-04-03)

- Frontend chat surface is extended into a three-zone real-time workspace:
  - Left: Smart topic sidebar with LLM/fallback anchors and smooth message-index navigation.
  - Center: Streaming chat surface with smart scroll-to-bottom utility and new message indicator.
  - Right: Live workflow tracker (React Flow) fed by websocket agent + task lifecycle events.
- Session-aware websocket routing is now explicit:
  - Global stream: `agent_chat`
  - Session stream: `agent_{session_id}`
  - Client subscribes to session channel on session switch to avoid cross-session telemetry bleed.
- Topic detection architecture:
  - Primary: backend lightweight LLM classification in `chat_service.get_topic_anchors()`.
  - Fallback: deterministic keyword-based classifier in frontend `buildFallbackTopicAnchors()`.
  - Delivery: REST fetch + push updates over `chat_topics_updated`.
- Chat+ menu UX architecture:
  - Primary action: New Chat (session reset/create).
  - Secondary action: Chat History with refresh and session quick-switch.
  - Data source: backend session APIs (persisted chat sessions).

## Updated Task List

- [x] Implement smart navigation sidebar with live anchor updates and smooth scrolling.
- [x] Ensure topic anchors are emitted from backend and consumed via websocket (`chat_topics_updated`).
- [x] Add `Chat+` menu with:
  - [x] New Chat action.
  - [x] Chat History list and refresh.
- [x] Implement smart scroll-to-bottom control that appears away from live edge.
- [x] Add live workflow tracker and wire to real websocket events.
- [x] Integrate task lifecycle events (`task_started`, `task_progress`, `task_phase_update`, `task_completed`, `task_failed`) into workflow visualization.
- [x] Add session-scoped websocket subscription (`agent_{session_id}`) to keep telemetry accurate per conversation.
- [x] Broadcast topic updates to both global and session channels on backend.
- [x] Harden Windows launcher (`START-DRAGON-R1.bat`) for broken PATH environments (`cmd`/`choice`) and local `.venv` Python fallback.

## Next Validation Steps

- [ ] Run frontend lint/test focused on chat components (blocked: Node/npm unavailable in current shell host).
- [ ] Run gateway tests for websocket + chat topic anchors (blocked: Python package tooling broken in current local interpreter).
- [ ] Verify in Podman environment with backend reachable from UI (blocked: Podman machine unavailable because `wsl` is missing from PATH/runtime).
