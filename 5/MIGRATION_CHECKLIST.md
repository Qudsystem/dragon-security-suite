<!-- Original Path: C:\Users\Hossam Galal\Desktop\RED\MIGRATION_CHECKLIST.md -->

# Migration Checklist - قائمة المراجعة لعملية التحويل (Unified UI Consolidation)

يحتوي هذا الملف على قائمة مسار التنفيذ ثنائية اللغة لدعم المراجعة والمتابعة للمراحل الخمس لتأسيس لوحة تحكم Dragon-R1.
*This file contains the bilingual execution path checklist to support auditing and tracking of the 5 phases for building the Dragon-R1 dashboard.*

---

### Phase 0: Preparation (Days 1-5) | المرحلة 0: التحضير (الأيام 1-5)
- [x] Extract Feature Inventory (`FEATURE_INVENTORY_DETAILED.md`)
      *(استخراج جرد شامل للميزات)*
- [x] Generate Dependency Graph (`DEPENDENCY_GRAPH.json`)
      *(توليد الرسم البياني للاعتمادات البرمجية)*
- [x] Validate initial Next.js / React setups
      *(التحقق من الإعدادات الأولية لمشاريع Next.js و React)*
- [x] Create Rollback Point Script (`scripts/create_rollback_point.sh`)
      *(إنشاء سكربت لإنشاء نقاط الاستعادة الفورية)*
- [x] Write Technical Design (`design.md`, `requirements.md`)
      *(كتابة ملفات التصميم التقني والمتطلبات الأساسية)*

### Phase 1: Web UI Migration (Days 6-12) | المرحلة 1: ترحيل واجهة الويب (الأيام 6-12)
- [x] Initialize Next.js App Router for `dragon-ui`
      *(تهيئة منظومة Next.js App Router الأساسية للواجهة الموحدة)*
- [x] Port `Cerberus/webapp` components (15 components)
      *(نقل 15 مكونًا من واجهة Cerberus/webapp)*
- [x] Translate `Pentagi frontend` Vite/React patterns to Next.js (54 components)
      *(ترجمة 54 مكونًا وأنماط الواجهة من Pentagi إلى نظام Next.js)*
- [x] Consolidate UI Framework (shadcn/ui, Tailwind)
      *(توحيد إطارات واجهة المستخدم واعتماد shadcn/ui و Tailwind)*
- [x] Build Component Adapter logic for routing translation
      *(بناء منطق مهايئ المكونات "Component Adapter" لترجمة توجيه المسارات)*

### Phase 2: AI Service Integration (Days 13-19) | المرحلة 2: تكامل خدمات الذكاء الاصطناعي (الأيام 13-19)
- [x] Setup unified API Gateway (FastAPI)
      *(إعداد بوابة API الموحدة)*
- [x] Connect DeepCode Service (Port 8100) via REST
      *(ربط خدمة DeepCode عبر REST)*
- [x] Connect CAI Terminal Service (Port 8200) via WebSocket
      *(ربط خدمة الأوامر الطرفية CAI عبر WebSocket)*
- [x] Connect Mem0 API Service (Port 8888) via REST
      *(ربط خدمة الذاكرة Mem0 عبر REST)*
- [x] Connect Nanochat Service (Port 8501) via WebSocket
      *(ربط خدمة المحادثة Nanochat عبر WebSocket)*

### Phase 3: UX Unification (Days 20-23) | المرحلة 3: توحيد تجربة المستخدم (الأيام 20-23)
- [ ] Apply Unified Theme / CSS Variables
      *(تطبيق السمات الموحدة ومتغيرات CSS النظامية)*
- [ ] Normalize Loading States & Error Boundaries
      *(توحيد حالات التحميل وحواجز التقاط الأخطاء)*
- [ ] Implement Circuit Breaker fallbacks visually
      *(تنفيذ الاستجابات المرئية (الواجهات البديلة) لقاطع الدائرة عند سقوط الخدمة)*
- [ ] Performance Optimization (Lighthouse > 85%)
      *(تحسين الأداء لضمان وصول نتيجة التقييم فوق 85%)*

### Phase 4: Testing & Deprecation (Days 24-30) | المرحلة 4: الاختبار والإلغاء (الأيام 24-30)
- [ ] Execute End-to-End (E2E) Test Suites
      *(تنفيذ حزم الاختبارات الشاملة للأنظمة)*
- [ ] Validate Backend connections
      *(التحقق التام من سلاسة استجابات الواجهات الخلفية)*
- [ ] Setup `migration_validator.js` hooks in CI/CD
      *(نشر خطافات التحقق في خطوط النشر المستمر)*
- [ ] Enact 30-day Grace Period for old UIs
      *(فرض فترة سماح مدتها 30 يوماً للواجهات القديمة قبل طمسها)*
- [ ] Deprecate and isolate old Docker Compose endpoints
      *(إلغاء وعزل نقاط الـ Docker Compose القديمة المنتهية الصلاحية)*
