# مستند متطلبات إصلاح الأخطاء الشامل
# Comprehensive Error Handling Fix Requirements Document

## مقدمة | Introduction

هذا المستند يحدد متطلبات إصلاح جميع الأخطاء والمشاكل المكتشفة في مشروع Dragon-R1، وهو نظام أمني متقدم للاختبار الاختراقي الأخلاقي. المشروع يتكون من:

- **Frontend (Cerberus/dragon-ui)**: Next.js 16 مع React 19, TypeScript, Tailwind CSS
- **Backend (Cerberus/gateway)**: Python FastAPI 
- **CLI (dragon-cli)**: Go TUI
- **قواعد البيانات**: Neo4j, Redis, PostgreSQL
- **أدوات إضافية**: Ollama, mem0, RAG-Anything

الهدف هو إنشاء بيئة تطوير مستقرة وموثوقة تعمل بدون Docker مؤقتاً، مع معالجة شاملة للأخطاء وضمان عمل جميع المكونات بشكل متكامل على localhost:3000.

## تحليل الأخطاء | Bug Analysis

### السلوك الحالي (العيوب) | Current Behavior (Defects)

#### 1. أخطاء TypeScript والبناء | TypeScript and Build Errors

1.1 عندما يتم تشغيل فحص TypeScript في dragon-ui، يظهر خطأ في ملف `tool-validation.property.test.ts` في السطر 491 حيث لا يمكن تعيين نوع `string` إلى نوع محدد للمراحل

1.2 عندما يتم تشغيل عملية البناء، تفشل العملية بسبب أخطاء TypeScript غير محلولة

1.3 عندما يتم فحص المشروع بواسطة IDE، يظهر 66 مشكلة إجمالية تشمل أخطاء TypeScript وتحذيرات ESLint

#### 2. مشاكل التكامل بين Frontend/Backend | Frontend/Backend Integration Issues

2.1 عندما يحاول Frontend الاتصال بـ Backend عبر API، تحدث أخطاء اتصال أو timeout

2.2 عندما يتم إنشاء WebSocket connection، تفشل المحاولة أو تنقطع الاتصالات بشكل متكرر

2.3 عندما يتم إرسال طلبات API، تحدث أخطاء CORS أو authentication failures

##### 2.3.1 **[CRITICAL - HIGHEST PRIORITY]** Infinite Loop في logout

**الموقع | Location**: `Desktop_RED_Cerberus_b9ae7cc1._.js` — دالة logout  
**الخطأ | Error**: Infinite recursive loop في عملية logout  
**الأثر | Impact**: الملف كله ~48,000 سطر، غالبيتها العظمى تكرار لنفس stack trace لـ logout عبر Axios  
**السبب الجذري | Root Cause**: فشل الـ refreshToken (المشكلة 2.3.2) بيطلق logout → الـ logout نفسه بيعمل API call بيفشل → فيرجع يستدعي logout من جديد، وهكذا بلا توقف  
**الشرط | Condition**: عندما يفشل refreshToken ويتم استدعاء logout، ثم logout نفسه يفشل ويستدعي نفسه مرة أخرى

##### 2.3.2 **[HIGH PRIORITY]** API Error 422 في refreshToken

**الموقع | Location**: `api.ts` → `refreshToken` → مستدعى من `listTasks` → `loadStats` في `sidebar.tsx`  
**الخطأ | Error**: `API Error: 422 Unprocessable Entity`  
**السبب | Cause**: طلب refreshToken بيفشل لأن البيانات المرسلة للـ API غلط أو الـ token منتهي الصلاحية بالكامل. المشكلة بتبدأ من useEffect في الـ Sidebar عند تحميل الكومبوننت  
**الشرط | Condition**: عندما يحاول النظام تحديث token منتهي الصلاحية أو غير صالح، يفشل الطلب ويسبب cascade failure

#### 3. مشاكل إعداد البيئة مع Podman | Environment Setup Issues With Podman

3.1 عندما يتم تشغيل المشروع مع Podman، تفشل الخدمات في العثور على dependencies المطلوبة أو تحدث مشاكل في التكوين

3.2 عندما يتم تشغيل قواعد البيانات باستخدام Podman containers، تحدث أخطاء في الاتصال أو التكوين أو مشاكل في الشبكة

3.3 عندما يتم تشغيل Ollama في Podman container، تحدث مشاكل في تحميل النماذج أو الاتصال أو مشاكل في الموارد

#### 4. أخطاء الاختبارات | Testing Errors

4.1 عندما يتم تشغيل اختبارات Jest في Frontend، تفشل بعض الاختبارات بسبب أخطاء في التكوين أو dependencies

4.2 عندما يتم تشغيل اختبارات Python في Backend، تحدث أخطاء في import أو missing dependencies

4.3 عندما يتم تشغيل property-based tests، تفشل بسبب أخطاء في validation logic

#### 5. مشاكل معالجة الأخطاء | Error Handling Issues

5.1 عندما تحدث أخطاء في runtime، لا يتم عرضها بشكل واضح للمستخدم

5.2 عندما تفشل العمليات، لا يتم تسجيل الأخطاء بشكل مناسب في logs

5.3 عندما تحدث أخطاء في المكونات، لا يتم recovery أو fallback بشكل تلقائي

##### 5.3.1 **[MEDIUM PRIORITY]** TypeError في ValidationManager

**الموقع | Location**: `ValidationManager.ts:97` — دالة `updateRules`  
**الخطأ | Error**: `TypeError: newRules.forEach is not a function`  
**السبب | Cause**: الـ API بيرجع response مش array، فالكود بيحاول يعمل forEach على قيمة مش iterable (ممكن تكون null، object، أو undefined)  
**الشرط | Condition**: عندما يتم استدعاء updateRules مع قيمة newRules ليست array، يحدث crash بدلاً من validation

##### 5.3.2 **[LOW PRIORITY]** Unhandled WebSocket Messages

**الموقع | Location**: `websocket.ts:167`  
**الرسائل | Messages**: `Unknown message type: notification_info`  
**السبب | Cause**: الـ WebSocket server بيبعث messages من نوع notification_info والكود مش عارف يتعامل معاها  
**الشرط | Condition**: عندما يستقبل WebSocket رسالة من نوع notification_info، يتم تسجيل تحذير متكرر بدلاً من معالجة صامتة

#### ملخص الأولويات | Priority Summary

**ترتيب الأولويات للأخطاء المحددة | Priority Order for Specific Errors:**

1. **CRITICAL (أعلى أولوية)**: المشكلة 2.3.1 — Infinite loop في logout  
   - تأثيرها كارثي على الأداء (~48,000 سطر من stack traces)
   - تسبب استهلاك موارد هائل وتجميد التطبيق
   - يجب إصلاحها أولاً لأنها تمنع أي عمل آخر

2. **HIGH (أولوية عالية)**: المشكلة 2.3.2 — API Error 422 في refreshToken  
   - تسبب المشكلة الأولى (infinite loop)
   - تمنع المستخدمين من الاستمرار في استخدام التطبيق
   - cascade failure يؤثر على جميع API calls

3. **MEDIUM (أولوية متوسطة)**: المشكلة 5.3.1 — TypeError في ValidationManager  
   - تسبب crash في validation logic
   - تؤثر على استقرار التطبيق لكن ليست catastrophic
   - يمكن معالجتها بعد حل مشاكل authentication

4. **LOW (أولوية منخفضة)**: المشكلة 5.3.2 — Unhandled WebSocket Messages  
   - مجرد console warnings متكررة
   - لا تؤثر على الوظائف الأساسية
   - يمكن إصلاحها في النهاية

### السلوك المتوقع (الصحيح) | Expected Behavior (Correct)

#### 1. إصلاح أخطاء TypeScript والبناء | TypeScript and Build Fixes

2.1 عندما يتم تشغيل فحص TypeScript، يجب أن يمر بدون أخطاء مع تعريف صحيح لأنواع البيانات

2.2 عندما يتم تشغيل عملية البناء، يجب أن تكتمل بنجاح وتنتج build قابل للتشغيل

2.3 عندما يتم فحص المشروع بواسطة IDE، يجب أن تظهر 0 أخطاء حرجة و أقل من 10 تحذيرات غير حرجة

#### 2. تحسين التكامل بين Frontend/Backend | Improved Frontend/Backend Integration

2.4 عندما يحاول Frontend الاتصال بـ Backend، يجب أن تنجح جميع API calls خلال أقل من 500ms

2.5 عندما يتم إنشاء WebSocket connection، يجب أن يتم الاتصال بنجاح مع auto-reconnect عند الانقطاع

2.6 عندما يتم إرسال طلبات API، يجب أن تمر بدون أخطاء CORS مع proper authentication

##### 2.6.1 **[CRITICAL]** إصلاح Infinite Loop في logout

**السلوك المطلوب | Expected Behavior**: عندما يفشل refreshToken ويتم استدعاء logout، يجب أن يتم تنفيذ logout مرة واحدة فقط مع منع أي استدعاءات متكررة  
**الآلية | Mechanism**: إضافة حماية (flag أو semaphore) تمنع استدعاء logout أكثر من مرة، وإيقاف أي retry loops تلقائياً عند فشل المصادقة  
**النتيجة | Result**: يجب أن يتم logout بنجاح مرة واحدة، ثم إعادة توجيه المستخدم لصفحة تسجيل الدخول بدون infinite loop

##### 2.6.2 **[HIGH]** إصلاح API Error 422 في refreshToken

**السلوك المطلوب | Expected Behavior**: عندما يفشل refreshToken بسبب token منتهي أو غير صالح، يجب أن يتم معالجة الفشل gracefully  
**الآلية | Mechanism**: معالجة حالة فشل الـ token refresh — إعادة توجيه المستخدم لصفحة تسجيل الدخول بدل الاستمرار في المحاولة  
**النتيجة | Result**: يجب أن يتم إعادة توجيه المستخدم فوراً لصفحة login عند فشل refresh، بدون محاولات متكررة أو cascade failures

#### 3. إعداد بيئة مستقرة مع Podman | Stable Environment Setup With Podman

2.7 عندما يتم تشغيل المشروع مع Podman، يجب أن تعمل جميع الخدمات مع تكوين Podman صحيح ومتوافق

2.8 عندما يتم تشغيل قواعد البيانات في Podman containers، يجب أن تتصل بنجاح مع إعداد تلقائي للمخططات وشبكة Podman مناسبة

2.9 عندما يتم تشغيل Ollama في Podman container، يجب أن يحمل النماذج المطلوبة ويستجيب للطلبات مع تكوين موارد مناسب

#### 4. إصلاح الاختبارات | Testing Fixes

2.10 عندما يتم تشغيل اختبارات Jest، يجب أن تمر جميع الاختبارات مع coverage أكثر من 80%

2.11 عندما يتم تشغيل اختبارات Python، يجب أن تمر جميع الاختبارات مع proper mocking للخدمات الخارجية

2.12 عندما يتم تشغيل property-based tests، يجب أن تمر مع validation logic صحيح

#### 5. تحسين معالجة الأخطاء | Enhanced Error Handling

2.13 عندما تحدث أخطاء في runtime، يجب أن تظهر رسائل خطأ واضحة ومفيدة للمستخدم

2.14 عندما تفشل العمليات، يجب أن يتم تسجيل الأخطاء بتفاصيل كاملة في structured logs

2.15 عندما تحدث أخطاء في المكونات، يجب أن يتم تطبيق graceful degradation مع fallback mechanisms

##### 2.15.1 **[MEDIUM]** إصلاح TypeError في ValidationManager

**السلوك المطلوب | Expected Behavior**: عندما يتم استدعاء updateRules، يجب أن يتم التحقق من أن newRules هو array قبل استدعاء forEach  
**الآلية | Mechanism**: إضافة validation قبل استدعاء forEach — التحقق إن newRules هو فعلاً array قبل المعالجة، مع معالجة الحالات الأخرى (null، object، undefined) بشكل آمن  
**النتيجة | Result**: يجب أن يعمل updateRules بدون crash حتى لو كان response من API غير متوقع، مع تسجيل warning مناسب

##### 2.15.2 **[LOW]** معالجة Unhandled WebSocket Messages

**السلوك المطلوب | Expected Behavior**: عندما يستقبل WebSocket رسالة من نوع notification_info، يجب أن يتم معالجتها أو تجاهلها بشكل صامت  
**الآلية | Mechanism**: إضافة handler لهذا النوع من الرسائل أو على الأقل تسجيله بشكل صامت بدون تحذيرات متكررة  
**النتيجة | Result**: يجب أن تعمل WebSocket connection بدون console warnings متكررة للرسائل المعروفة

### السلوك غير المتغير (منع التراجع) | Unchanged Behavior (Regression Prevention)

#### 3. الحفاظ على الوظائف الموجودة | Preserve Existing Functionality

3.1 عندما تعمل الميزات الحالية بشكل صحيح، يجب أن تستمر في العمل بنفس الطريقة بعد الإصلاحات

3.2 عندما تعمل واجهة المستخدم بشكل صحيح، يجب أن تحتفظ بنفس UX وتصميم UI

3.3 عندما تعمل API endpoints بشكل صحيح، يجب أن تحتفظ بنفس response format وbehavior

3.4 عندما تعمل قاعدة البيانات بشكل صحيح، يجب أن تحتفظ بنفس schema وdata integrity

3.5 عندما تعمل عمليات الأمان بشكل صحيح، يجب أن تحتفظ بنفس مستوى الحماية والتشفير

3.6 عندما تعمل عمليات التكامل مع الخدمات الخارجية، يجب أن تستمر في العمل بدون تغيير

3.7 عندما تعمل عمليات الذاكرة والتخزين المؤقت، يجب أن تحتفظ بنفس الأداء والكفاءة

3.8 عندما تعمل عمليات التسجيل والمراقبة، يجب أن تستمر في توفير نفس مستوى التفاصيل

3.9 عندما تعمل عمليات النسخ الاحتياطي والاستعادة، يجب أن تحتفظ بنفس الموثوقية

3.10 عندما تعمل عمليات التحديث والصيانة، يجب أن تستمر في العمل بدون تعطيل الخدمة