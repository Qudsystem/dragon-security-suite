# Dragon-R1 Batch Scripts

هذا المجلد يحتوي على ملفات البات المحسنة لتشغيل وإدارة مشروع Dragon-R1.

## الملفات الرئيسية

### 🚀 ملفات التشغيل
- **`DRAGON-START.bat`** - الملف الموحد لتشغيل Dragon-R1 بطرق مختلفة
- **`start-dragon-simple.bat`** - تشغيل بسيط بدون containers
- **`start-services-local.bat`** - تشغيل الخدمات محلياً

### 🧪 ملفات الاختبار
- **`DRAGON-TEST.bat`** - الملف الموحد لجميع الاختبارات
- **`check-endpoints.bat`** - فحص نقاط النهاية
- **`test-endpoints.bat`** - اختبار نقاط النهاية

### 🔧 ملفات الإعداد والإصلاح
- **`diagnose-dragon.bat`** - تشخيص شامل للنظام
- **`fix-podman-setup.bat`** - إصلاح مشاكل Podman
- **`setup-podman-services.bat`** - إعداد خدمات Podman
- **`setup-databases.bat`** - إعداد قواعد البيانات

### 📦 ملفات التثبيت
- **`install-dragon.bat`** - تثبيت متطلبات Dragon-R1

## الملفات القديمة (محدثة)
- **`START-DRAGON-R1.bat`** - يستدعي الملف الموحد الآن
- **`START-DRAGON-R1-PODMAN.bat`** - مبسط ويستدعي إعداد Podman

## كيفية الاستخدام

### التشغيل السريع
```bash
# تشغيل Dragon-R1 مع خيارات متعددة
DRAGON-START.bat

# تشغيل بسيط فوري
start-dragon-simple.bat
```

### التشخيص والإصلاح
```bash
# تشخيص النظام أولاً
diagnose-dragon.bat

# إصلاح مشاكل Podman
fix-podman-setup.bat
```

### الاختبارات
```bash
# تشغيل جميع الاختبارات
DRAGON-TEST.bat

# فحص نقاط النهاية
check-endpoints.bat
```

## الميزات الجديدة

### 1. واجهة عربية
- جميع الرسائل والخيارات باللغة العربية
- سهولة الاستخدام للمطورين العرب

### 2. تشخيص شامل
- فحص Python, Node.js, Docker, Podman
- فحص البورتات والخدمات
- اقتراحات للحلول

### 3. إصلاح تلقائي
- إصلاح مشاكل Podman Machine
- إنشاء البيئات الافتراضية تلقائياً
- تحديث الإعدادات

### 4. تشغيل مرن
- تشغيل بسيط بدون containers
- تشغيل كامل مع Podman/Docker
- خيارات متعددة حسب البيئة

## الملفات المحذوفة

تم حذف الملفات المكررة التالية:
- `test_dragon.bat` (مدمج في DRAGON-TEST.bat)
- `test_dragon_r1.bat` (مدمج في DRAGON-TEST.bat)  
- `run_dragon_tests.bat` (مدمج في DRAGON-TEST.bat)

## ملاحظات مهمة

1. **تشغيل كـ Administrator**: بعض الملفات تحتاج صلاحيات إدارية
2. **المسار الصحيح**: شغل الملفات من المجلد الرئيسي للمشروع
3. **التشخيص أولاً**: استخدم `diagnose-dragon.bat` لفهم المشاكل
4. **التشغيل البسيط**: `start-dragon-simple.bat` للتجربة السريعة

## الدعم

لأي مشاكل:
1. شغل `diagnose-dragon.bat` أولاً
2. اقرأ رسائل الخطأ بعناية
3. استخدم ملفات الإصلاح المناسبة
4. راجع AGENTS.md للتفاصيل التقنية