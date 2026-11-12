# Single-Machine MVP Plan

## Goal

Run the entire authorized validation workflow on one workstation first.

## Recommended deployment profiles

### Profile A. Recommended

- host OS: `Ubuntu 22.04/24.04 LTS`
- local retrieval: `FAISS`
- metadata: `SQLite`
- graph: embedded graph or small `Neo4j`
- orchestration: `FastAPI` + `LangGraph`
- sandbox: Linux isolation for lab-contained validation only

### Profile B. Windows-first compromise

- host OS: `Windows 11`
- control plane: native Python services
- local lab isolation: `WSL2`, `Hyper-V`, or Windows-native sandbox abstraction
- same engine and storage contracts as Profile A

## Process map

| Process | Responsibility |
| --- | --- |
| `api` | UI and external interface |
| `orchestrator` | run lifecycle and agent graph |
| `retrieval` | vector, graph, and file access |
| `engine-router` | engine selection and health |
| `simulation` | attack-graph and control simulation |
| `reporting` | HTML, JSON, and PPTX generation |

## Memory and storage strategy

The safe MVP does not promise magical resident `100M` context. It uses:

- RAM for hot context
- local vector index for semantic recall
- local graph store for relationships
- disk artifacts for full evidence
- summary packs for replay

Optional systems-level optimizations preserved from the source set:

- `OpenVINO`
- `AVX2`
- `TurboQuant`
- `ZRAM`
- `mmap`

## Default local stack

- `Python 3.11+`
- `FastAPI`
- `Pydantic`
- `SQLite`
- `FAISS`
- filesystem evidence store
- optional `Redis`
- optional `Neo4j`

## Single-machine MVP milestones

1. scope intake works
2. evidence is indexed
3. all five required agents run
4. model router supports at least one local engine and one remote engine
5. simulation traces are generated
6. final report is produced
7. run replay succeeds

## Resource discipline

- prefer small context packs over giant prompts
- use ensemble mode only when ambiguity is high
- cache retrieval results aggressively
- keep report rendering asynchronous

## What belongs on the same machine today

- API
- orchestrator
- vector store
- graph store
- artifact files
- report generation
- lightweight local engine runtime

## What can remain optional in the MVP

- BitNet-specific acceleration
- OpenVINO-specific optimization
- specialist local engines
- heavy lab validation
