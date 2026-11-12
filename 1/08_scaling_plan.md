# Scaling Plan

## Scaling philosophy

Do not start distributed. Start with a deterministic single node, then split the system along natural boundaries.

## Stage 1. Single-node baseline

- one machine
- embedded storage
- one orchestrator
- small engine pool

## Stage 2. Split storage plane

- externalize vector store
- externalize graph store
- move artifact blobs to object storage
- keep control plane on one host

## Stage 3. Split model gateway

- central `Model Gateway`
- per-provider rate and budget controls
- local engine workers and remote engine adapters

## Stage 4. Split simulation and report workers

- queued simulation jobs
- queued report generation jobs
- isolated lab validation workers

## Stage 5. Enterprise scale

- multi-tenant workspaces
- policy packs per tenant
- audit log export
- dedicated engine budgets
- multi-region artifact storage where needed

## Service boundaries

| Service | Responsibility |
| --- | --- |
| `Control Plane` | orchestration, auth, policy |
| `Model Gateway` | engine registry, routing, budgeting |
| `Retrieval Plane` | vector, graph, and artifact access |
| `Simulation Workers` | graph simulation and lab replay |
| `Report Workers` | long-form outputs and deck generation |
| `Observability Plane` | tracing, metrics, run audit |

## Scaling triggers

- more than one concurrent enterprise run
- report rendering backlog
- model routing p95 breach
- retrieval p95 breach
- single-node memory pressure

## Safety during scale-out

- no worker may bypass central policy
- fusion requires evidence regardless of cluster size
- simulation workers stay lab-contained
- unsafe outputs are centrally quarantined
