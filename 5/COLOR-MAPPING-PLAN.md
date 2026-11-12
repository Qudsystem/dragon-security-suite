# 🎨 خطة استبدال الألوان - Dragon R1 Unified System

## الألوان المكتشفة في الجوانب

### الألوان الحالية (المراد استبدالها):
- `#1f1f1e` - الخلفية الرمادية الداكنة للجوانب
- `#27272a` - لون الهوفر والحدود الثانوية  
- `#3f3f46` - لون الحدود والعناصر التفاعلية
- `#111113` - الخلفية الأساسية الداكنة جداً

### ألوان Dragon R1 البديلة:
- `#1a2332` → بدلاً من `#1f1f1e` (الخلفية الرئيسية)
- `#2d3748` → بدلاً من `#27272a` (الخلفية الثانوية)
- `#4a5568` → بدلاً من `#3f3f46` (الحدود والعناصر)
- `#1a2332` → بدلاً من `#111113` (الخلفية الأساسية)

## الملفات المستهدفة للتحديث:

### 1. ملفات CSS الرئيسية:
- `Cerberus/dragon-ui/src/app/globals.css`
- `volto-ui/src/index.css`
- `Cerberus/dragon-ui/src/styles/design-system.css`

### 2. مكونات React:
- `volto-ui/src/components/ui/RuntimeDropdown.tsx`
- `volto-ui/src/components/ui/ModelDropdown.tsx`
- `volto-ui/src/components/ui/CommandPalette.tsx`
- `volto-ui/src/components/Status/StatusBar.tsx`
- `volto-ui/src/components/sidebar/Sidebar.tsx`

## خطة التنفيذ:

### المرحلة 1: تحديث متغيرات CSS الأساسية
- استبدال الألوان في ملفات CSS الرئيسية
- تحديث متغيرات CSS custom properties

### المرحلة 2: تحديث مكونات React
- استبدال الألوان المباشرة في className
- تحديث الألوان في style attributes

### المرحلة 3: التحقق والاختبار
- فحص التناسق البصري
- اختبار الوصولية والتباين
- التأكد من عدم كسر أي وظائف

## الأولوية: عاجل ⚡
هذا التحديث مطلوب لتوحيد نظام الألوان بالكامل.