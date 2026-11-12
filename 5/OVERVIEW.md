# نظرة عامة وشرح المواصفات | Specs Overview & Guide

---

## بالعربية

### ما هذا المجلد؟
هذا المجلد يضم **مواصفات نظام Dragon-R1 Enhanced** — منصة أمنية هجومية ذاتية تعمل وفق المبدأ: **"يستلم الهدف ولا يتوقف إلا وهو محقق الهدف"**.

### ملفات المواصفات (ثلاثة)

| الملف | الغرض |
|-------|--------|
| **requirements.md** | المتطلبات الوظيفية وغير الوظيفية، معايير القبول، والمسرد. |
| **design.md** | التصميم المعماري، المكونات، نماذج البيانات، وخصائص الصحة (Correctness Properties). |
| **tasks.md** | خطة التنفيذ: المهام، المراحل، التبعيات، والاختبارات. |

### التصحيحات الأخيرة (شباط 2025)

تم التحقق وإصلاح الأخطاء التالية في النصوص والمواصفات:

1. **تصحيح تعليق في design.md**  
   استبدال "XBOWsed panels" بـ **"Exposed panels"** (الواجهات المعرّضة) في تعليق ماسح الثغرات.

2. **تغطية المتطلب 1 بخصائص الصحة**  
   المتطلب 1 (الواجهة الموحدة الاحترافية — أولوية حرجة) لم يكن مرتبطاً بأي خاصية صحة. تمت إضافة **Property 0: Unified Professional Interface** في design.md بحيث تُختبر الواجهة الموحدة ضمن استراتيجية الاختبار بالخصائص.

3. **استبدال "XBOWnential" بـ "exponential"**  
   في جميع المواضع (استراتيجية إعادة المحاولة والاتصال): في design.md و requirements.md و tasks.md — استُخدمت كلمة **exponential** (التراجع الأسي) بدلاً من "XBOWnential".

4. **تصحيح توسيع اختصار CVE**  
   في requirements.md تم تصحيح "Common Vulnerabilities and XBOWsures" إلى **"Common Vulnerabilities and Exposures"** (الثغرات والمعرّضات الشائعة).

5. **تحسينات إضافية للاتساق**  
   - استبدال علاقة **XBOWSES** بـ **EXPOSES** في design.md.  
   - استبدال "XBOWrt" بـ **export** (تصدير) في requirements.md و tasks.md.

### خطة التنفيذ (باختصار)

- **المرحلة 1**: البنية التحتية وتطبيق الويب (Next.js، API، WebSocket).  
- **المرحلة 2**: وكيل LangGraph الذاتي وأدوات MCP.  
- **المرحلة 3**: خط أنابيب الاستطلاع والرسم البياني (Neo4j).  
- **المرحلة 4**: الاستغلال وما بعد الاستغلال والاختبارات.  
- **المرحلة 5**: الأداء، التوثيق، والنشر.

الجدول الزمني والمهام التفصيلية موجودة في **tasks.md**.

---

## In English

### What is this folder?
This folder contains the **specifications for the Dragon-R1 Enhanced system** — an autonomous offensive security platform that operates on the principle: **"Receives the target and doesn't stop until achieving the goal"**.

### Specification files (three)

| File | Purpose |
|------|--------|
| **requirements.md** | Functional and non-functional requirements, acceptance criteria, and glossary. |
| **design.md** | Architecture, components, data models, and correctness properties. |
| **tasks.md** | Implementation plan: tasks, phases, dependencies, and tests. |

### Recent corrections (February 2025)

The following issues were verified and fixed in the spec text:

1. **Comment fix in design.md**  
   Replaced "XBOWsed panels" with **"Exposed panels"** in the VulnerabilityScanner comment.

2. **Requirement 1 covered by correctness properties**  
   Requirement 1 (Unified Professional Interface — CRITICAL priority) was not validated by any property. **Property 0: Unified Professional Interface** was added in design.md so the unified interface is covered by the property-based testing strategy.

3. **"XBOWnential" → "exponential"**  
   All occurrences (retry/reconnection backoff) in design.md, requirements.md, and tasks.md were updated to **exponential** backoff.

4. **CVE acronym expansion fix**  
   In requirements.md, "Common Vulnerabilities and XBOWsures" was corrected to **"Common Vulnerabilities and Exposures"**.

5. **Additional consistency fixes**  
   - Relationship **XBOWSES** renamed to **EXPOSES** in design.md.  
   - "XBOWrt" replaced with **export** in requirements.md and tasks.md.

### Implementation plan (summary)

- **Phase 1**: Core infrastructure and web app (Next.js, API, WebSocket).  
- **Phase 2**: LangGraph autonomous agent and MCP tools.  
- **Phase 3**: Reconnaissance pipeline and graph (Neo4j).  
- **Phase 4**: Exploitation, post-exploitation, and testing.  
- **Phase 5**: Performance, documentation, and deployment.

Detailed tasks and timeline are in **tasks.md**.
