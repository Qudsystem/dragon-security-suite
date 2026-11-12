# شجرة مشروع Dragon-R1

## البنية الرئيسية للمشروع

```
Dragon-R1/
│
├── .agents/                          # وكلاء الأتمتة
│   └── workflows/                    # سير عمل الوكلاء
│       ├── saas-mvp.md
│       └── security-audit.md
│
├── .kiro/                            # إعدادات Kiro
│   ├── hooks/                        # خطافات الأحداث
│   ├── settings/                     # الإعدادات
│   ├── specs/                        # مواصفات المشاريع
│   │   ├── autonomous-agent-core/
│   │   ├── autonomous-mode/
│   │   ├── chat-response-no-reply-fix/
│   │   ├── dragon-r1-banner-fix/
│   │   ├── dragon-r1-frontend-security/
│   │   ├── dragon-r1-podman-migration/
│   │   ├── jest-memory-crash-fix/
│   │   ├── kiro-gemini-coordination/
│   │   ├── python-environment-dependency-fix/
│   │   ├── scripts-organization/
│   │   ├── unified-standalone-dragon-system/
│   │   ├── unified-ui-consolidation/
│   │   └── wsl-distribution-support/
│   └── steering/                     # قواعد التوجيه
│
├── Cerberus/                         # المكون الرئيسي للنظام
│   ├── gateway/                      # بوابة Backend (Python/FastAPI)
│   │   ├── adapters/                 # محولات الخدمات الخارجية
│   │   ├── core/                     # المحركات الأساسية (~60 وحدة)
│   │   ├── services/                 # خدمات منطق الأعمال
│   │   ├── tests/                    # اختبارات Backend
│   │   ├── main.py                   # نقطة الدخول الرئيسية
│   │   ├── orchestrator.py           # منسق خط الهجوم
│   │   ├── langgraph_agent.py        # وكيل LangGraph
│   │   ├── models.py                 # نماذج Pydantic
│   │   ├── auth.py                   # مصادقة JWT
│   │   ├── config.py                 # إعدادات مركزية
│   │   └── websocket_enhanced.py     # إدارة WebSocket
│   │
│   ├── dragon-ui/                    # واجهة Frontend (Next.js 16)
│   │   ├── src/
│   │   │   ├── app/                  # صفحات App Router
│   │   │   │   ├── chat/             # واجهة الدردشة الرئيسية
│   │   │   │   ├── dashboard/
│   │   │   │   ├── tasks/
│   │   │   │   ├── reports/
│   │   │   │   ├── graph/
│   │   │   │   ├── admin/
│   │   │   │   ├── red-team/
│   │   │   │   ├── tools/
│   │   │   │   ├── settings/
│   │   │   │   └── projects/
│   │   │   ├── components/           # مكونات UI
│   │   │   │   ├── ui/               # Shadcn/Radix UI
│   │   │   │   ├── chat-interface.tsx
│   │   │   │   ├── ChatInterface.tsx
│   │   │   │   ├── sidebar.tsx
│   │   │   │   ├── navbar.tsx
│   │   │   │   ├── attack-map.tsx
│   │   │   │   ├── terminal.tsx
│   │   │   │   ├── memory-viewer.tsx
│   │   │   │   ├── model-selector.tsx
│   │   │   │   └── chat-message-surface.tsx
│   │   │   └── lib/
│   │   │       ├── store.ts          # Zustand state management
│   │   │       ├── websocket.ts      # DragonWebSocket class
│   │   │       ├── api.ts            # HTTP client
│   │   │       ├── security/         # أمان جانب العميل
│   │   │       └── validation/       # Zod schemas
│   │   ├── tests/                    # اختبارات Frontend
│   │   ├── jest.config.js
│   │   └── package.json
│   │
│   └── webapp/                       # DEPRECATED - لا تستخدم
│
├── dragon-cli/                       # CLI (Go/Bubble Tea TUI)
│   ├── main.go
│   └── ...
│
├── Dragon-R/                         # مشاريع فرعية مرجعية
│   ├── cai-main/                     # إطار عمل CAI
│   ├── pentagi-master/               # تنسيق PentAGI
│   ├── PentestGPT-main/              # PentestGPT
│   ├── DeepCode-main/                # تحليل DeepCode
│   ├── mem0-main/                    # طبقة ذاكرة mem0
│   ├── nanoGPT-master/               # NanoGPT للتدريب
│   └── RAG-Anything-main/            # خط أنابيب RAG
│
├── DragonR1-Archive-2026/            # أرشيف المشاريع القديمة
│   ├── mem0-main/
│   ├── nanoGPT-master/
│   ├── pentagi-master/
│   ├── PentestGPT-main/
│   └── RAG-Anything-main/
│
├── scripts/                          # سكريبتات الأتمتة
│   ├── run-tests.sh
│   ├── run-tests.bat
│   ├── generate-test-report.sh
│   ├── generate-test-report.bat
│   ├── test-summary.py
│   ├── setup-databases.sh
│   └── setup-databases.bat
│
├── reports/                          # تقارير المهام
│   ├── runtime-check/
│   └── task-evidence/
│
├── payloads/                         # حمولات الاختبار
│
├── output/                           # مخرجات التنفيذ
│
├── venv/                             # بيئة Python الافتراضية
│
├── .hypothesis/                      # بيانات اختبار Hypothesis
│
├── .pytest_cache/                    # ذاكرة تخزين pytest
│
├── .playwright-cli/                  # سجلات Playwright
│
├── AGENTS.md                         # دليل الوكلاء
├── README.md                         # الوثائق الرئيسية
├── README-TESTING.md                 # دليل الاختبار
├── START-DRAGON-R1.bat               # بدء النظام (Windows)
├── start-dragon-r1.sh                # بدء النظام (Linux/Mac)
├── run-all-tests.sh                  # تشغيل جميع الاختبارات
├── run-all-tests.bat
└── .gitignore
```

## المكونات الرئيسية

### 1. Backend Gateway (Cerberus/gateway)
- **اللغة**: Python 3.10+
- **الإطار**: FastAPI
- **المنفذ**: 8000
- **الميزات**:
  - مصادقة JWT مع RBAC
  - WebSocket للتحديثات الفورية
  - منسق خط الهجوم (RECON → ENUMERATE → DISCOVER → EXPLOIT → ESCALATE → EXFILTRATE → REPORT)
  - وكيل LangGraph ReAct
  - تكامل مع خدمات خارجية متعددة

### 2. Frontend UI (Cerberus/dragon-ui)
- **الإطار**: Next.js 16 (App Router)
- **المنفذ**: 3000
- **التقنيات**:
  - React 19
  - Zustand (إدارة الحالة)
  - Tailwind CSS 4
  - Shadcn/Radix UI
  - React Flow (تصور الرسوم البيانية)
  - Recharts (الرسوم البيانية)
  - XTerm (الطرفية)
  - Monaco Editor

### 3. CLI (dragon-cli)
- **اللغة**: Go
- **الإطار**: Bubble Tea / Lip Gloss
- **الميزات**:
  - TUI تفاعلي
  - إدارة الخدمات
  - Copilot chat REPL

### 4. قواعد البيانات (Docker)
- **Neo4j**: 7687/7474 (تخزين رسم الهجوم)
- **Redis**: 6379 (التخزين المؤقت)
- **PostgreSQL**: 5432 (البيانات الدائمة)

## بنية الاختبار

### Backend Tests
```
Cerberus/gateway/tests/
├── unit/                   # اختبارات الوحدة
├── integration/            # اختبارات التكامل
├── property/               # اختبارات قائمة على الخصائص (Hypothesis)
└── test_dragon_pipeline.py
```

### Frontend Tests
```
Cerberus/dragon-ui/tests/
├── unit/                   # اختبارات الوحدة
├── integration/            # اختبارات التكامل
└── performance/            # اختبارات الأداء
```

## أوامر مهمة

### بدء النظام الكامل
```bash
# Windows
.\START-DRAGON-R1.bat

# Linux/Mac
./start-dragon-r1.sh
```

### Backend
```bash
cd Cerberus/gateway
python -m venv .venv
.venv\Scripts\activate  # Windows
pip install -r requirements.txt
python main.py
```

### Frontend
```bash
cd Cerberus/dragon-ui
npm install
npm run dev
```

### الاختبارات
```bash
# جميع الاختبارات
./run-all-tests.sh

# Backend فقط
cd Cerberus/gateway
pytest

# Frontend فقط
cd Cerberus/dragon-ui
npm test
```

## ملاحظات معمارية

1. **الفصل بين الاهتمامات**: كل ميزة وحدة مستقلة
2. **البنية الطبقية**: core/domain → application → infrastructure → features
3. **التدهور الرشيق**: جميع الوحدات الاختيارية قابلة للاستيراد مع try/except
4. **نقاط التوسع**: خطافات للميزات المستقبلية
5. **المسؤولية الواحدة**: وحدة واحدة = مسؤولية واحدة
