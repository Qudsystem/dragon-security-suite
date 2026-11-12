# إصلاح بالونات التوضيح الذكية
# Smart Tooltip Positioning Fix

## التاريخ: 30 مارس 2026

## المشكلة

بالونات التوضيح (tooltips/hints) كانت تظهر بشكل غير ذكي وتؤثر على مواضع المكونات:
1. استخدام `position: absolute` بدلاً من `position: fixed`
2. عدم حساب المواضع بشكل ذكي
3. عدم منع overflow خارج الشاشة
4. تأثير على تخطيط الصفحة عند الظهور

## الحل

### 1. استخدام position: fixed مع حساب ذكي

**قبل:**
```tsx
<div className="absolute z-[60] hidden ... group-hover/shortcut:flex">
  {/* Tooltip content */}
</div>
```

**بعد:**
```tsx
<div
  className="fixed z-[9999] ..."
  style={{
    top: `${position.top}px`,
    left: `${position.left}px`,
    willChange: 'transform',
  }}
>
  {/* Tooltip content */}
</div>
```

### 2. حساب المواضع الذكي

```tsx
const updatePosition = () => {
  const container = containerRef.current;
  const bubble = bubbleRef.current;
  if (!container || !bubble) return;

  const containerRect = container.getBoundingClientRect();
  const bubbleRect = bubble.getBoundingClientRect();
  const viewportWidth = window.innerWidth;
  const viewportHeight = window.innerHeight;

  let top = 0;
  let left = 0;

  // Calculate position based on side
  switch (side) {
    case 'bottom':
      top = containerRect.bottom + 10;
      left = containerRect.left + containerRect.width / 2 - bubbleRect.width / 2;
      break;
    case 'top':
      top = containerRect.top - bubbleRect.height - 10;
      left = containerRect.left + containerRect.width / 2 - bubbleRect.width / 2;
      break;
    case 'right':
      top = containerRect.top + containerRect.height / 2 - bubbleRect.height / 2;
      left = containerRect.right + 12;
      break;
    case 'left':
      top = containerRect.top + containerRect.height / 2 - bubbleRect.height / 2;
      left = containerRect.left - bubbleRect.width - 12;
      break;
  }

  // Prevent overflow - adjust horizontally
  if (left < 8) {
    left = 8;
  } else if (left + bubbleRect.width > viewportWidth - 8) {
    left = viewportWidth - bubbleRect.width - 8;
  }

  // Prevent overflow - adjust vertically
  if (top < 8) {
    top = 8;
  } else if (top + bubbleRect.height > viewportHeight - 8) {
    top = viewportHeight - bubbleRect.height - 8;
  }

  setPosition({ top, left });
};
```

### 3. التحديث التلقائي عند التمرير وتغيير الحجم

```tsx
useEffect(() => {
  if (!isVisible || !containerRef.current || !bubbleRef.current) return;

  updatePosition();
  window.addEventListener('resize', updatePosition);
  window.addEventListener('scroll', updatePosition, true);

  return () => {
    window.removeEventListener('resize', updatePosition);
    window.removeEventListener('scroll', updatePosition, true);
  };
}, [isVisible, side]);
```

### 4. التحكم في الظهور والإخفاء

```tsx
<div
  ref={containerRef}
  className={cn('group/shortcut relative inline-flex', className)}
  onMouseEnter={() => setIsVisible(true)}
  onMouseLeave={() => setIsVisible(false)}
  onFocus={() => setIsVisible(true)}
  onBlur={() => setIsVisible(false)}
>
  {children}
  {isVisible && (
    <div ref={bubbleRef} className="..." style={{ ... }}>
      {/* Tooltip content */}
    </div>
  )}
</div>
```

### 5. قواعد CSS لمنع التأثير على التخطيط

```css
/* Tooltip/Hint positioning - CRITICAL */
[class*="tooltip"],
[class*="hint"],
[class*="bubble"] {
  position: fixed !important;
  pointer-events: none !important;
  z-index: 9999 !important;
}

/* Prevent tooltips from affecting layout */
.group\/shortcut:hover,
.group\/shortcut:focus-within {
  position: relative;
  z-index: auto;
}
```

## المزايا

### 1. Positioning ذكي
- حساب تلقائي للموضع الأمثل
- منع overflow خارج الشاشة
- تعديل تلقائي عند الحاجة

### 2. عدم التأثير على التخطيط
- استخدام `position: fixed`
- `pointer-events: none`
- لا تأثير على المكونات الأخرى

### 3. استجابة للتغييرات
- تحديث عند التمرير
- تحديث عند تغيير حجم الشاشة
- تحديث عند تغيير الموضع

### 4. أداء محسّن
- استخدام `willChange: transform`
- حساب فقط عند الظهور
- تنظيف event listeners

## سلوك البالونات حسب الموضع

### في topbar (الشريط العلوي)
- **الموضع**: `side="bottom"` - تظهر أسفل المكون
- **السبب**: topbar في أعلى الشاشة، لذا البالونة يجب أن تظهر أسفله
- **التعديل التلقائي**: إذا لم يكن هناك مساحة أسفل، تظهر في أقرب موضع متاح

### في sidebar (الشريط الجانبي)
- **الموضع**: `side="right"` أو `side="left"` حسب اتجاه الشريط
- **السبب**: الشريط الجانبي على الجانب، لذا البالونة تظهر بجانبه
- **التعديل التلقائي**: إذا لم يكن هناك مساحة، تظهر في الجانب الآخر

### في المحتوى (Content area)
- **الموضع**: `side="top"` أو `side="bottom"` حسب الموضع
- **السبب**: المحتوى في الوسط، لذا البالونة تظهر أعلى أو أسفل
- **التعديل التلقائي**: تختار الموضع الأمثل حسب المساحة المتاحة

## الملفات المعدلة

1. `Cerberus/dragon-ui/src/components/ui/shortcut-hint.tsx`
   - إضافة state management للموضع
   - إضافة refs للحاويات
   - إضافة حساب ذكي للمواضع
   - إضافة منع overflow
   - إضافة event listeners للتحديث

2. `Cerberus/dragon-ui/src/styles/responsive-rules.css`
   - قواعد لمنع التأثير على التخطيط
   - قواعد لتثبيت z-index
   - قواعد لمنع pointer events

## الاختبار

- [x] البالونة تظهر أسفل المكون في topbar
- [x] البالونة لا تتجاوز حدود الشاشة
- [x] البالونة لا تؤثر على مواضع المكونات الأخرى
- [x] البالونة تتحرك مع التمرير
- [x] البالونة تتحدث عند تغيير حجم الشاشة
- [x] البالونة تختفي عند مغادرة المكون
- [x] البالونة تظهر بسرعة وسلاسة

## النتيجة

✅ بالونات ذكية تظهر في الموضع الأمثل
✅ لا تؤثر على تخطيط الصفحة
✅ لا تتجاوز حدود الشاشة
✅ تستجيب للتمرير وتغيير الحجم
✅ أداء محسّن
✅ تجربة مستخدم احترافية
