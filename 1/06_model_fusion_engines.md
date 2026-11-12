# Model Fusion Engines

## 1. Purpose

The source requirement calls for:

- a named engine per model
- intelligent routing
- a fusion layer
- an ensemble system that produces a stronger final output

This file defines the safe design for that requirement.

## 2. Engine registry

| Engine | Primary use | Notes |
| --- | --- | --- |
| `Claude Engine` | deep analysis, coding, structured reasoning, long-context synthesis | benchmarked against current Anthropic model family |
| `GPT Engine` | broad tool use, general reasoning, coding, report synthesis | strong generalist engine |
| `Gemini Engine` | multimodal analysis, large-input synthesis, workflow support | useful when image or multimodal reasoning matters |
| `DeepSeek Engine` | lower-cost deep reasoning and open-weight experimentation | strong reasoning bench candidate |
| `Qwen Engine` | coding, multilingual, open-weight flexibility | useful local or hosted |
| `Gemma Engine` | compact open model option for constrained environments | source set highlights `Gemma 4` |
| `Llama Engine` | broad open-weight ecosystem support | useful fallback and specialization |
| `Mistral Specialist Engine` | specialist OSS runs, including source-mentioned `Dolphin` and `Hermes 2 Pro` references | use only under policy |
| `BitNet Engine` | experimental local ultra-efficient draft or triage engine | retained because heavily emphasized in sources |
| `Fallback Local Engine` | offline-safe continuity mode | deterministic degraded service |

## 3. Verified Claude note

Before finalizing this package, current official Anthropic references were checked because the source set mixes aspirational and inconsistent naming.

- Anthropic system-card and model pages show `Claude Sonnet 4.6` and `Claude Opus 4.6` in February 2026.
- Anthropic announced `Claude Opus 4.7` on April 16, 2026.

This means the source set's `Opus 4.6 / Sonnet 4.6` benchmark language is reasonable as a dated target, but the output package should use exact dates when talking about "latest" model references.

## 4. Per-engine contracts

Each engine must publish:

```json
{
  "engine_name": "Claude Engine",
  "provider": "anthropic",
  "capabilities": ["analysis", "coding", "reporting"],
  "context_profile": "long|medium|short",
  "cost_profile": "high|medium|low",
  "latency_profile": "fast|balanced|slow",
  "modalities": ["text", "image"],
  "safety_profile": "strict",
  "best_for": ["reasoning", "auditing", "code review"]
}
```

## 5. Routing logic

### 5.1 Inputs to routing

- task type
- context size
- modality needs
- policy level
- latency budget
- cost budget
- evidence density
- user preference for local, cloud, or hybrid

### 5.2 Routing rules

1. Apply policy filter first.
2. Disqualify engines that cannot accept the modality or context profile.
3. Score remaining engines on fit, cost, and latency.
4. Decide single-engine, dual-engine, or ensemble mode.
5. Normalize output into shared schema.

### 5.3 Suggested default patterns

- short deterministic formatting: `Fallback Local Engine` or lightweight open model
- code-plus-analysis: `Claude Engine` plus `GPT Engine` or `Qwen Engine`
- multimodal design or screenshots: `Gemini Engine` plus one reasoning engine
- low-connectivity mode: `BitNet Engine` plus `Gemma Engine` or `Llama Engine`
- difficult ambiguity: `Claude Engine`, `GPT Engine`, and `DeepSeek Engine` in ensemble

## 6. Fusion layer

The `Fusion Layer` merges outputs through six steps:

1. schema normalization
2. citation merge
3. contradiction detection
4. evidence-weighted ranking
5. confidence calibration
6. final synthesis

### 6.1 Scoring dimensions

- `evidence_score`
- `reasoning_score`
- `policy_score`
- `task_fit_score`
- `consistency_score`
- `latency_score`

### 6.2 Candidate object

```json
{
  "engine": "Claude Engine",
  "answer": "structured content",
  "citations": ["artifact-1", "artifact-2"],
  "claims": [],
  "confidence": 0.91,
  "risks": [],
  "policy_flags": []
}
```

### 6.3 Fusion output

```json
{
  "verdict": "final structured answer",
  "winning_engines": ["Claude Engine", "DeepSeek Engine"],
  "confidence": 0.89,
  "consensus_level": "high",
  "dissent_notes": [],
  "evidence_refs": []
}
```

## 7. Reasoning patterns preserved from the source set

The source set repeatedly references:

- `Chain-of-Thought`
- `Self-Verification`
- `Process Reward Model`
- `Tree-of-Thoughts`
- `Graph-of-Thoughts`
- `MCTS`
- `Multi-Path Reasoning`
- `Self-Consistency`

Safe usage in Dragon R1:

- use these internally as reasoning and evaluation strategies
- do not expose raw reasoning traces unless explicitly configured for internal audit
- compress traces into decision summaries with evidence links

## 8. BitNet role in the fusion stack

`BitNet` is preserved because it is central to the sources, but in the product architecture it is best treated as:

- local triage engine
- draft engine
- cache warm-up engine
- low-cost summarization engine

It should not be treated as proof that the system already has frontier parity by itself.

## 9. Claude-level enforcement reinterpreted safely

The source requirement says the final output must be `>= Claude`.

The defensible engineering translation is:

- compare every subsystem against a strong `Claude` baseline
- add explicit memory, routing, validation, and evidence layers where a single model is insufficient
- benchmark parity or advantage on defined workflow tasks, not on vague global intelligence claims

## 10. Benchmark pack

Create benchmark suites for:

- finding synthesis accuracy
- report quality
- citation coverage
- contradiction detection
- simulated attack-graph quality
- remediation usefulness

Track three baselines:

- best single-engine run
- best dual-engine ensemble
- full Dragon R1 fusion pipeline
