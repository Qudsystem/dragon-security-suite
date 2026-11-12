# Kiro-Gemini Coordination System

نظام تنسيق متقدم بين Kiro و Gemini CLI لضمان مراجعة الكود قبل التعديل.

## المكونات

### 1. Hook: `kiro-gemini-sync`
- **النوع**: `preToolUse` (قبل استخدام أدوات الكتابة)
- **الوظيفة**: يتم تشغيله تلقائياً قبل أي عملية كتابة كود من Kiro
- **الموقع**: `.kiro/hooks/kiro-gemini-sync.kiro.hook`

### 2. سكربت التنسيق: `kiro-sync.sh` / `kiro-sync.ps1`
- **الوظيفة**: 
  - فحص توفر Gemini CLI
  - استخراج السياق الحالي من Kiro (Spec نشط، مهام، إلخ)
  - تشغيل Gemini CLI للمراجعة
  - حفظ نتائج المراجعة في `.gemini/gemini-review.txt`
- **الموقع**: `.gemini/kiro-sync.sh` (Linux/Mac) و `.gemini/kiro-sync.ps1` (Windows)

### 3. سكربت المراقبة التلقائية: `auto-trigger.sh`
- **الوظيفة**: مراقبة نشاط Kiro والتنسيق التلقائي
- **الموقع**: `.gemini/auto-trigger.sh`

## كيفية الاستخدام

### التفعيل التلقائي
Hook مفعّل تلقائياً. عند كل محاولة من Kiro لكتابة كود:
1. يتم تشغيل `kiro-sync.sh` تلقائياً
2. Gemini CLI يراجع السياق الحالي
3. النتائج تُحفظ في `.gemini/gemini-review.txt`
4. Kiro يكمل عملية الكتابة

### التعطيل المؤقت
لتعطيل Hook مؤقتاً:
```bash
# عدّل الملف .kiro/hooks/kiro-gemini-sync.kiro.hook
# غيّر "enabled": true إلى "enabled": false
```

### إعداد Gemini CLI
تأكد من توفر Gemini CLI:
```bash
# Linux/Mac
which gemini

# Windows
where gemini

# أو حدد المسار يدوياً
export GEMINI_CLI_PATH="/path/to/gemini"  # Linux/Mac
$env:GEMINI_CLI_PATH = "C:\path\to\gemini.exe"  # Windows
```

## السجلات والمخرجات

### ملف السجل
- **الموقع**: `.gemini/sync.log`
- **المحتوى**: سجل كامل لجميع عمليات التنسيق مع الطوابع الزمنية

### مخرجات المراجعة
- **الموقع**: `.gemini/gemini-review.txt`
- **المحتوى**: آخر مراجعة من Gemini CLI

## التخصيص

### تعديل شروط التشغيل
عدّل `.kiro/hooks/kiro-gemini-sync.kiro.hook`:
```json
{
  "when": {
    "type": "preToolUse",
    "toolTypes": ["write"]  // يمكن تغييرها إلى ["*"] لجميع الأدوات
  }
}
```

### تعديل رسالة المراجعة
عدّل السكربت `kiro-sync.sh` أو `kiro-sync.ps1`:
```bash
# في دالة trigger_gemini_review
local prompt="رسالتك المخصصة هنا"
```

## استكشاف الأخطاء

### Hook لا يعمل
1. تحقق من تفعيل Hook: `"enabled": true`
2. تحقق من صلاحيات التنفيذ:
   ```bash
   chmod +x .gemini/kiro-sync.sh
   ```

### Gemini CLI لا يستجيب
1. تحقق من التثبيت: `which gemini` أو `where gemini`
2. حدد المسار يدوياً في متغير البيئة `GEMINI_CLI_PATH`
3. راجع السجل: `.gemini/sync.log`

### لا توجد مخرجات
- تحقق من `.gemini/gemini-review.txt`
- راجع السجل للأخطاء: `.gemini/sync.log`

## الأمان

- جميع السكربتات تعمل في بيئة محلية
- لا يتم إرسال بيانات خارج النظام المحلي
- السجلات محلية فقط

## الدعم

للمساعدة أو الإبلاغ عن مشاكل:
1. راجع السجل: `.gemini/sync.log`
2. تحقق من مخرجات Hook في terminal Kiro
3. تأكد من توفر Gemini CLI

---

**ملاحظة**: هذا النظام يضمن أن Kiro لا يكتب كود دون مراجعة Gemini CLI أولاً، مما يوفر طبقة إضافية من المراجعة والتنسيق.
