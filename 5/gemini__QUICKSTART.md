# دليل البدء السريع - Kiro-Gemini Coordination

## ✅ ما تم إنشاؤه

تم إنشاء نظام تنسيق كامل بين Kiro و Gemini CLI:

1. **Hook تلقائي**: `.kiro/hooks/kiro-gemini-sync.kiro.hook`
   - يعمل قبل كل عملية كتابة كود من Kiro
   - مفعّل تلقائياً

2. **سكربتات التنسيق**:
   - `.gemini/kiro-sync.sh` (Linux/Mac/WSL)
   - `.gemini/kiro-sync.ps1` (Windows PowerShell)

3. **سكربت إصلاح Node.js**: `.gemini/fix-node-path.sh`
   - يحل مشكلة مسار Node.js في WSL

4. **التوثيق**: `.gemini/README.md`

## 🚀 البدء السريع

### الخطوة 1: إصلاح مسار Node.js (إذا لزم الأمر)

```bash
chmod +x .gemini/fix-node-path.sh
./.gemini/fix-node-path.sh
```

### الخطوة 2: اختبار Hook

```bash
# Hook يعمل تلقائياً عند محاولة Kiro كتابة كود
# لاختباره، اطلب من Kiro إنشاء ملف بسيط:
# "يا Kiro، أنشئ ملف test.txt"
```

### الخطوة 3: مراجعة السجلات

```bash
# عرض سجل التنسيق
cat .gemini/sync.log

# عرض آخر مراجعة من Gemini
cat .gemini/gemini-review.txt
```

## 📊 كيف يعمل النظام

```
┌─────────────────────────────────────────────────┐
│  Kiro يحاول كتابة كود                          │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  Hook: preToolUse يتم تشغيله تلقائياً          │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  kiro-sync.sh يتم تنفيذه                       │
│  - يفحص توفر Gemini CLI                        │
│  - يستخرج السياق الحالي                        │
│  - يشغل Gemini للمراجعة                        │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  Gemini CLI يراجع ويعطي ملاحظات                │
│  النتائج تُحفظ في gemini-review.txt            │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  Hook ينتهي بنجاح (Exit Code 0)                │
└─────────────────┬───────────────────────────────┘
                  │
                  ▼
┌─────────────────────────────────────────────────┐
│  Kiro يكمل عملية الكتابة                       │
└─────────────────────────────────────────────────┘
```

## 🎯 الحالة الحالية

✅ Hook مفعّل ويعمل
✅ السكربتات جاهزة
⚠️  Gemini CLI يحتاج إصلاح مسار Node.js (استخدم fix-node-path.sh)

## 🔧 التخصيص

### تعطيل Hook مؤقتاً

عدّل `.kiro/hooks/kiro-gemini-sync.kiro.hook`:
```json
{
  "enabled": false,  // غيّر من true إلى false
  ...
}
```

### تغيير شروط التشغيل

```json
{
  "when": {
    "type": "preToolUse",
    "toolTypes": ["write"]  // أو ["*"] لجميع الأدوات
  }
}
```

### تخصيص رسالة المراجعة

عدّل `kiro-sync.sh` في دالة `trigger_gemini_review`:
```bash
local prompt="رسالتك المخصصة هنا"
```

## 📝 الملفات المهمة

| الملف | الوظيفة |
|------|---------|
| `.kiro/hooks/kiro-gemini-sync.kiro.hook` | تعريف Hook |
| `.gemini/kiro-sync.sh` | سكربت التنسيق (Linux/Mac) |
| `.gemini/kiro-sync.ps1` | سكربت التنسيق (Windows) |
| `.gemini/sync.log` | سجل جميع عمليات التنسيق |
| `.gemini/gemini-review.txt` | آخر مراجعة من Gemini |
| `.gemini/fix-node-path.sh` | إصلاح مسار Node.js |

## 🐛 استكشاف الأخطاء

### المشكلة: "node: not found"
**الحل**: 
```bash
chmod +x .gemini/fix-node-path.sh
./.gemini/fix-node-path.sh
```

### المشكلة: Hook لا يعمل
**الحل**:
1. تحقق من `"enabled": true` في ملف Hook
2. أعد تشغيل Kiro
3. راجع `.gemini/sync.log`

### المشكلة: Gemini CLI لا يستجيب
**الحل**:
```bash
# تحقق من التثبيت
which gemini

# أو حدد المسار يدوياً
export GEMINI_CLI_PATH="/path/to/gemini"
```

## 🎉 الخطوات التالية

1. أصلح مسار Node.js: `./gemini/fix-node-path.sh`
2. اختبر النظام بطلب من Kiro إنشاء ملف
3. راجع السجلات للتأكد من عمل التنسيق
4. خصص الإعدادات حسب احتياجك

---

**تم إنشاء النظام بنجاح! 🎊**

Hook يعمل الآن تلقائياً قبل كل عملية كتابة من Kiro، مما يضمن مراجعة Gemini CLI أولاً.
