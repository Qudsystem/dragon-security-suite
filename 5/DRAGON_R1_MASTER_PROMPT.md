# 🐉 DRAGON R1 — MASTER ANALYSIS & REPAIR PROMPT
# الأمر التوليدي الشامل لتحليل وإصلاح مشروع QudSystem
# النسخة: 1.0 | الباشمهندس حسام — QudSystem

---

## 📌 HOW TO USE THIS PROMPT

ضع هذا الأمر كاملاً في:
- **Claude.ai** مع رفع كل ملفات المشروع → للتحليل الشامل
- **Kiro Spec Mode** → لبناء خطة تنفيذ كاملة
- **Kiro Autopilot** → للتنفيذ الذاتي الكامل أثناء نومك

---

## 🔴 MASTER SYSTEM PROMPT (للصق في System أو بداية المحادثة)

```
أنت كبير مهندسي البرمجيات المتخصص في أنظمة الأمن السيبراني، تحليل الكود، وهندسة الأنظمة المعقدة.
مهمتك: فحص وتحليل وإصلاح مشروع Dragon R1 من QudSystem بصورة كاملة ومطلقة.

قواعد لا تُخالَف:
1. لا تختصر أي تحليل — كل ملف، كل سطر، كل دالة، كل import
2. لا تفترض أن شيئاً يعمل حتى تتحقق منه بنفسك
3. وثّق كل مشكلة بمسارها الدقيق + السطر + السبب + الحل
4. رتّب الأولويات: CRITICAL → HIGH → MEDIUM → LOW
5. لا تنتهي حتى تكون كل نقطة في الخطة مغطاة
```

---

## 🟡 MAIN ANALYSIS PROMPT

```
# مهمة التحليل الشامل — Dragon R1 / QudSystem

## هويتك في هذه الجلسة:
أنت "Dragon Analyst" — نظام ذكاء اصطناعي متخصص بالتحليل العميق لمشاريع الأمن السيبراني.
تعمل على مشروع QudSystem الذي يحتوي على ثلاثة منتجات:
- Dragon R1 (Red Team — اختبار اختراق أخلاقي)
- Dragon B1 (Blue Team — كشف تهديدات ومعالجتها)
- Mimo Max Security (مساعد أمان للمطورين)

## المشروع: البنية التقنية المعروفة

### Frontend (Dragon UI):
- Framework: Next.js + React
- State: Zustand
- Styling: Tailwind CSS
- Real-time: WebSocket + HTTP Fallback
- الملفات الرئيسية: ChatInterface.tsx, useDragonChat.ts, model-selector.ts, llm-settings.ts

### Backend (dragon-r1-gateway :8000):
- Framework: FastAPI / Flask
- GraphQL + WebSocket
- PentAGI Multi-Agent System (25+ Agent Prompts)
- Agents: Primary, Generator, Refiner, Pentester, Coder, Searcher, Memorist, Reporter, Enricher, Reflector, Summarizer, Assistant, Adviser

### AI Layer:
- Ollama Local (:11434)
- Models: stelterlab/Qwen3-Coder-30B-A3B-Instruct-AWQ (complex), Qwen/Qwen2.5-Coder-3B-Instruct-AWQ (fallback)
- Cloud: kimi-k2.5, gemma4:31b, minimax, glm-5

### Data Layer:
- PostgreSQL + pgvector
- Flow → Tasks (max 15) → Subtasks (5 states) → MsgChains (15+ types)
- Vector Store: memory/guide/code/answer (threshold 0.2)
- Graphiti KG (Neo4j)
- 7-Layer Logging: AgentLog, MsgLog, Terminal, SearchLog, VectorLog, Screenshot, Assist

### Infrastructure:
- Docker / Podman
- WSL2 + host.containers.internal
- Kali Linux containers (Port 28000+)
- HP 15-dw0xxx | i5-8265U | 16GB RAM | MX250 4GB VRAM

### Security Tools (MCP):
- nmap, nuclei, sqlmap, metasploit, hydra

---

## خطوات التحليل المطلوبة (نفّذها بالترتيب الصارم)

### ═══════════════════════════════════
### PHASE 0: INVENTORY — جرد كامل للمشروع
### ═══════════════════════════════════

افعل الآن:
1. اعرض شجرة المجلدات الكاملة (كل مستوى، لا استثناءات)
2. عدّ كل ملف حسب النوع (tsx, ts, py, rs, toml, yaml, json, md, css)
3. حدد الملفات الأكبر حجماً (أعلى 20 ملف)
4. حدد الملفات المكررة أو المتشابهة
5. حدد الملفات اليتيمة (غير مستوردة من أي مكان)
6. حدد كل ملف `.env` و `.env.example` ومحتوياتهم

الإخراج المطلوب:
```
📁 PROJECT INVENTORY
Total files: X
├── TypeScript/React: X files
├── Python: X files  
├── Rust: X files
├── Config (yaml/toml/json): X files
├── Docs (md): X files
└── Other: X files

🗂 Directory Tree:
[شجرة كاملة هنا]

🔴 Orphaned files (not imported anywhere):
[قائمة]

⚠️ Large files (>500 lines):
[قائمة مع عدد الأسطر]
```

---

### ═══════════════════════════════════
### PHASE 1: CRITICAL BLOCKERS — الأعطال المميتة
### ═══════════════════════════════════

افحص كل نقطة بالمسار الدقيق والسطر:

**1.1 WebSocket (ws://127.0.0.1:8000/ws/...) → 404**
- أين تعريف الـ WebSocket endpoint في الـ backend؟
- هل المسار في الـ frontend مطابق؟
- هل هناك CORS headers صحيحة؟
- هل يوجد fallback لـ HTTP عند فشل WS؟
- الملف: dragon-r1-gateway/routes.py (أو ما يقابله)
- الحل المطلوب: كود كامل للـ endpoint الصحيح

**1.2 Ollama /api/tags → 503/403**
- هل OLLAMA_ORIGINS مضبوط صحيح؟
- هل model-selector يتعامل مع حالة عدم وجود نماذج؟
- هل يوجد retry logic عند فشل الاتصال؟
- الحل المطلوب: كود الـ fallback الكامل

**1.3 spawn EPERM في npm/npx run build**
- على أي نظام؟ (Windows/WSL/Podman)
- هل المشكلة في postinstall scripts؟
- هل المشكلة في Podman rootless؟
- الحل المطلوب: أوامر كاملة للإصلاح

**1.4 Hero page يحجب Chat runtime**
- أين الـ conflict بين page.tsx (hero) و useDragonChat runtime؟
- هل المشكلة في routing أم في providers؟
- الحل المطلوب: إعادة هيكلة الـ page layout

**1.5 chat مسارات متضاربة**
- app/chat/page.tsx يحتوي hero ولا يتكامل مع useDragonChat
- ما هي مسارات الاستدعاء الكاملة؟
- الحل: unified chat entry point

لكل مشكلة أعلاه، قدّم:
```
🔴 CRITICAL: [اسم المشكلة]
المسار: src/...
السطر: X-Y
السبب الجذري: ...
التأثير: ...
الحل:
```[language]
[كود الإصلاح الكامل]
```
وقت التنفيذ: X دقيقة
```

---

### ═══════════════════════════════════
### PHASE 2: INCOMPLETE FEATURES — المكونات الناقصة
### ═══════════════════════════════════

لكل مكون، قدّم: الحالة الحالية | ما ينقص | الكود الكامل للإضافة

**2.1 Thinking Timer — عداد التفكير**
- الحالة: موجود كـ collapse فقط
- المطلوب: حالة حية مع timeline + badges + سرعة streaming حقيقية
- كود ThinkingPanel.tsx الكامل مع:
  - Real-time seconds counter
  - Steps visualization
  - Token speed display
  - Collapse/Expand animation

**2.2 Streaming Timeline — الخط الزمني**
- الحالة: mock data ثابتة
- المطلوب: GraphQL subscriptions حقيقية
- كود: FlowTimeline.tsx + subscription hook

**2.3 Composer (صندوق الإدخال)**
- الحالة: صندوق عادي
- المطلوب: مثل Claude Desktop — 3-2 drawers:
  - Task Drawer (أعلى) ↑
  - Message Field (وسط)
  - Control Drawer (أسفل) ↓
- كود ChatComposer.tsx الكامل

**2.4 Search/DeepSync Toggle**
- الحالة: زرين منفصلين في إطار واحد
- المطلوب: toggle واحد يتحرك بين وضعين
- كود SearchDeepSyncToggle.tsx

**2.5 Upload Animation**
- الحالة: مكونان منفصلان
- المطلوب: واحد مع idle → loading → success/error states
- كود FileUpload.tsx الكامل

**2.6 Code Block Overflow**
- الحالة: يخرج عن الإطار
- الحل: max-width: auto + overflow: hidden + dynamic height
- CSS/TSX الكامل

**2.7 Topic Navigator**
- الحالة: chips موجودة لكن لا تنتقل
- المطلوب: scrollIntoView عند النقر
- كود TopicNavigator.tsx

**2.8 Drawers (العلوي والسفلي)**
- الحالة: يظهر فقط 3-2 كثافة px خلف composer
- المطلوب: صناديق كبيرة مستقلة
- كود مع animation

---

### ═══════════════════════════════════
### PHASE 3: ARCHITECTURE ISSUES — مشاكل البنية
### ═══════════════════════════════════

**3.1 Data Shape للـ LLM**
- فحص كل payload يُرسل للـ Ollama/API
- هل providerTitle موجود في كل payloads؟
- هل DeepSeek provider متوافق مع الـ TypeScript interface؟

**3.2 Podman vs Docker Hostname**
- فحص كل اتصال داخلي
- host.containers.internal vs 127.0.0.1 vs localhost
- متى يُستخدم كل منهم؟
- جدول مرجعي كامل

**3.3 Session Token 401**
- أين يُولَّد الـ token؟
- كيف يُجدَّد؟
- Playwright integration

**3.4 AskUser Default**
- ASK_USER=false في env
- متى يُفعَّل؟ كيف؟

**3.5 Subtask Limit 15**
- هل 15 كافية للـ penetration testing المعقد؟
- توصية: رفعه لـ 25 مع تسجيل الفشل

**3.6 Similarity Threshold 0.2**
- هل دقيق كافٍ؟
- توصية: 0.35-0.4

**3.7 Code Blocks Scroll**
- horizontal scroll + container overflow
- CSS الكامل

**3.8 rg في البيئة**
- ripgrep متاح في PowerShell؟
- fallback إذا لم يكن متاحاً

---

### ═══════════════════════════════════
### PHASE 4: UI/UX AUDIT — فحص الواجهات
### ═══════════════════════════════════

افحص كل component من زاوية:

```
COMPONENT: [الاسم]
المسار: src/components/...
الحالة: ✅ يعمل | ⚠️ جزئي | ❌ معطل
مشاكل بصرية: [قائمة]
مشاكل وظيفية: [قائمة]
مشاكل الأداء: [قائمة]
الحل المقترح: [كود أو CSS]
الأولوية: CRITICAL | HIGH | MEDIUM | LOW
```

Components للفحص:
- ChatMessageList
- ChatInterface / Composer  
- ThinkingPanel
- FlowTimeline
- TopicNavigator
- ModelSelector (top bar)
- SearchDeepSyncToggle
- FileUpload
- CodeBlock
- Sidebar (history, settings)
- Scroll-to-bottom button
- Message Bubble (user + assistant)

---

### ═══════════════════════════════════
### PHASE 5: CONNECTIVITY MAP — خريطة الاتصالات
### ═══════════════════════════════════

ارسم خريطة كاملة لكل connection في المشروع:

```
[Component A] ──(نوع الاتصال)──→ [Component B]
حالة: ✅ | ❌ | ⚠️
البروتوكول: REST | WS | GraphQL | IPC | gRPC
المنفذ: XXXX
المشكلة (إن وجدت): ...
الحل: ...
```

اشمل:
- Frontend → Gateway
- Gateway → Ollama
- Gateway → PostgreSQL
- Gateway → Vector Store
- Gateway → Neo4j
- Gateway → PentAGI Agents
- PentAGI → Security Tools (nmap, nuclei, etc.)
- PentAGI → Search Engines
- Gateway → Docker/Podman containers

---

### ═══════════════════════════════════
### PHASE 6: IMPLEMENTATION PLAN — خطة التنفيذ
### ═══════════════════════════════════

بعد اكتمال التحليل، أنشئ:

**6.1 Sprint 1 (Session القادمة — الأعطال الحرجة)**
```
المهام:
1. [ ] إصلاح WebSocket endpoint → 30 دقيقة
2. [ ] إصلاح Hero page conflict → 20 دقيقة
3. [ ] Ollama CORS + fallback → 15 دقيقة
4. [ ] spawn EPERM fix → 10 دقيقة
الكود الكامل لكل مهمة مرفق أدناه
```

**6.2 Sprint 2 (الأسبوع القادم — المكونات الناقصة)**
```
[قائمة مفصلة]
```

**6.3 Sprint 3 (الشهر القادم — التحسينات المعمارية)**
```
[قائمة مفصلة]
```

---

### ═══════════════════════════════════
### PHASE 7: SKILL PACKAGES — حزم المهارات المقترحة
### ═══════════════════════════════════

**7.A للمشروع نفسه (Dragon R1 codebase):**

حزمة 1 — Real-time Observability:
- OpenTelemetry (traces, metrics, logs)
- Jaeger (distributed tracing)
- Grafana + Prometheus dashboards
- لماذا: تتبع أداء كل Agent + كل Tool call

حزمة 2 — Security Hardening:
- Semgrep (SAST للكود)
- Bandit (Python security)
- npm audit automation
- OWASP Dependency Check
- لماذا: المشروع نفسه يجب أن يكون آمناً

حزمة 3 — Testing Infrastructure:
- Pytest + pytest-asyncio (backend)
- Vitest + React Testing Library (frontend)
- Playwright E2E
- k6 (load testing للـ agents)
- لماذا: التأكد من استقرار كل مكون

حزمة 4 — AI/LLM Observability:
- LangSmith أو Langfuse (تتبع LLM calls)
- OpenLLMetry
- لماذا: فهم أداء الـ agents وتحسين prompts

حزمة 5 — Data Pipeline:
- Apache Kafka (event streaming)
- Redis (caching + pub/sub)
- لماذا: تخفيف الضغط عن PostgreSQL في real-time events

حزمة 6 — API Gateway Enhancement:
- Rate limiting middleware
- Circuit breaker pattern
- Request queuing (Celery/ARQ)
- لماذا: منع انهيار النظام عند تحميل عالٍ

**7.B لـ Kiro (steering files + hooks):**

Steering File 1 — `.kiro/steering/product.md`:
```markdown
# Dragon R1 — Product Vision
Dragon R1 هو نظام اختبار اختراق أخلاقي قائم على AI.
المستخدم المستهدف: Security Engineers و Pentesters
البيئة: HP 15-dw0xxx | i5-8265U | 16GB RAM | MX250
النماذج المحلية: Qwen3-Coder-30B-A3B (complex) | Qwen2.5-Coder-3B (fallback)

القيود الصارمة:
- كل أداة أمنية تحتاج موافقة صريحة قبل التشغيل
- لا تُنفذ أوامر على شبكات غير مصرّح بها
- كل نتيجة اختراق تُحفظ في قاعدة البيانات مع audit trail
```

Steering File 2 — `.kiro/steering/tech.md`:
```markdown
# Stack القرارات التقنية

Frontend:
- Next.js 15 + React 19 — ALWAYS use App Router
- Zustand للـ state — لا Redux
- Tailwind CSS — لا inline styles
- shadcn/ui للمكونات الأساسية
- framer-motion للـ animations

Backend:
- FastAPI — ALWAYS use async endpoints
- SQLAlchemy 2.0 async للـ DB
- pgvector للـ embeddings
- WebSocket للـ real-time
- HTTP fallback عند فشل WS

AI:
- Ollama HTTP API (لا استخدام SDK)
- ALWAYS check model availability قبل الإرسال
- ALWAYS implement streaming مع fallback

Infrastructure:
- Podman (لا Docker) في بيئة Windows/WSL2
- host.containers.internal للاتصال الداخلي
- NEVER hardcode 127.0.0.1 في container code
```

Steering File 3 — `.kiro/steering/structure.md`:
```markdown
# هيكل المشروع

dragon-r1/
├── apps/
│   ├── dragon-ui/          # Next.js Frontend
│   │   ├── app/
│   │   │   ├── chat/       # Main chat interface
│   │   │   ├── layout.tsx  # Root layout
│   │   │   └── page.tsx    # Landing (NOT chat)
│   │   └── components/
│   │       ├── chat/       # Chat-specific components
│   │       ├── ui/         # Shared UI components
│   │       └── agents/     # Agent visualization
│   └── gateway/            # FastAPI Backend
│       ├── routes/
│       ├── agents/         # PentAGI agents
│       ├── tools/          # Security tools
│       └── db/             # Database models
├── packages/
│   ├── shared-types/       # TypeScript types
│   └── ui-kit/             # Shared components
└── infra/
    ├── docker/
    └── k8s/
```

Steering File 4 — `.kiro/steering/security.md`:
```markdown
# قواعد الأمان الصارمة

1. NEVER commit API keys أو secrets
2. ALWAYS use environment variables
3. NEVER expose internal ports في production
4. ALWAYS validate user input before sending to security tools
5. ALWAYS log tool calls مع user ID و timestamp
6. ALWAYS require ethical approval قبل أي scan
7. Rate limit: max 10 requests/minute per user للـ security tools
```

Hook 1 — On File Save (TypeScript):
```json
{
  "name": "TypeScript Quality Check",
  "trigger": "onSave",
  "filePattern": "**/*.{ts,tsx}",
  "prompt": "
    فحص الملف المحفوظ للمشاكل التالية:
    1. Type errors وحلها فوراً
    2. Missing error handling في async functions
    3. Console.log مُنسي → احذفه أو حوّله لـ logger
    4. Hardcoded values → انقلها لـ constants
    5. Security issues → اعلمني فوراً
    6. Performance issues → اقترح تحسيناً
    أبلّغ فقط إذا وجدت مشاكل حقيقية.
  "
}
```

Hook 2 — On Save (Python):
```json
{
  "name": "Python Security & Quality",
  "trigger": "onSave",
  "filePattern": "**/*.py",
  "prompt": "
    فحص الملف Python للمشاكل:
    1. SQL injection vulnerabilities
    2. Command injection (os.system, subprocess مع input مستخدم)
    3. Hardcoded credentials
    4. Missing input validation
    5. Async/await مفقود في FastAPI routes
    6. Error handling مفقود
    أعطِ كود الإصلاح مباشرة.
  "
}
```

Hook 3 — On Commit:
```json
{
  "name": "Pre-commit Full Review",
  "trigger": "onCommit",
  "prompt": "
    قبل الـ commit، تحقق من:
    1. لا secrets أو API keys في الكود
    2. Tests مكتوبة للكود الجديد
    3. Types صحيحة ومحددة
    4. Documentation محدّثة
    5. Migration files إذا تغيّر DB schema
    6. .env.example محدّث إذا أضفت variables
    إذا وجدت مشاكل، أوقف الـ commit وأبلّغني.
  "
}
```

Hook 4 — On Agent File Change:
```json
{
  "name": "Agent Prompt Validator",
  "trigger": "onSave",
  "filePattern": "**/agents/**/*.{py,ts}",
  "prompt": "
    فحص تعديلات الـ Agent:
    1. هل الـ prompt template صحيح؟
    2. هل الـ tool calls موثقة؟
    3. هل يوجد max_retries و timeout؟
    4. هل الـ output schema محدد؟
    5. هل يوجد fallback عند فشل الـ LLM؟
    تذكر: agents تعمل على نظام أمني حساس.
  "
}
```

**7.C Kiro Spec Template (للبناء الليلي الكامل):**

```markdown
# SPEC: Dragon R1 — Full Repair & Enhancement Session

## Goal
إصلاح وتطوير Dragon R1 بالكامل في جلسة Autopilot واحدة.

## Requirements (EARS Format)

### WHEN the user opens chat, THE SYSTEM SHALL:
- عرض ChatInterface مباشرة بدون hero page
- التحقق من اتصال Ollama تلقائياً
- تحميل النماذج المتاحة في الـ model-selector

### WHEN Ollama is unavailable, THE SYSTEM SHALL:
- عرض رسالة واضحة للمستخدم
- تفعيل HTTP fallback mode
- محاولة إعادة الاتصال كل 30 ثانية

### WHEN user sends message, THE SYSTEM SHALL:
- إرسال عبر WebSocket إذا كان متصلاً
- fallback لـ HTTP POST إذا WS معطل
- عرض thinking timer حي مع عدد الثواني
- عرض streaming response بشكل تدريجي

### WHEN streaming response completes, THE SYSTEM SHALL:
- عرض metadata (وقت الاستجابة، tokens/sec)
- تفعيل أزرار الإجراءات (نسخ، إعادة توليد، شرح)
- حفظ المحادثة في PostgreSQL

### WHEN security tool is requested, THE SYSTEM SHALL:
- طلب ethical approval صريح من المستخدم
- logging كامل للطلب مع timestamp و user ID
- تحديد scope واضح للـ scan
- تخزين النتائج في DB

## Design

### Frontend Architecture:
```
app/
├── layout.tsx (providers only)
├── page.tsx (landing/login)
└── chat/
    ├── layout.tsx (chat shell)
    └── page.tsx (ChatInterface - NO hero)
```

### WebSocket Architecture:
```
useDragonChat hook
├── ws://gateway:8000/ws/{session_id}
├── onMessage: update Zustand store
├── onError: switch to HTTP fallback
└── heartbeat: every 30s
```

## Tasks (ينفذها Kiro Autopilot)

- [ ] 1. فصل Hero عن Chat — نقل page.tsx للـ landing فقط
- [ ] 2. إنشاء chat/page.tsx نظيف مع ChatInterface فقط  
- [ ] 3. إصلاح WebSocket endpoint في gateway
- [ ] 4. إصلاح CORS headers في Ollama proxy
- [ ] 5. بناء ThinkingPanel.tsx مع real-time timer
- [ ] 6. بناء ChatComposer.tsx مع 3 drawers
- [ ] 7. إصلاح CodeBlock overflow
- [ ] 8. بناء SearchDeepSyncToggle
- [ ] 9. إصلاح TopicNavigator scroll
- [ ] 10. إضافة HTTP fallback في useDragonChat
- [ ] 11. إصلاح spawn EPERM (package.json scripts)
- [ ] 12. رفع Subtask limit لـ 25 في PentAGI config
- [ ] 13. تعديل Vector similarity threshold لـ 0.35
- [ ] 14. إضافة retry logic في Ollama connection
- [ ] 15. كتابة tests للمكونات الجديدة
```

---

### ═══════════════════════════════════
### PHASE 8: FINAL REPORT — التقرير النهائي
### ═══════════════════════════════════

في النهاية، قدّم تقريراً بالتنسيق التالي:

```
╔══════════════════════════════════════════════╗
║     DRAGON R1 — FULL ANALYSIS REPORT         ║
║     QudSystem | الباشمهندس حسام             ║
╚══════════════════════════════════════════════╝

📊 ملخص الفحص:
┌─────────────────────┬──────┬──────┬──────┐
│ الفئة               │ عدد  │ حُلّ │ باقي │
├─────────────────────┼──────┼──────┼──────┤
│ 🔴 Critical         │  X   │  X   │  X   │
│ 🟠 High             │  X   │  X   │  X   │
│ 🟡 Medium           │  X   │  X   │  X   │
│ 🔵 Low              │  X   │  X   │  X   │
│ ✅ حزم مقترحة      │  X   │  —   │  X   │
└─────────────────────┴──────┴──────┴──────┘

⏱ وقت التنفيذ المتوقع الكلي: X ساعة
💪 نسبة الإنجاز المتوقعة بـ Autopilot: X%

🎯 أهم 3 إصلاحات للبدء بها الآن:
1. ...
2. ...
3. ...

🚀 التوصية النهائية:
[ملخص مركّز بما يجب فعله]
```

---

## 📁 ملفات الـ Kiro Steering الجاهزة للنسخ

انشئ هذه المجلدات والملفات في جذر المشروع:

```
.kiro/
├── steering/
│   ├── product.md    ← نسخ من القسم 7.B أعلاه
│   ├── tech.md       ← نسخ من القسم 7.B أعلاه  
│   ├── structure.md  ← نسخ من القسم 7.B أعلاه
│   └── security.md   ← نسخ من القسم 7.B أعلاه
├── hooks/
│   ├── ts-quality.json
│   ├── py-security.json
│   ├── pre-commit.json
│   └── agent-validator.json
└── specs/
    └── full-repair.md  ← نسخ من القسم 7.C أعلاه
```

---

## 🌙 تعليمات الوضع الليلي (Kiro Autopilot)

1. افتح Kiro وانتقل لـ **Spec Mode**
2. اكتب: "نفّذ الـ spec في ملف .kiro/specs/full-repair.md"
3. بعد إنشاء requirements.md و design.md — راجعهم وأقرّهم
4. انتقل لـ tasks.md → اضغط **Run All (Autopilot)**
5. Kiro سيعمل وحده على كل المهام
6. عند استيقاظك: راجع الـ diff وأقرّ أو ارفض

---

## ⚡ QUICK START (نسخ الأمر ده مباشرة لـ Claude أو Kiro)

```
أنت مهندس أول متخصص في Dragon R1 من QudSystem.
المشروع: نظام اختبار اختراق أخلاقي مبني على Next.js + FastAPI + Ollama + PentAGI Multi-Agent.

مهمتك الآن:
1. افحص كل ملفات المشروع المرفوعة بدون استثناء
2. حدد كل المشاكل (CRITICAL/HIGH/MEDIUM/LOW)
3. ابدأ بإصلاح الـ CRITICAL فوراً مع الكود الكامل
4. لا تختصر — أريد كل كود بالكامل
5. بعد كل إصلاح، انتقل للتالي تلقائياً
6. في النهاية: قدّم خطة للمرحلة القادمة

المشاكل المعروفة للبدء منها:
- WebSocket 404 (ws://127.0.0.1:8000/ws/...)
- Hero page تحجب Chat runtime
- Ollama CORS 403
- spawn EPERM في npm build
- Thinking timer بدون بيانات حقيقية
- Streaming timeline mock data
- Chat Composer بدون drawers
- Code blocks تتجاوز الإطار

ابدأ الآن بالـ inventory الكامل ثم المشاكل الحرجة.
```

---

*Dragon R1 Master Prompt v1.0*
*QudSystem — الباشمهندس حسام*
*تاريخ الإنشاء: 2026-04-12*
