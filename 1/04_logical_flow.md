# Logical Flow

## End-to-end flow

1. User uploads authorization package, target scope, and artifacts.
2. `Policy Agent` validates scope and blocks unsupported requests.
3. Ingestion normalizer parses code, logs, configs, scanner results, prior reports, and architecture documents.
4. `Context Engine` segments artifacts and builds:
   - metadata index
   - vector index
   - graph relationships
   - replayable context packs
5. `Recon Agent` builds an authorized asset-and-exposure map from supplied evidence.
6. `Analysis Agent` correlates evidence against weakness, control, and dependency knowledge.
7. `Model Orchestration Layer` fingerprints the task and selects named engines.
8. One or more model engines run in parallel and return structured candidates.
9. `Simulation Agent` builds attack graphs and control-failure scenarios in simulation only.
10. `Intelligence Fusion Agent` merges model outputs, simulation traces, and evidence.
11. `Judge Agent` checks contradictions, missing citations, and policy violations.
12. `Optimization Agent` records telemetry and proposes future routing or retrieval improvements.
13. Reporting layer emits:
   - executive summary
   - technical report
   - remediation backlog
   - YC-style presentation narrative

## Sequence diagram

```mermaid
sequenceDiagram
    participant U as User
    participant P as Policy Agent
    participant C as Context Engine
    participant R as Recon Agent
    participant A as Analysis Agent
    participant M as Model Router
    participant E as Named Engines
    participant S as Simulation Agent
    participant F as Intelligence Fusion Agent
    participant J as Judge Agent
    participant O as Optimization Agent
    participant X as Reporting

    U->>P: Upload scope, evidence, authorization
    P->>C: Approved artifacts only
    C->>R: Context pack + asset graph
    R->>A: Authorized exposure map
    A->>M: Task fingerprint + retrieval hints
    M->>E: Parallel engine calls
    E-->>F: Structured candidates
    A->>S: Findings + graph state
    S-->>F: Simulation traces
    F->>J: Fused candidate
    J-->>X: Approved narrative + citations
    J-->>O: Telemetry + errors + gaps
    X-->>U: Reports + deck + backlog
```

## Context handling flow

```mermaid
flowchart LR
    RAW["Raw Artifacts"] --> SEG["Segmentation"]
    SEG --> EMB["Embedding / Indexing"]
    SEG --> PARSE["Structured Parsing"]
    PARSE --> GRAPH["Graph Build"]
    EMB --> RET["Top-k Retrieval"]
    GRAPH --> RET
    RET --> SUM["Compression / Summaries"]
    SUM --> PACK["Task Context Pack"]
```

## Routing flow

1. classify task
2. apply policy constraints
3. determine required modalities
4. estimate context size
5. shortlist engines
6. decide single-engine versus ensemble
7. run structured prompts
8. normalize and score results

## Failure handling flow

- if scope invalid: stop
- if evidence incomplete: request more artifacts
- if model disagreement high: escalate to human review
- if retrieval confidence low: rebuild context pack
- if simulation output lacks evidence: suppress finding
