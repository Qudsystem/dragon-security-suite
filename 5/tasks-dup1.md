# مهام مشروع دمج الواجهات الموحدة | Unified UI Consolidation Tasks

هذه القائمة تتكون من مهام عملية مستمدة مباشرة من متطلبات التصميم والوثيقة التقنية. يجب تحديث هذه الملفات إذا أضيفت تفاصيل جديدة.
This list consists of practical tasks derived directly from the design requirements and technical document. These files must be updated if new details are added.

## 1. التحضيرات العامة | General Preparations

- [x] تطوير سكريبت جرد الميزات `scripts/feature_inventory.py` وإخراج JSON وMarkdown
  - *Develop feature inventory script `scripts/feature_inventory.py` and output JSON and Markdown*
- [x] توليد ملفي `FEATURE_INVENTORY.md` و`DEPENDENCY_GRAPH.json`
  - *Generate `FEATURE_INVENTORY.md` and `DEPENDENCY_GRAPH.json` files*
- [x] إعداد أداة `migration_validator.js` للتحقق الآلي من النقل والتكامل
  - *Setup `migration_validator.js` tool for automated verification of migration and integration*
- [x] إنشاء سكريبت `create_rollback_point.sh` لإنشاء نقطة استعادة كاملة
  - *Create script `create_rollback_point.sh` to generate a complete rollback point*

## 2. التخطيط والمراحل | Planning and Phases

- [x] كتابة خطة مراحلية مفصلة تشمل الفترات الخمسة (التحضير، واجهة الويب، خدمات الذكاء، توحيد UX، الاختبار والإيقاف)
  - *Write a detailed phased plan covering all 5 periods (Preparation, Web UI, AI Services, UX Unification, Testing & Deprecation)*
- [x] تحديد معايير الدخول والخروج لكل مرحلة دمج (Consolidation_Phase) وتقدير الجهد الزمني
  - *Define entry and exit criteria for each Consolidation_Phase and estimate time effort*
- [x] إنشاء وثائق `TIMELINE.md` و`MIGRATION_CHECKLIST.md`
  - *Create `TIMELINE.md` and `MIGRATION_CHECKLIST.md` documents*
- [ ] متابعة الفروق بين التقديرات والوقت الفعلي وتحديث التقارير الأسبوعية
  - *Track differences between estimates and actual time, and update weekly reports*

## 3. المعمارية والهجينة | Hybrid Architecture

- [x] تصميم وتنفيذ معمارية هجينة (Hybrid Architecture) في واجهة dragon-ui بناءً على الرسم التخطيطي وملف design.md
  - *Design and implement Hybrid Architecture in dragon-ui based on the diagram and design.md file*
- [x] توثيق المخططات التخطيطية (Mermaid) والرسوم البيانية المستخدمة في وثيقة التصميم
  - *Document schematic diagrams (Mermaid) and charts used in the design document*
- [x] بناء API Gateway مع Circuit Breaker، Rate Limiter، وطبقة Service Discovery عبر Redis
  - *Build API Gateway with Circuit Breaker, Rate Limiter, and Service Discovery layer via Redis*
- [ ] تنفيذ نظام تسجيل الحالة في Registry وتحديثها كل 30 ثانية مع خوارزمية تراجع (backoff)
  - *Implement health registry system updating every 30 seconds with exponential backoff*
- [x] تعريف واجهات التكوين (`GatewayConfig`, `ServiceConfig`, وغيرها) ومكون تكوين إداري واجهي
  - *Define config interfaces (`GatewayConfig`, `ServiceConfig`, etc.) and an admin config UI component*
- [x] كتابة وثيقة `ARCHITECTURE.md` تتضمن القرارات والمعايير ومخططات التكامل
  - *Write `ARCHITECTURE.md` involving architecture decisions, standards, and integration flows*

## 6. قواعد البيانات والبنية التحتية | Databases and Infrastructure

- [x] إعداد قواعد البيانات الثلاث: Postgres للمعلومات العلائقية، Neo4j للرسوم البيانية، Redis للخدمات والتخزين المؤقت عبر docker-compose
  - *Setup the 3 databases: Postgres (relational), Neo4j (graph), Redis (services/caching) via docker-compose*
- [x] إنشاء ملف `docker-compose.migration.yml` يُعرّف الخدمات والاعتمادات والحاويات بالكامل
  - *Create `docker-compose.migration.yml` fully defining services, dependencies, and containers*
- [x] إضافة مهام الفحص الصحي (health checks)، وتقييدات الموارد (CPU/Memory)، واعتمادات (depends_on) لتكوين الحاويات
  - *Add health checks, resource limits (CPU/Memory), and dependency order (depends_on) to containers config*
- [x] كتابة وثيقة `DOCKER_ARCHITECTURE.md` لشرح بنية الحاويات
  - *Write `DOCKER_ARCHITECTURE.md` explaining container architecture*

## 4. النقل والتحويل | Migration and Conversion

- [x] إنشاء Component Adapter لاستخراج وتحويل المكونات من Next.js و Vite/React لتتوافق مع Next.js App Router
  - *Create Component Adapter to extract and convert components from Next.js and Vite/React to conform with Next.js App Router*
- [x] نقل مكونات Cerberus/webapp (15 مكون) وPentagi (54 مكون) وDeepCode new_ui (17+ مكون) إلى dragon-ui
  - *Migrate components from Cerberus/webapp, Pentagi, and DeepCode new_ui directly into dragon-ui (Completed layout, admin, red-team)*
- [ ] توثيق أي اختلافات سلوكية وتعديل الوحدات الاختبارية
  - *Document any behavioral differences and adjust unit tests accordingly*
- [ ] إنشاء اختبارات وحدة لكل مكون محوّل
  - *Create unit tests for each migrated and converted component*

## 5. تكامل خدمات AI المستقلة | Independent AI Services Integration

- [x] بناء واجهات تحكم (Control Interfaces) لكل من DeepCode، CAI، Mem0، وNanochat في dragon-ui
  - *Build Control Interfaces for DeepCode, CAI, Mem0, and Nanochat directly inside dragon-ui*
- [ ] تنفيذ الاتصالات الثنائية والمتزامنة (REST للتحكم، WebSocket للوقت الفعلي كـ Nanochat) كما هو محدد في التصاميم
  - *Implement bidirectional and synchronous comms (REST for control, WebSocket for real-time like Nanochat) as specified in designs*
- [x] تطوير واجهة مستخدم طرفية (web-based TUI) لـ CAI Terminal مع جميع اختصارات لوحة المفاتيح
  - *Develop web-based TUI for CAI Terminal supporting all standard keyboard shortcuts*
- [ ] تنفيذ استراتيجيات الاسترجاع الاحتياطي (Fallback Strategies) الخاصة بكل خدمة وفقاً للتصميم (مثل التخزين المؤقت، وضع الاوفلاين)
  - *Implement service-specific Fallback Strategies as designed (e.g. caching, offline mode functionality)*



## 7. التوثيق والتدريب | Documentation and Training

- [ ] إعداد ملفات الوثائق المطلوبة بصيغة مدعومة باللغتين (العربية والإنجليزية)
  - *Prepare required documentation files in supported bilingual formats (Arabic & English)*
- [ ] كتابة دليل المستخدم، دليل المطور، دليل الترحيل، وأدلة التعليمات البرمجية
  - *Write user guide, developer guide, migration guide, and codebase instructions*
- [ ] إعداد سجل التغييرات (changelog) ودليل إعداد البيئة (Environment variables)
  - *Prepare a changelog and Environment Variables setup guide*
- [ ] إنشاء محتوى تعليمي للمهام الرئيسية إذا أمكن (لقطات شاشة أو فيديوهات)
  - *Create walkthrough content for main tasks if possible (screenshots or videos)*

## 8. تجربة المستخدم والعلامة التجارية | UX and Branding

- [x] تطبيق نظام التصميم الموحد للون والخط المسطر في ملف التصميم dragon-ui على جميع الصفحات
  - *Apply the unified design system (colors, fonts) outlined in dragon-ui design to all pages*
- [x] تنفيذ شعار Dragon الهرمي مع الألوان والخطوط الفرعية لكل خدمات AI
  - *Implement hierarchical Dragon branding with sub-service colors and sub-brands for each AI service*
- [ ] تحديث favicon وأيقونات التطبيق لتتلاءم مع العلامة التجارية والهوية الفرعية
  - *Update favicon and app icons to match the overall brand and sub-identities*

## 9. الجودة والاختبار | Quality and Testing

- [ ] تطوير اختبارات E2E شاملة تغطي جميع السيناريوهات عبر الخدمات المدمجة باستخدام Cypress/Playwright
  - *Develop comprehensive E2E tests covering all scenarios across unified services using Cypress/Playwright*
- [ ] تنفيذ اختبارات التحميل (100 مستخدم متزامن) واختبارات الأداء لدرجة Lighthouse أعلى من 85%
  - *Execute load testing (100 concurrent users) and performance testing targeted at Lighthouse score > 85%*
- [ ] إعداد اختبارات التراجع والتحقق من صحة نقاط الاستعادة (Rollback points) تحت الضغط
  - *Prepare regression tests and validate Rollback points under simulated load*
- [ ] تنفيذ اختبار التراجع البصري (Visual Regression) لضمان عدم تأثر الواجهات سلباً
  - *Perform visual regression testing to ensure UIs are not negatively impacted*

## 10. إزالة الواجهات القديمة | Legacy UIs Deprecation

- [ ] وضع الواجهات المصدر (Source UIs) في وضع القراءة فقط مؤقتًا بعد التأكد من الترحيل
  - *Set Source UIs to read-only mode temporarily after migration validation*
- [ ] عرض شريط تحذير بشأن إيقاف الاستخدام مع توجيه الرابط إلى الواجهة الموحدة Dragon UI
  - *Display deprecation warning banner linking and routing to the unified Dragon UI*
- [ ] أرشفة الكود في فروع Git مخصصة قبل الحذف النهائي مع إنشاء تقارير بالنسخة الاحتياطية
  - *Archive logic into specific Git branches before final deletion, accompanying backup reports*
