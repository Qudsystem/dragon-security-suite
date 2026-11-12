# Safety Scope And Guardrails

## Operating principle

Dragon R1 in this package is a lawful security validation system, not an offensive automation stack.

## Allowed

- ingesting written authorization and scope
- importing owner-provided asset inventories
- importing authorized scanner outputs and logs
- correlating evidence to `CVE`, `CWE`, `CAPEC`, and control catalogs
- simulating attack paths in graph form
- validating defenses in lab-contained mirrors and intentionally prepared environments
- generating remediation guidance, risk narratives, and executive reports

## Not allowed

- live exploit execution against real customer systems
- payload creation or mutation
- stealth or evasion tuning
- persistence
- lateral movement automation
- privilege escalation playbooks
- credential harvesting
- phishing or social engineering automation
- autonomous propagation
- anti-forensics or log tampering

## Source term translation map

| Source term | Safe treatment in this package |
| --- | --- |
| `Recon Agent` | retained as authorized asset and evidence discovery agent |
| `Analysis Agent` | retained |
| `Simulation Agent` | retained as simulation only |
| `Exploit Node` | replaced by `Simulation Node` and `Validation Node` |
| `Execution Node` | replaced by `Lab Validation Node` |
| `Judge Node` | retained as safety and evidence judge |
| `Payload` | replaced by `simulation trace`, `control check`, or `test artifact` |
| `Exploit Skill` | replaced by `validated control-check recipe` |
| `Attack Path` | retained only as a graph-modeling abstraction |
| `BlackArch Container` | replaced by `sandboxed validation lab` |
| `Stealth` and `Anti-Detection` | excluded |
| `Memory Zeroing` and `Self-Destruct` | excluded except secure deletion of temporary lab artifacts under normal compliance controls |

## Human approval gates

The platform must pause for human approval before:

- importing a new remote engine into a regulated environment
- running lab validations that change system state
- publishing executive-facing findings above a configured severity threshold
- accepting prompt or routing changes proposed by the `Optimization Agent`

## Evidence rules

- every finding must cite evidence
- every high-severity finding must map to at least one source artifact
- every report must include scope boundaries and excluded tests
- every simulation trace must be marked as simulated, lab-validated, or evidence-only

## Windows and Linux sandbox policy

- Linux path: `bwrap` or equivalent process isolation for lab-contained validation only
- Windows path: `Job Objects`, `AppContainer`, `Hyper-V`, `WSL2`, or equivalent sandbox abstraction
- The application must not assume Linux-only primitives on Windows without an adapter layer

## Model safety policy

- every engine output is classified before use
- unsafe recommendations are quarantined
- the `Intelligence Fusion Agent` cannot override the `Policy Agent`
- the `Optimization Agent` can tune routing and prompts, but cannot relax safety policy on its own
