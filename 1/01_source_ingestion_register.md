# Source Ingestion Register

## Read set

The following authored source files were read and incorporated into this design package:

- `D2.md`
- `dragon_r1_full_docs.html`
- `D1.txt`
- `D1_REFINED.md`
- `Dragon_R1_Architecture_v1.docx` via extracted text copy in `tmp/docs/Dragon_R1_Architecture_v1.txt`

The following authored HTML files were also reviewed because they add project framing, reports, or usage context:

- `reports/Dashboard-Dragon-R1-2026-04-09.html`
- `reports/Presentation-Dragon-R1-Slides-2026-04-09.html`
- `Cerberus/gateway/reports/dragon_r1_localhost_20260227_015353.html`
- `Cerberus/gateway/reports/dragon_r1_localhost_20260325_161543.html`
- `report_templates/html_template.html`
- `Cerberus/gateway/report_templates/html_template.html`
- `volto-ui/index.html`
- `volto-ui/src/test-pipeline.html`

Generated build artifacts such as `.next`, coverage reports, and dependency HTML under `node_modules` were treated as generated outputs rather than authored product requirements sources.

## Locked model names

| Locked item | Seen in source set | Where it appears in output package | Status |
| --- | --- | --- | --- |
| `Claude 3.5 Sonnet` | `D1_REFINED.md`, `D1.txt` | `06_model_fusion_engines.md`, benchmark discussion | retained |
| `Claude 3.7 Sonnet` | `D2.md` | `06_model_fusion_engines.md`, benchmark discussion | retained |
| `Claude 4.5` | `D1_REFINED.md` | `06_model_fusion_engines.md`, benchmark discussion | retained |
| `Claude Opus 4.6` | `D1.txt` | `06_model_fusion_engines.md`, benchmark target notes | retained |
| `Claude Sonnet 4.6` | `D1.txt` | `06_model_fusion_engines.md`, benchmark target notes | retained |
| `GPT-4` | `D1_REFINED.md`, `D1.txt` | `06_model_fusion_engines.md` | retained |
| `GPT-4o` | `D2.md` | `06_model_fusion_engines.md` | retained |
| `GPT-5` | `D2.md`, `D1_REFINED.md` | `06_model_fusion_engines.md` | retained |
| `GPT-5.2` | `D2.md`, `dragon_r1_full_docs.html`, `D1.txt` | `06_model_fusion_engines.md` | retained |
| `Gemini 1.5 Flash` | `D2.md` | `06_model_fusion_engines.md` | retained |
| `Gemini 3.1 Pro` | `D2.md`, `D1_REFINED.md`, `D1.txt` | `06_model_fusion_engines.md` | retained |
| `Gemma 4` | `D2.md`, `D1_REFINED.md`, `D1.txt`, `Dragon_R1_Architecture_v1.docx` | `06_model_fusion_engines.md` | retained |
| `Muse Spark` | `D2.md`, `D1_REFINED.md`, `Dragon_R1_Architecture_v1.docx` | `06_model_fusion_engines.md` | retained |
| `BitNet b1.58` | all major source docs | `PRD_Dragon_R1.md`, `03_system_architecture.md`, `06_model_fusion_engines.md` | retained |
| `Qwen 30B` | `D2.md` | `01_source_ingestion_register.md`, compression notes | retained |
| `Qwen2.5-Coder` | `D1.txt` | `06_model_fusion_engines.md` | retained |
| `DeepSeek-R1` | `D1_REFINED.md`, `D1.txt` | `06_model_fusion_engines.md` | retained |
| `DeepSeek-R1-Distill-Qwen-32B` | `D1.txt` | `06_model_fusion_engines.md` | retained |
| `QwQ-32B-Preview` | `D1.txt` | `06_model_fusion_engines.md` | retained |
| `Llama 3.1` | `D2.md`, `D1_REFINED.md` | `06_model_fusion_engines.md` | retained |
| `TinyLlama 1B` | `D2.md` | `03_system_architecture.md`, `06_model_fusion_engines.md` | retained |
| `Dolphin` | `D1_REFINED.md` | `06_model_fusion_engines.md` | retained as specialist-source reference |
| `Hermes 2 Pro` | `D1_REFINED.md` | `06_model_fusion_engines.md` | retained as specialist-source reference |
| `CyberLLaMA` | `D1_REFINED.md` | `06_model_fusion_engines.md` | retained as source reference, not default |
| `Groq` | `D2.md` | `06_model_fusion_engines.md` | retained as market reference |
| `PentestGPT` | `D1_REFINED.md`, `dragon_r1_full_docs.html`, `Dragon_R1_Architecture_v1.docx` | safety and market context | retained as competitor reference |
| `RedAmon` | `D1_REFINED.md`, `Dragon_R1_Architecture_v1.docx` | safety and market context | retained as competitor reference |
| `Metasploit AI` | `dragon_r1_full_docs.html` | safety and market context | retained as competitor reference |

## Locked techniques and concepts

| Locked item | Output location | Treatment |
| --- | --- | --- |
| `TurboQuant` | `03_system_architecture.md`, `06_model_fusion_engines.md` | adapted as cache and transport optimization research track |
| `OpenVINO` | `03_system_architecture.md`, `07_single_machine_mvp_plan.md` | retained as local acceleration option |
| `AVX2` | `03_system_architecture.md`, `07_single_machine_mvp_plan.md` | retained |
| `VNNI` | `03_system_architecture.md` | retained |
| `SSM`, `Mamba`, `Jamba` | `03_system_architecture.md`, `04_logical_flow.md` | adapted for long-context design discussion |
| `StreamingLLM` | `03_system_architecture.md` | retained as context strategy reference |
| `Attention Sinks` | `03_system_architecture.md` | retained as context strategy reference |
| `Graph RAG`, `GraphRAG` | `03_system_architecture.md`, `04_logical_flow.md` | retained |
| `RAG` | `PRD_Dragon_R1.md`, `03_system_architecture.md` | retained |
| `LoRA`, `MoL` | `03_system_architecture.md`, `06_model_fusion_engines.md` | retained as specialization mechanism |
| `MoE`, `Switch Transformer`, `GLaM`, `Mixtral` | `03_system_architecture.md`, `06_model_fusion_engines.md` | retained as design references |
| `Mixture of Depths`, `MoD` | `03_system_architecture.md` | retained as optional efficiency layer |
| `Speculative Decoding`, `Lookahead Decoding` | `03_system_architecture.md`, `07_single_machine_mvp_plan.md` | retained |
| `MCTS`, `Test-Time Compute` | `03_system_architecture.md`, `06_model_fusion_engines.md` | adapted for safe reasoning and simulation ranking |
| `GBNF`, `Outlines`, `XGrammar` | `03_system_architecture.md`, `06_model_fusion_engines.md` | retained for schema-constrained generation |
| `Cognitive Caching` | `03_system_architecture.md` | adapted as reusable reasoning cache |
| `FAISS`, `ChromaDB`, `Qdrant`, `Neo4j`, `Mem0`, `Redis` | `03_system_architecture.md`, `07_single_machine_mvp_plan.md`, `08_scaling_plan.md` | retained with primary/secondary choices |
| `ZRAM`, `Bcache`, `mmap` | `07_single_machine_mvp_plan.md`, `08_scaling_plan.md` | retained as systems-level references |
| `QuIP#`, `GLA`, `BTC-LLM`, `HQQ`, `GPTQ`, `AWQ`, `QLoRA`, `SmoothQuant`, `DeepCompressor`, `NanoQuant` | `03_system_architecture.md`, `06_model_fusion_engines.md` | preserved as research appendix and future optimization candidates |
| `Hyperdimensional Computing`, `HDC` | `03_system_architecture.md` | preserved as research-only |
| `Spiking Neural Networks`, `SNN` | `03_system_architecture.md` | preserved as research-only |
| `Quantum Logic Simulation` | `03_system_architecture.md` | preserved as research-only |
| `Meta-Plasticity` | `03_system_architecture.md`, `05_agents_design.md` | adapted as online optimization policy, not self-modifying exploit logic |
| `Process Reward Model`, `PRM` | `06_model_fusion_engines.md` | retained as evaluation method |
| `Tree-of-Thoughts`, `Graph-of-Thoughts` | `06_model_fusion_engines.md` | retained for reasoning orchestration |
| `Parallel Processing`, `Hybrid Intelligence`, `Multi-Agent Systems`, `Orchestration` | all design docs | retained |
| `100M Token`, `1M context`, `200K context`, `128K context` | `PRD_Dragon_R1.md`, `03_system_architecture.md`, `06_model_fusion_engines.md` | retained with realism note that Dragon R1 uses virtualized context, not a universal literal resident window |

## Locked unsafe source terms preserved but not implemented

These terms were present in the source set and are preserved verbatim here for traceability, but they are explicitly excluded from implementation:

- `Exploit Agent`
- `Payloads`
- `Payload Optimization Specialist`
- `Attack Innovator`
- `Lateral Movement Expert`
- `Privilege Escalation Expert`
- `Wireless Network Exploitation`
- `Social Engineering Agent`
- `Stealth`
- `Anti-Detection`
- `Emergency Self-Destruct`
- `Self-Healing Code`
- `Polymorphic Evolution`
- `Tor`
- `VLESS`
- `Reality`
- `Zero-days`
- `Buffer Overflow`
- `BlackArch Integration`
- `Autonomous Red-Teaming Engine`
- `Exploit Generation and Payload Creation`
- `Multi-Stage Attack Orchestration`
- `Memory Zeroing` when used as anti-forensics

Their mapped safe treatment appears in `02_safety_scope_and_guardrails.md`.

## Current-model verification note

Because the source set mixes aspirational and inconsistent Claude naming, current official references were checked before finalizing the benchmark language:

- Anthropic model pages and system-card listings indicate `Claude Sonnet 4.6` and `Claude Opus 4.6` were released in February 2026, and `Claude Opus 4.7` was announced on April 16, 2026.

These official references are used only to ground comparison language and dates. They do not change the locked source-term registry above.
