# Agents Design

## Required agents

### 1. Recon Agent

**Role**

Builds an authorized understanding of assets, services, identities, dependencies, and evidence already in scope.

**Inputs**

- scope package
- asset inventory
- architecture files
- approved scanner outputs
- logs and configs

**Outputs**

- asset graph
- exposure map
- uncertainty list
- retrieval hints for other agents

**Forbidden**

- unapproved live scanning
- bypassing scope
- internet-side discovery outside authorization

### 2. Analysis Agent

**Role**

Correlates evidence into plausible findings and remediation candidates.

**Inputs**

- asset graph
- retrieval results
- vulnerability and control knowledge
- code and config evidence

**Outputs**

- preliminary findings
- confidence estimates
- evidence bundles
- simulation requests

**Forbidden**

- generating live payloads
- issuing live exploitation instructions

### 3. Simulation Agent

**Role**

Models attack paths, control gaps, and blast radius in simulation only.

**Inputs**

- asset graph
- preliminary findings
- control map
- environment assumptions

**Outputs**

- simulated attack graph
- ranked scenario list
- required preconditions
- impact estimates

**Forbidden**

- live exploit execution
- persistence
- stealth tuning
- credential theft

### 4. Intelligence Fusion Agent

**Role**

Merges model outputs, simulation traces, and evidence into a single decision object.

**Inputs**

- engine candidates
- simulation traces
- citations
- prior successful patterns

**Outputs**

- fused verdict
- contradiction report
- confidence score
- escalation flag

### 5. Optimization Agent

**Role**

Improves future runs by tuning prompts, routing, retrieval depth, and caching policies within approved boundaries.

**Inputs**

- latency metrics
- disagreement statistics
- report acceptance data
- human feedback

**Outputs**

- routing updates
- prompt revisions
- retrieval policy suggestions
- benchmark deltas

**Forbidden**

- relaxing safety policy
- introducing new tools without approval

## Supporting agents

### Policy Agent

- validates scope
- enforces denied behaviors
- blocks unsafe outputs

### Evidence Agent

- manages citations
- checks artifact integrity
- builds finding-to-evidence trace maps

### Judge Agent

- validates output completeness
- detects contradictions
- enforces evidence thresholds

### Report Agent

- converts the final decision object into executive, technical, and board-ready outputs

## Agent contracts

Every agent should emit:

```json
{
  "agent_name": "string",
  "task_id": "string",
  "status": "ok|needs_more_evidence|blocked|escalate",
  "confidence": 0.0,
  "evidence_refs": ["artifact-id"],
  "claims": [],
  "gaps": [],
  "next_actions": []
}
```

## Agent coordination model

The source set strongly prefers `LangGraph` for macro orchestration and `CrewAI` for micro specialization. The safe design keeps that split:

- macro routing and run state: `LangGraph`-style state graph
- specialist collaboration inside bounded nodes: `CrewAI`-style teams

## Benchmark expectations per agent

| Agent | Claude-class baseline | Dragon R1 advantage target |
| --- | --- | --- |
| `Recon Agent` | broad reasoning over artifacts | stronger structured evidence graph |
| `Analysis Agent` | strong narrative synthesis | tighter citation and schema discipline |
| `Simulation Agent` | general scenario reasoning | explicit graph simulation and replay |
| `Intelligence Fusion Agent` | implicit self-consistency | explicit multi-model consensus scoring |
| `Optimization Agent` | prompt iteration | measured routing, caching, and benchmark loops |
