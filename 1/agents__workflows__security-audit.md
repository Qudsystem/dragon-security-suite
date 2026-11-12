---
description: "Run a focused security review from scope definition to remediation validation."
---

# /security-audit — Security Audit Workflow

Run a focused security review from scope definition to remediation validation.

**Related bundles:** `Security Engineer`, `Security Developer`, `Observability & Monitoring`

### Prerequisites
- Explicit authorization for testing.
- In-scope targets documented.
- Logging and environment details available.

### Steps
1. **Define scope and threat model**
   - **Goal:** Identify assets, trust boundaries, and attack paths.
   - **Skills:** `@ethical-hacking-methodology`, `@007` (Mapping phase)
2. **Review auth and access control**
   - **Goal:** Detect account takeover and authorization flaws.
   - **Skills:** `@007` (Threat Modeling phase)
3. **Assess API and input security**
   - **Goal:** Uncover high-impact API and injection vulnerabilities.
   - **Skills:** `@fastapi-pro` (Security section)
4. **Harden and verify**
   - **Goal:** Convert findings into fixes and verify evidence.
   - **Skills:** `@analyze-project`, `@007` (Veredito phase)
