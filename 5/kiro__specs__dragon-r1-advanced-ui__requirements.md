# مستند المتطلبات - واجهة Dragon R1 المتقدمة

## المقدمة

واجهة مستخدم احترافية متقدمة لمنصة Dragon R1 (Agentic AI Platform للأمن الهجومي)، مستوحاة من تحليل الهندسة العكسية لواجهة OpenAI CodeX. تركز الواجهة على عرض نظام Multi-Agent يعمل في الوقت الفعلي مع تصور واضح لتدفق العمليات، التفكير، والتقدم.

## المصطلحات (Glossary)

- **UI_System**: نظام واجهة المستخدم المتقدمة لـ Dragon R1
- **Workflow_Graph**: لوحة تدفق العمليات التي تعرض خطوات المشروع كـ nodes متصلة
- **Reasoning_Panel**: لوحة التفكير والتحليل التي تعرض Chain of Thought للنظام
- **Node**: عقدة في الرسم البياني تمثل خطوة واحدة (Search, Code Edit, Files, API call)
- **Edge**: خط يربط بين nodes في الرسم البياني
- **Light_Pulse**: نبضة ضوئية متحركة على الخطوط توضح تدفق البيانات
- **CoT_Tree**: شجرة Chain of Thought التي تعرض خطوات التفكير
- **Status_Bar**: شريط حالة المشروع العام
- **Gateway_API**: واجهة برمجية للـ backend على المسار `/api/v1/`
- **WebSocket_Client**: عميل WebSocket للاتصال الحي بالـ backend
- **Zustand_Store**: مخزن الحالة العام للتطبيق
- **Dark_Mode**: وضع الألوان الداكن الاحترافي
- **Active_Node**: عقدة نشطة حالياً في التنفيذ
- **Pending_Node**: عقدة في انتظار التنفيذ
- **Completed_Node**: عقدة تم تنفيذها بنجاح
- **Progress_Indicator**: مؤشر التقدم (spinner + نسبة مئوية)
- **Thinking_Timer**: عداد الوقت الحي لعملية التفكير
- **Dragon_Icon**: أيقونة التنين النابضة
- **Insight**: ملخص سطر واحد لخطوة تفكير
- **Synthesis_Point**: نقطة التجميع النهائية للخلاصة
- **Overall_Progress**: التقدم الإجمالي للمشروع
- **React_Flow**: مكتبة React Flow لعرض الرسوم البيانية
- **Attack_Pipeline**: خط أنابيب الهجوم (RECON → ENUMERATE → DISCOVER → EXPLOIT → ESCALATE → EXFILTRATE → REPORT)
- **Sidebar**: الشريط الجانبي المنزلق للتنقل والمحادثات
- **Header_Bar**: شريط العنوان العلوي الصغير
- **Chat_Bubble**: فقاعة المحادثة للرسائل
- **Deep_Think_Component**: مكون التفكير والبحث العميق
- **Code_Block**: كتلة عرض الأكواد مع خيارات النسخ والتنزيل
- **Table_Component**: مكون عرض الجداول المضبوطة
- **Hierarchical_Input**: صندوق الإدخال المتدرج
- **Topic_Navigator**: مكون التنقل الذكي بين المواضيع
- **Scroll_Button**: زر النزول لآخر رسالة
- **Ollama_Settings**: بطاقة إعدادات Ollama
- **RTL_Support**: دعم اللغة العربية واتجاه RTL
- **Desktop_Frame**: إطار التطبيق المحاكي لسطح المكتب
- **Avatar**: الصورة الرمزية الدائرية
- **Badge**: الشارة التي تعرض مصدر المعلومات
- **Chip**: الحبة في مكون التنقل بين المواضيع
- **Progress_Bar**: شريط التقدم في تحميل النموذج
- **Context_Menu**: القائمة المنبثقة عند الضغط المطول
- **Search_Box**: مربع البحث داخل القائمة
- **Time_Section**: قسم زمني في سجل المحادثات
- **File_Preview**: معاينة مصغرة للملفات المرفقة

## المتطلبات

### المتطلب 1: لوحة تدفق العمليات (Workflow Graph)

**User Story:** كمستخدم للمنصة، أريد رؤية تدفق العمليات بشكل مرئي، حتى أفهم الخطوات الجارية والعلاقات بينها.

#### معايير القبول (Acceptance Criteria)

1. THE Workflow_Graph SHALL عرض جميع خطوات المشروع كـ nodes باستخدام React_Flow
2. WHEN a Node تكون في حالة pending، THE Workflow_Graph SHALL عرضها بشفافية 0.3
3. WHEN a Node تكون Active_Node، THE Workflow_Graph SHALL عرضها محاطة بهالة مضيئة
4. WHEN a Node تكون Completed_Node، THE Workflow_Graph SHALL عرضها بلون كامل مع علامة إتمام
5. THE Workflow_Graph SHALL رسم Edges رفيعة تربط بين Nodes المتصلة
6. WHEN بيانات تتدفق بين nodes، THE Workflow_Graph SHALL عرض Light_Pulse متحركة على Edge المعنية
7. WHEN a Node تكون Active_Node، THE Workflow_Graph SHALL عرض Progress_Indicator عليها (spinner دائري + نسبة مئوية)
8. THE Workflow_Graph SHALL دعم أنواع Nodes التالية: Search, Code Edit, Files, API calls
9. THE Workflow_Graph SHALL تحديث الحالة في الوقت الفعلي عبر WebSocket_Client
10. THE Workflow_Graph SHALL السماح بالتكبير والتصغير والسحب للتنقل

### المتطلب 2: لوحة التفكير والتحليل (Reasoning Panel)

**User Story:** كمستخدم للمنصة، أريد رؤية عملية تفكير النظام، حتى أفهم كيف يتخذ القرارات.

#### معايير القبول (Acceptance Criteria)

1. THE Reasoning_Panel SHALL عرض Thinking_Timer حي يعد الثواني (مثال: "Thinking Time: 00:04s")
2. THE Reasoning_Panel SHALL عرض Dragon_Icon نابضة في الـ header
3. WHEN التفكير نشط، THE Reasoning_Panel SHALL عرض CoT_Tree كخط رأسي منقط
4. WHEN خطوة تفكير جديدة تبدأ، THE Reasoning_Panel SHALL إضافة أيقونة صغيرة على CoT_Tree
5. THE Reasoning_Panel SHALL عرض Insight (ملخص سطر واحد) تحت كل أيقونة خطوة
6. THE Reasoning_Panel SHALL عرض عداد فرعي للثواني لكل خطوة تفكير
7. WHEN التفكير يكتمل، THE Reasoning_Panel SHALL عرض Synthesis_Point كنقطة مضيئة كبيرة
8. WHEN Synthesis_Point تظهر، THE Reasoning_Panel SHALL كتابة الخلاصة النهائية بشكل متحرك
9. THE Reasoning_Panel SHALL استخدام خط Monospace لعرض محتوى التفكير
10. THE Reasoning_Panel SHALL السماح بطي وفتح CoT_Tree بالنقر على السهم

### المتطلب 3: شريط حالة المشروع العام (Overall Status Bar)

**User Story:** كمستخدم للمنصة، أريد رؤية التقدم الإجمالي للمشروع، حتى أعرف مدى اقتراب الإنجاز.

#### معايير القبول (Acceptance Criteria)

1. THE Status_Bar SHALL عرض Overall_Progress كشريط تقدم مرئي
2. THE Status_Bar SHALL عرض النسبة المئوية للتقدم الإجمالي
3. THE Status_Bar SHALL حساب التقدم بناءً على عدد Nodes المكتملة مقابل الإجمالي
4. THE Status_Bar SHALL تحديث التقدم في الوقت الفعلي
5. THE Status_Bar SHALL عرض المرحلة الحالية من Attack_Pipeline
6. WHERE المشروع مكتمل، THE Status_Bar SHALL عرض رسالة نجاح مع أيقونة
7. THE Status_Bar SHALL استخدام ألوان متدرجة لتوضيح التقدم (أحمر → أصفر → أخضر)

### المتطلب 4: نظام الألوان والتصميم (Dark Mode Design System)

**User Story:** كمستخدم للمنصة، أريد واجهة احترافية مريحة للعين، حتى أستطيع العمل لفترات طويلة.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL استخدام Dark_Mode كوضع افتراضي
2. THE UI_System SHALL استخدام رماديات داكنة كألوان أساسية (background: #0a0a0a إلى #1a1a1a)
3. THE UI_System SHALL استخدام ألوان متباينة للعناصر النشطة (cyan: #00d9ff, green: #00ff88, red: #ff4444)
4. THE UI_System SHALL استخدام خط Sans-serif للواجهة العامة
5. THE UI_System SHALL استخدام خط Monospace للأكواد والـ Reasoning_Panel
6. THE UI_System SHALL تطبيق Visual Hierarchy قوي (حجم خط، تباين، مسافات)
7. THE UI_System SHALL استخدام shadows و glows خفيفة للعناصر النشطة
8. THE UI_System SHALL الحفاظ على نظافة مطلقة (minimal clutter)
9. THE UI_System SHALL استخدام animations سلسة (transitions: 200-300ms)
10. THE UI_System SHALL دعم Material Icons أو Font Awesome للأيقونات

### المتطلب 5: التكامل مع WebSocket للبيانات الحية

**User Story:** كمستخدم للمنصة، أريد رؤية التحديثات فوراً، حتى أتابع العمليات في الوقت الفعلي.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL الاتصال بـ WebSocket_Client الموجود في `src/lib/websocket.ts`
2. WHEN WebSocket_Client يستقبل رسالة `task_updates`، THE UI_System SHALL تحديث Workflow_Graph
3. WHEN WebSocket_Client يستقبل رسالة `agent_chat`، THE UI_System SHALL تحديث Reasoning_Panel
4. WHEN WebSocket_Client ينقطع، THE UI_System SHALL عرض مؤشر انقطاع الاتصال
5. WHEN WebSocket_Client يعيد الاتصال، THE UI_System SHALL إخفاء مؤشر الانقطاع وتحديث البيانات
6. THE UI_System SHALL الاشتراك في القنوات: `task_updates`, `agent_chat`, `approvals`
7. THE UI_System SHALL معالجة رسائل الموافقات (approvals) وعرضها للمستخدم
8. IF WebSocket_Client يفشل في الاتصال بعد 5 محاولات، THEN THE UI_System SHALL عرض رسالة خطأ واضحة

### المتطلب 6: التكامل مع Zustand Store

**User Story:** كمطور، أريد إدارة حالة الواجهة بشكل مركزي، حتى تكون البيانات متسقة عبر المكونات.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL استخدام Zustand_Store الموجود في `src/lib/store.ts`
2. THE UI_System SHALL قراءة حالة tasks من Zustand_Store لعرض Workflow_Graph
3. THE UI_System SHALL قراءة حالة chat messages من Zustand_Store لعرض Reasoning_Panel
4. THE UI_System SHALL قراءة حالة graph state من Zustand_Store لعرض Attack_Pipeline
5. THE UI_System SHALL تحديث Zustand_Store عند استقبال بيانات جديدة من WebSocket_Client
6. THE UI_System SHALL استخدام selectors لتحسين الأداء وتجنب re-renders غير ضرورية
7. THE UI_System SHALL الحفاظ على immutability عند تحديث الحالة

### المتطلب 7: التكامل مع Gateway API

**User Story:** كمستخدم للمنصة، أريد تحميل البيانات الأولية، حتى أرى حالة المشروع عند فتح الواجهة.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL استخدام Gateway_API الموجود في `src/lib/api.ts`
2. WHEN الواجهة تُحمّل، THE UI_System SHALL طلب البيانات الأولية من Gateway_API
3. THE UI_System SHALL طلب قائمة المشاريع من `/api/v1/projects`
4. THE UI_System SHALL طلب قائمة المهام من `/api/v1/tasks`
5. THE UI_System SHALL طلب حالة Attack_Pipeline من `/api/v1/pipeline/status`
6. IF Gateway_API يفشل في الاستجابة، THEN THE UI_System SHALL عرض رسالة خطأ وإعادة المحاولة
7. THE UI_System SHALL عرض مؤشر تحميل أثناء انتظار البيانات من Gateway_API
8. THE UI_System SHALL التعامل مع أخطاء المصادقة (401) بإعادة توجيه لصفحة تسجيل الدخول

### المتطلب 8: الرسوم المتحركة والتفاعلات (Animations & Interactions)

**User Story:** كمستخدم للمنصة، أريد واجهة سلسة وجذابة، حتى تكون التجربة ممتعة واحترافية.

#### معايير القبول (Acceptance Criteria)

1. WHEN Node تتغير من pending إلى active، THE Workflow_Graph SHALL تطبيق fade-in animation (300ms)
2. WHEN Light_Pulse تتحرك على Edge، THE Workflow_Graph SHALL استخدام linear animation (1-2 seconds)
3. WHEN Dragon_Icon نابضة، THE Reasoning_Panel SHALL استخدام scale animation (1s loop)
4. WHEN Synthesis_Point تظهر، THE Reasoning_Panel SHALL استخدام glow animation متزايدة
5. WHEN المستخدم يحوم فوق Node، THE Workflow_Graph SHALL عرض tooltip مع التفاصيل
6. WHEN المستخدم ينقر على Node، THE Workflow_Graph SHALL عرض panel جانبي بالمعلومات الكاملة
7. THE UI_System SHALL استخدام easing functions طبيعية (ease-in-out)
8. THE UI_System SHALL تجنب animations مزعجة أو بطيئة (> 500ms)
9. WHERE المستخدم يفضل reduced motion، THE UI_System SHALL تقليل أو إيقاف الرسوم المتحركة

### المتطلب 9: عرض مراحل Attack Pipeline

**User Story:** كمستخدم للمنصة، أريد رؤية المرحلة الحالية من خط الهجوم، حتى أفهم موقع العملية.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL عرض جميع مراحل Attack_Pipeline: RECON, ENUMERATE, DISCOVER, EXPLOIT, ESCALATE, EXFILTRATE, REPORT
2. WHEN مرحلة نشطة، THE UI_System SHALL تمييزها بلون مختلف وإضاءة
3. WHEN مرحلة مكتملة، THE UI_System SHALL عرض علامة صح (✓) بجانبها
4. WHEN مرحلة في انتظار، THE UI_System SHALL عرضها بشفافية منخفضة
5. THE UI_System SHALL عرض المراحل كخط زمني أفقي أو رأسي
6. THE UI_System SHALL السماح بالنقر على مرحلة لعرض تفاصيلها
7. THE UI_System SHALL عرض عدد Nodes المكتملة/الإجمالية لكل مرحلة

### المتطلب 10: الاستجابة والأداء (Responsiveness & Performance)

**User Story:** كمستخدم للمنصة، أريد واجهة سريعة ومستجيبة، حتى لا أواجه تأخيرات أو تجمد.

#### معايير القبول (Acceptance Criteria)

1. WHEN الواجهة تُحمّل، THE UI_System SHALL عرض المحتوى الأساسي خلال 2 ثانية
2. WHEN بيانات جديدة تصل عبر WebSocket، THE UI_System SHALL تحديث الواجهة خلال 100ms
3. THE UI_System SHALL استخدام React.memo و useMemo لتحسين الأداء
4. THE UI_System SHALL استخدام virtualization لقوائم طويلة (> 50 عنصر)
5. THE UI_System SHALL تجنب re-renders غير ضرورية باستخدام Zustand selectors
6. THE UI_System SHALL دعم شاشات مختلفة (1920x1080 كحد أدنى)
7. THE UI_System SHALL الحفاظ على 60 FPS أثناء الرسوم المتحركة
8. IF عدد Nodes يتجاوز 100، THEN THE UI_System SHALL استخدام clustering أو pagination
9. THE UI_System SHALL تحميل الموارد الثقيلة (images, icons) بشكل lazy
10. THE UI_System SHALL استخدام code splitting لتقليل حجم الحزمة الأولية

### المتطلب 11: إمكانية الوصول (Accessibility)

**User Story:** كمستخدم ذو احتياجات خاصة، أريد واجهة قابلة للاستخدام، حتى أستطيع التفاعل مع المنصة.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL دعم التنقل بالكيبورد (Tab, Enter, Escape, Arrow keys)
2. THE UI_System SHALL استخدام ARIA labels لجميع العناصر التفاعلية
3. THE UI_System SHALL الحفاظ على contrast ratio لا يقل عن 4.5:1 للنصوص
4. THE UI_System SHALL دعم screen readers بشكل صحيح
5. WHEN focus ينتقل لعنصر، THE UI_System SHALL عرض focus indicator واضح
6. THE UI_System SHALL دعم تكبير النص حتى 200% دون كسر التخطيط
7. THE UI_System SHALL توفير نصوص بديلة (alt text) لجميع الأيقونات المعلوماتية
8. WHERE المستخدم يفضل reduced motion، THE UI_System SHALL احترام هذا التفضيل

### المتطلب 12: معالجة الأخطاء والحالات الاستثنائية

**User Story:** كمستخدم للمنصة، أريد رسائل خطأ واضحة، حتى أفهم المشكلة وكيفية حلها.

#### معايير القبول (Acceptance Criteria)

1. IF Gateway_API غير متاح، THEN THE UI_System SHALL عرض رسالة "Backend غير متصل" مع زر إعادة المحاولة
2. IF WebSocket_Client ينقطع، THEN THE UI_System SHALL عرض banner تحذيري في أعلى الشاشة
3. IF بيانات Node فاشلة أو مفقودة، THEN THE Workflow_Graph SHALL عرض Node بلون خطأ (أحمر) مع أيقونة تحذير
4. WHEN خطأ يحدث، THE UI_System SHALL تسجيله في console للمطورين
5. THE UI_System SHALL عرض رسائل خطأ بلغة واضحة وغير تقنية للمستخدم النهائي
6. IF عملية تستغرق أكثر من 30 ثانية، THEN THE UI_System SHALL عرض رسالة "العملية تستغرق وقتاً أطول من المعتاد"
7. THE UI_System SHALL توفير زر "Report Issue" لإرسال تقرير الخطأ
8. WHERE خطأ حرج يحدث، THE UI_System SHALL عرض fallback UI بدلاً من شاشة بيضاء

### المتطلب 13: الشريط الجانبي المنزلق (Sliding Sidebar)

**User Story:** كمستخدم للمنصة، أريد شريط جانبي منظم للتنقل، حتى أصل بسهولة للمحادثات والإعدادات.

#### معايير القبول (Acceptance Criteria)

1. THE Sidebar SHALL عرض رأس قائمة يحتوي على عنوان "Dragon R1"
2. THE Sidebar SHALL عرض ثلاث أيقونات في الرأس: بحث، إضافة محادثة جديدة، إعدادات
3. THE Sidebar SHALL عرض Search_Box لتصفية المحادثات
4. THE Sidebar SHALL عرض سجل المحادثات مقسم إلى Time_Sections: اليوم، الأمس، 7 أيام، 30 يوم، أقدم
5. WHEN المستخدم يضغط ضغطة طويلة على محادثة، THE Sidebar SHALL عرض Context_Menu مع خيارات: إعادة تسمية، تثبيت، مشاركة، حذف
6. THE Sidebar SHALL السماح بالإخفاء والإظهار بحركة انزلاق سلسة
7. WHEN الشريط الجانبي مخفي، THE UI_System SHALL عرض Header_Bar صغير
8. THE Sidebar SHALL الحفاظ على عرض ثابت 280px عند الإظهار
9. THE Sidebar SHALL دعم التمرير العمودي لقوائم المحادثات الطويلة
10. THE Sidebar SHALL عرض عداد المحادثات غير المقروءة بجانب كل Time_Section

### المتطلب 14: شريط العنوان العلوي المصغر (Compact Header Bar)

**User Story:** كمستخدم للمنصة، أريد وصول سريع للقائمة عند إخفائها، حتى لا أفقد إمكانية التنقل.

#### معايير القبول (Acceptance Criteria)

1. WHEN الـ Sidebar مخفي، THE Header_Bar SHALL الظهور في أعلى الشاشة
2. THE Header_Bar SHALL يكون بارتفاع 48px بالضبط
3. THE Header_Bar SHALL عرض زرين فقط: زر القائمة (☰) وزر محادثة جديدة (+)
4. WHEN المستخدم ينقر على زر القائمة، THE Header_Bar SHALL إظهار الـ Sidebar
5. WHEN المستخدم ينقر على زر محادثة جديدة، THE Header_Bar SHALL بدء محادثة فارغة
6. THE Header_Bar SHALL استخدام نفس ألوان الـ Dark_Mode
7. THE Header_Bar SHALL الاختفاء تلقائياً عند إظهار الـ Sidebar
8. THE Header_Bar SHALL عرض عنوان المحادثة الحالية في المنتصف (اختياري)

### المتطلب 15: فقاعات المحادثة المميزة (Distinctive Chat Bubbles)

**User Story:** كمستخدم للمنصة، أريد تمييز واضح بين رسائلي ورسائل المساعد، حتى أتابع المحادثة بسهولة.

#### معايير القبول (Acceptance Criteria)

1. THE Chat_Bubble للمستخدم SHALL استخدام خلفية مميزة (#3B82F6) مع زاوية حادة في أسفل اليمين
2. THE Chat_Bubble للمساعد SHALL استخدام خلفية مختلفة (#1A232E) مع زاوية حادة في أسفل اليسار
3. THE Chat_Bubble SHALL عرض Avatar دائري بجانب كل رسالة
4. THE Chat_Bubble للمستخدم SHALL محاذاة لليمين مع هامش أيسر 20%
5. THE Chat_Bubble للمساعد SHALL محاذاة لليسار مع هامش أيمن 20%
6. THE Chat_Bubble SHALL دعم النصوص متعددة الأسطر مع تباعد مناسب
7. THE Chat_Bubble SHALL عرض طابع زمني صغير أسفل كل رسالة
8. WHEN رسالة طويلة جداً، THE Chat_Bubble SHALL كسر النص تلقائياً دون تجاوز العرض المحدد
9. THE Chat_Bubble SHALL دعم اتجاه RTL للنصوص العربية
10. THE Chat_Bubble SHALL عرض مؤشر "جاري الكتابة" للمساعد عند الحاجة

### المتطلب 16: مكون التفكير والبحث العميق المحدث (Enhanced Deep Think Component)

**User Story:** كمستخدم للمنصة، أريد رؤية تفاصيل عملية البحث والتفكير، حتى أفهم مصادر المعلومات والخطوات المتبعة.

#### معايير القبول (Acceptance Criteria)

1. THE Deep_Think_Component SHALL دعم حالتين: انكماش (عداد فقط) وتوسع (عرض الخطوات)
2. WHEN في حالة الانكماش، THE Deep_Think_Component SHALL عرض عداد الخطوات فقط (مثال: "5 خطوات")
3. WHEN في حالة التوسع، THE Deep_Think_Component SHALL عرض جميع الخطوات مع التفاصيل
4. THE Deep_Think_Component SHALL عرض Badge لكل خطوة تعرض مصدر المعلومات أو عدد الصفحات
5. THE Deep_Think_Component SHALL استخدام أيقونات مختلفة لكل نوع نشاط: 🔍 بحث، 🌐 ويب، 📊 إحصاء، 📄 ملف، 🧠 تحليل
6. WHEN المستخدم ينقر على العنوان، THE Deep_Think_Component SHALL التبديل بين الانكماش والتوسع
7. THE Deep_Think_Component SHALL عرض شريط تقدم أثناء تنفيذ الخطوات
8. THE Deep_Think_Component SHALL عرض الوقت المستغرق لكل خطوة
9. THE Deep_Think_Component SHALL السماح بنسخ محتوى أي خطوة
10. THE Deep_Think_Component SHALL عرض روابط قابلة للنقر للمصادر الخارجية

### المتطلب 17: عرض المحتوى المتقدم (Advanced Content Display)

**User Story:** كمستخدم للمنصة، أريد عرض منظم للأكواد والجداول، حتى أقرأ المحتوى بوضوح وأتفاعل معه.

#### معايير القبول (Acceptance Criteria)

1. THE Code_Block SHALL استخدام خلفية داكنة (#0B0E12) مع رأس يعرض اسم الملف/اللغة
2. THE Code_Block SHALL عرض أيقونتين في الرأس: نسخ وتنزيل
3. WHEN المستخدم ينقر على أيقونة النسخ، THE Code_Block SHALL نسخ المحتوى وعرض إشعار "تم النسخ"
4. WHEN المستخدم ينقر على أيقونة التنزيل، THE Code_Block SHALL تنزيل الملف بالاسم المحدد
5. THE Table_Component SHALL ضبط عرض الأعمدة تلقائياً لمنع خروج النص عن حدود الخلايا
6. THE Table_Component SHALL دعم التمرير الأفقي للجداول العريضة
7. THE Table_Component SHALL عرض خطوط فاصلة واضحة بين الصفوف والأعمدة
8. THE Table_Component SHALL دعم ترتيب الأعمدة بالنقر على رأس العمود
9. THE Code_Block SHALL دعم syntax highlighting للغات البرمجة الشائعة
10. THE Code_Block SHALL عرض أرقام الأسطر للأكواد الطويلة (> 10 أسطر)

### المتطلب 18: صندوق الإدخال المتدرج (Hierarchical Input Box)

**User Story:** كمستخدم للمنصة، أريد صندوق إدخال غني بالميزات، حتى أرسل رسائل متنوعة مع مرفقات.

#### معايير القبول (Acceptance Criteria)

1. THE Hierarchical_Input SHALL استخدام تصميم هرمي: بروز من الضلع العلوي والسفلي بعروض مختلفة
2. THE Hierarchical_Input SHALL عرض أيقونة رفع صورة: مربع لامع مع File_Preview مصغرة
3. THE Hierarchical_Input SHALL عرض أيقونة إرفاق ملف مع دعم أنواع ملفات متعددة
4. THE Hierarchical_Input SHALL عرض أيقونة بحث ويب/DeepThink للتفعيل
5. THE Hierarchical_Input SHALL حقل نصي متعدد الأسطر يتمدد تلقائياً (حد أقصى 6 أسطر)
6. THE Hierarchical_Input SHALL عرض زر إرسال دائري بسهم للأعلى
7. WHEN المستخدم يرفع صورة، THE Hierarchical_Input SHALL عرض معاينة مصغرة قابلة للحذف
8. WHEN المستخدم يكتب نص، THE Hierarchical_Input SHALL تفعيل زر الإرسال
9. THE Hierarchical_Input SHALL دعم اختصارات الكيبورد: Ctrl+Enter للإرسال، Escape للإلغاء
10. THE Hierarchical_Input SHALL عرض عداد الأحرف عند الاقتراب من الحد الأقصى

### المتطلب 19: مكون التنقل الذكي بين المواضيع (Smart Topic Navigator)

**User Story:** كمستخدم للمنصة، أريد التنقل السريع بين أقسام المحادثة، حتى أصل للمعلومات المطلوبة بسرعة.

#### معايير القبول (Acceptance Criteria)

1. THE Topic_Navigator SHALL عرض شريط أفقي بـ Chips تمثل أقسام المحادثة
2. THE Topic_Navigator SHALL عرض شريط عمودي صغير بجانب كل Chip (مضيء/باهت)
3. THE Topic_Navigator SHALL عرض أزرار أسهم للتمرير الأفقي عند وجود chips كثيرة
4. WHEN المستخدم ينقر على Chip، THE Topic_Navigator SHALL التنقل إلى رأس القسم المقابل (Scroll to Heading)
5. THE Topic_Navigator SHALL تمييز الـ Chip النشط حالياً بلون مختلف
6. THE Topic_Navigator SHALL تحديث الـ Chip النشط تلقائياً عند التمرير
7. THE Topic_Navigator SHALL عرض عدد الرسائل في كل قسم على الـ Chip
8. THE Topic_Navigator SHALL دعم التمرير السلس (smooth scrolling) عند النقر
9. THE Topic_Navigator SHALL إخفاء الأسهم عند عدم الحاجة لها
10. THE Topic_Navigator SHALL دعم التنقل بالكيبورد (أسهم يمين/يسار)

### المتطلب 20: زر النزول لآخر رسالة (Scroll to Bottom Button)

**User Story:** كمستخدم للمنصة، أريد العودة السريعة لآخر رسالة، حتى أتابع المحادثة الجارية.

#### معايير القبول (Acceptance Criteria)

1. THE Scroll_Button SHALL الظهور كزر عائم عند التمرير لأعلى أكثر من 200px
2. THE Scroll_Button SHALL الاختفاء تلقائياً عند الوصول لأسفل المحادثة
3. WHEN المستخدم ينقر على الزر، THE Scroll_Button SHALL النزول تلقائياً لأحدث رسالة
4. THE Scroll_Button SHALL استخدام أيقونة سهم للأسفل واضحة
5. THE Scroll_Button SHALL عرض عداد الرسائل الجديدة إذا وجدت (مثال: "3 رسائل جديدة")
6. THE Scroll_Button SHALL الموضع في الزاوية اليمنى السفلى مع هامش مناسب
7. THE Scroll_Button SHALL استخدام animation سلسة للظهور والاختفاء
8. THE Scroll_Button SHALL تغيير اللون عند وجود رسائل جديدة
9. THE Scroll_Button SHALL دعم النقر والضغط المطول (للخيارات الإضافية)
10. THE Scroll_Button SHALL الاختفاء أثناء الكتابة في صندوق الإدخال

### المتطلب 21: بطاقة إعدادات Ollama (Ollama Settings Card)

**User Story:** كمستخدم للمنصة، أريد مراقبة حالة اتصال Ollama، حتى أتأكد من عمل النظام بكفاءة.

#### معايير القبول (Acceptance Criteria)

1. THE Ollama_Settings SHALL عرض عنوان API الحالي مع أيقونة نسخ
2. THE Ollama_Settings SHALL عرض أزرار اختيار طريقة الاتصال: Streaming, Basic, Retry, Chunked
3. THE Ollama_Settings SHALL عرض إحصائيات: عدد النجاح (✅)، الفشل (❌)، متوسط الزمن
4. THE Ollama_Settings SHALL عرض سرعة التوليد الحالية (tokens/s)
5. THE Ollama_Settings SHALL عرض Progress_Bar لتحميل النموذج
6. WHEN المستخدم ينقر على عنوان API، THE Ollama_Settings SHALL نسخ العنوان للحافظة
7. THE Ollama_Settings SHALL تحديث الإحصائيات في الوقت الفعلي
8. THE Ollama_Settings SHALL عرض حالة الاتصال: متصل/منقطع/جاري الاتصال
9. THE Ollama_Settings SHALL السماح بتغيير عنوان API وحفظ الإعدادات
10. THE Ollama_Settings SHALL عرض قائمة النماذج المتاحة مع أحجامها

### المتطلب 22: دعم اللغة العربية الكامل (Full Arabic RTL Support)

**User Story:** كمستخدم عربي، أريد واجهة تدعم اللغة العربية بالكامل، حتى أستخدم المنصة بلغتي الأم.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL دعم اتجاه RTL للنصوص العربية تلقائياً
2. THE UI_System SHALL ضبط اتجاه الأيقونات والأزرار مع اتجاه النص
3. THE UI_System SHALL استخدام خط "Cairo" أو "Tajawal" للنصوص العربية
4. THE UI_System SHALL ضبط الهوامش والمسافات مع اتجاه RTL
5. THE Sidebar SHALL الظهور من الجهة اليمنى عند استخدام العربية
6. THE Chat_Bubble SHALL عكس اتجاه الزوايا الحادة مع اتجاه النص
7. THE Topic_Navigator SHALL دعم التمرير من اليمين لليسار
8. THE UI_System SHALL دعم الأرقام العربية والإنجليزية حسب السياق
9. THE UI_System SHALL ضبط اتجاه القوائم المنبثقة مع اتجاه الواجهة
10. THE UI_System SHALL الحفاظ على قابلية القراءة للنصوص المختلطة (عربي/إنجليزي)

### المتطلب 23: البيئة التقنية المحاكاة (Simulated Desktop Environment)

**User Story:** كمستخدم للمنصة، أريد تجربة تطبيق سطح مكتب احترافي، حتى أشعر بالثقة والاحترافية.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL محاكاة تطبيق سطح مكتب مبني بـ Tauri/Electron
2. THE Desktop_Frame SHALL عرض إطار تطبيق نظيف مع شريط عنوان مخصص
3. THE Desktop_Frame SHALL عرض أزرار التحكم: تصغير، تكبير، إغلاق (حسب نظام التشغيل)
4. THE Desktop_Frame SHALL دعم تغيير حجم النافذة مع الحفاظ على التخطيط
5. THE Desktop_Frame SHALL عرض أيقونة التطبيق وعنوان في شريط العنوان
6. THE UI_System SHALL استخدام shadows واقعية حول إطار التطبيق
7. THE UI_System SHALL دعم وضع ملء الشاشة مع إخفاء شريط العنوان
8. THE Desktop_Frame SHALL عرض شريط حالة في الأسفل (اختياري)
9. THE UI_System SHALL محاكاة سلوك النوافذ الأصلية (focus, blur, resize)
10. THE Desktop_Frame SHALL دعم themes مختلفة حسب نظام التشغيل

### المتطلب 24: نظام التصميم المحدث والألوان المحددة (Updated Design System)

**User Story:** كمستخدم للمنصة، أريد واجهة متسقة وجميلة، حتى تكون التجربة ممتعة ومريحة للعين.

#### معايير القبول (Acceptance Criteria)

1. THE UI_System SHALL استخدام ألوان الخلفيات المحددة: #0B0E12, #12161C, #1A232E
2. THE UI_System SHALL استخدام ألوان التمييز المحددة: #3B82F6، #60A5FA
3. THE UI_System SHALL استخدام خط "Cairo" أو "Tajawal" للنصوص العربية
4. THE UI_System SHALL استخدام خط "JetBrains Mono" للأكواد والمحتوى التقني
5. THE UI_System SHALL تطبيق نظام spacing متسق: 4px, 8px, 16px, 24px, 32px
6. THE UI_System SHALL استخدام border-radius متسق: 4px للعناصر الصغيرة، 8px للكبيرة
7. THE UI_System SHALL تطبيق shadows متدرجة: صغيرة للعناصر العادية، كبيرة للعناصر العائمة
8. THE UI_System SHALL استخدام transitions سلسة 200-300ms لجميع التفاعلات
9. THE UI_System SHALL الحفاظ على contrast ratio عالي للنصوص على الخلفيات الداكنة
10. THE UI_System SHALL توفير متغيرات CSS للألوان والمسافات لسهولة الصيانة
