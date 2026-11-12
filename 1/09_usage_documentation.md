# Usage Documentation

## Intended repo layout

```text
dragon_r1/
  core/
  agents/
  engines/
  fusion/
  context/
  simulation/
  storage/
  reports/
  policy/
  configs/
  examples/
  tests/
```

## Core run sequence

1. create an authorization record
2. create a project workspace
3. upload evidence and scope
4. build indexes
5. run the multi-agent workflow
6. review fused findings
7. export reports and deck

## Required inputs

- engagement authorization
- target scope
- owner-provided asset list
- artifacts to analyze
- preferred engine policy

## Example configuration

```yaml
project:
  name: "dragon-r1-pilot"
  mode: "authorized-validation"
policy:
  simulation_only: true
  allow_remote_models: true
  require_written_scope: true
engines:
  primary_reasoning: "Claude Engine"
  secondary_reasoning: "GPT Engine"
  local_fallback: "Gemma Engine"
context:
  max_hot_context_tokens: 24000
  retrieval_top_k: 24
  graph_hops: 2
reports:
  formats: ["md", "html", "json", "pptx"]
```

## Run modes

### Local-only

- uses local engines only
- suitable for offline or restricted environments

### Hybrid

- uses local retrieval and storage
- calls selected remote frontier engines

### Cloud-assisted

- same architecture, remote-heavy routing
- still policy-bound and evidence-driven

## Output package

- executive summary
- technical findings
- remediation backlog
- evidence map
- PPTX pitch or board deck

## Agent execution rules

- `Recon Agent` never extends beyond scope
- `Analysis Agent` never emits a finding without evidence
- `Simulation Agent` never performs live offensive execution
- `Intelligence Fusion Agent` never returns a final verdict without citation coverage
- `Optimization Agent` never changes safety policy

## Review checklist for users

- confirm scope is correct
- confirm excluded systems are listed
- review unresolved uncertainty items
- confirm simulations are marked as simulated
- review final severity and remediation prioritization
