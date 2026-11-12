# 🎯 إصلاح ألوان الشريط العلوي والقوائم - Dragon R1

## الألوان المستهدفة للاستبدال

### الألوان الحالية (المراد استبدالها):
- `#111113` - الخلفية الأساسية الداكنة جداً (الشريط العلوي)
- `#0F1419` - خلفية داكنة في بعض المكونات
- `#161618` - خلفية القدم في الشريط الجانبي
- `#09090b` - الخلفية الأساسية الأكثر قتامة

### ألوان Dragon R1 البديلة:
- `#1a2332` → بدلاً من `#111113` (الخلفية الرئيسية)
- `#1a2332` → بدلاً من `#0F1419` (الخلفية الرئيسية)
- `#2d3748` → بدلاً من `#161618` (الخلفية الثانوية)
- `#1a2332` → بدلاً من `#09090b` (الخلفية الأساسية)

## الملفات والمكونات المستهدفة

### 1. الشريط العلوي (Header/Topbar):
- `volto-ui/src/components/layout/Shell.tsx`
- `Cerberus/dragon-ui/src/app/globals.css`

### 2. القوائم المنسدلة من الأيقونات:
- `volto-ui/src/components/ui/CommandPalette.tsx`
- `Cerberus/dragon-ui/src/components/sidebar.tsx`

### 3. الخلفيات الأساسية:
- `Cerberus/dragon-ui/src/app/layout.tsx`
- `Cerberus/dragon-ui/src/app/graph/page.tsx`
- `volto-ui/src/components/PipelineDemo.tsx`

### 4. المكونات الأخرى:
- `volto-ui/src/components/dashboard/ConnectedServices.tsx`
- `Cerberus/dragon-ui/src/components/terminal.tsx`
- `Cerberus/dragon-ui/src/components/client-root-layout.tsx`

## خطة التنفيذ

### المرحلة 1: الشريط العلوي والقوائم الرئيسية
1. تحديث Shell.tsx (الشريط العلوي)
2. تحديث القوائم المنسدلة
3. تحديث sidebar footer

### المرحلة 2: الخلفيات الأساسية
1. تحديث layout.tsx
2. تحديث globals.css variables
3. تحديث page backgrounds

### المرحلة 3: المكونات المتخصصة
1. تحديث Terminal component
2. تحديث Dashboard components
3. تحديث Graph page

### المرحلة 4: التحقق والاختبار
1. فحص التناسق البصري
2. اختبار الوصولية
3. التأكد من عدم كسر الوظائف

## الأولوية: عاجل جداً ⚡
هذا التحديث مطلوب لتوحيد ألوان الشريط العلوي والقوائم مع نظام Dragon R1.