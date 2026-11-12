# Dragon-R1 Boot Sequence

Principles
- Add never remove. Consolidate duplicates instead of deleting.
- Layer-by-layer startup. Each layer can degrade without blocking lower layers.
- Only start what is needed for the current mode.

## Layers
Layer 0: Config + Models + Auth
- Purpose: Core types, settings, JWT, RBAC.
- External deps: none.

Layer 1: FastAPI Gateway
- Purpose: HTTP API, health, auth, basic CRUD.
- External deps: none beyond core Python packages.

Layer 2: LLM Adapter
- Purpose: Local or remote LLM connectivity (Ollama or provider).
- External deps: LLM provider service.
- Degraded mode: mock or local stub responses.

Layer 3: Memory + Knowledge
- Purpose: mem0 and RAG integrations.
- External deps: mem0 service, RAG service.
- Degraded mode: in-memory store.

Layer 4: Orchestrator + Pipeline + Phase Manager
- Purpose: task lifecycle, recon pipeline, approvals.
- External deps: optional Neo4j for graph updates.
- Degraded mode: run pipeline with no graph persistence.

Layer 5: MCP Tools + Agents
- Purpose: tool registry, validation, LangGraph agents.
- External deps: Kali/GVM tools, external services.
- Degraded mode: tool stubs, limited toolset.

Layer 6: Autonomous + Advanced
- Purpose: autonomous mode, advanced engines, self-evolution.
- External deps: optional and heavy; should never block earlier layers.
- Degraded mode: disabled by default.

Layer 7: Frontend
- Purpose: dragon-ui.
- External deps: gateway availability.

## Boot Order
1. Layer 0
2. Layer 1
3. Layer 2 (optional)
4. Layer 3 (optional)
5. Layer 4 (optional)
6. Layer 5 (optional)
7. Layer 6 (optional)
8. Layer 7 (optional)

## Degraded Mode Rules
- If a layer fails, log the failure, mark the layer as degraded, and continue.
- Health endpoint must report per-layer status with reason.
- Gateway must start even if Layers 2-6 are down.

## Entry Points
- Pre-flight: `dragon_boot_check.py`
- Gateway: `main.py` (progressive loader to be added in Phase 2)
