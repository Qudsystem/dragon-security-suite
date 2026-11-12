# دليل إجراءات الاستعادة (Rollback Procedures)
**Dragon R1 UI Consolidation - Safety Net**

---

## 📋 المحتويات
1. [نظرة عامة](#نظرة-عامة)
2. [نقاط الاستعادة](#نقاط-الاستعادة)
3. [إجراءات الاستعادة السريعة](#إجراءات-الاستعادة-السريعة)
4. [إجراءات الاستعادة الكاملة](#إجراءات-الاستعادة-الكاملة)
5. [التحقق من نجاح الاستعادة](#التحقق-من-نجاح-الاستعادة)
6. [سيناريوهات الطوارئ](#سيناريوهات-الطوارئ)

---

## نظرة عامة

### 🎯 الهدف
توفير آلية آمنة وسريعة للتراجع عن أي تغييرات في حالة حدوث مشاكل أثناء عملية دمج الواجهات.

### 🔒 مبادئ الأمان
1. **نقطة استعادة قبل كل مرحلة** - لا تبدأ أي مرحلة بدون نقطة استعادة
2. **التحقق من سلامة النقطة** - اختبار النقطة قبل الاعتماد عليها
3. **الاستعادة خلال 15 دقيقة** - وقت استعادة مضمون
4. **الاحتفاظ بـ 3 نقاط على الأقل** - للمرونة في الاستعادة

### 📦 مكونات نقطة الاستعادة
كل نقطة استعادة تحتوي على:
```
Rollback Point
├── Git Tag (الكود المصدري)
├── Database Snapshot (قواعد البيانات)
├── Docker Images (الحاويات)
├── Configuration Files (ملفات الإعدادات)
├── Environment Variables (متغيرات البيئة)
└── Metadata (معلومات النقطة)
```

---

## نقاط الاستعادة

### Rollback Point 0: Baseline (نقطة البداية)
**التوقيت:** قبل بدء أي عمل  
**الغرض:** الحالة الأصلية للنظام قبل الدمج

```bash
# إنشاء نقطة البداية
./scripts/create_rollback_point.sh \
  --phase 0 \
  --name "baseline" \
  --description "Initial state before UI consolidation"
```

**المحتويات:**
- ✅ جميع الواجهات الثمانية في حالتها الأصلية
- ✅ قواعد البيانات الحالية (Neo4j, PostgreSQL, Redis)
- ✅ جميع الإعدادات والمتغيرات
- ✅ Docker images الحالية

---

### Rollback Point 1: After Inventory
**التوقيت:** بعد المرحلة 0 (التحضير والجرد)  
**الغرض:** بعد إنشاء Feature Inventory والتوثيق

```bash
./scripts/create_rollback_point.sh \
  --phase 1 \
  --name "after-inventory" \
  --description "After feature inventory and planning"
```

**المحتويات:**
- ✅ Feature Inventory كامل
- ✅ Dependency Graph
- ✅ Migration Plan
- ✅ النظام لم يتغير (نفس Rollback Point 0)

---

### Rollback Point 2: After Web UI Migration
**التوقيت:** بعد المرحلة 1 (دمج webapp + pentagi)  
**الغرض:** بعد نقل 67 مكون من webapp و pentagi

```bash
./scripts/create_rollback_point.sh \
  --phase 2 \
  --name "after-web-ui-migration" \
  --description "After webapp and pentagi frontend migration"
```

**المحتويات:**
- ✅ dragon-ui مع المكونات المنقولة (67 مكون)
- ✅ webapp و pentagi في حالة read-only
- ✅ قواعد البيانات محدثة
- ✅ اختبارات E2E ناجحة

---

### Rollback Point 3: After AI Services Integration
**التوقيت:** بعد المرحلة 2 (تكامل خدمات AI)  
**الغرض:** بعد ربط DeepCode, CAI, Mem0, Nanochat

```bash
./scripts/create_rollback_point.sh \
  --phase 3 \
  --name "after-ai-integration" \
  --description "After AI services microservices integration"
```

**المحتويات:**
- ✅ dragon-ui مع Control Interfaces
- ✅ AI Services تعمل كخدمات مستقلة
- ✅ API Gateway مُفعّل
- ✅ WebSocket connections تعمل

---

### Rollback Point 4: After UX Unification
**التوقيت:** بعد المرحلة 3 (توحيد التجربة)  
**الغرض:** بعد تطبيق Design System

```bash
./scripts/create_rollback_point.sh \
  --phase 4 \
  --name "after-ux-unification" \
  --description "After design system and performance optimization"
```

**المحتويات:**
- ✅ Design System موحد
- ✅ Lighthouse score > 85%
- ✅ Performance optimizations
- ✅ Dragon branding كامل

---

### Rollback Point 5: Before Production Cutover
**التوقيت:** قبل النشر النهائي  
**الغرض:** آخر نقطة آمنة قبل حذف الواجهات القديمة

```bash
./scripts/create_rollback_point.sh \
  --phase 5 \
  --name "before-production-cutover" \
  --description "Final backup before legacy UI removal"
```

**المحتويات:**
- ✅ dragon-ui جاهز للإنتاج
- ✅ جميع الاختبارات ناجحة
- ✅ الواجهات القديمة في read-only
- ✅ Zero critical bugs

---

## إجراءات الاستعادة السريعة

### 🚨 متى تستخدم الاستعادة السريعة؟
- خطأ بسيط في الكود
- مشكلة في مكون واحد
- خطأ في الإعدادات
- **لا توجد مشاكل في قاعدة البيانات**

### ⚡ الخطوات (5-10 دقائق)

#### 1. تحديد نقطة الاستعادة
```bash
# عرض جميع نقاط الاستعادة المتاحة
git tag -l "rollback-phase-*"

# مثال:
# rollback-phase-0-baseline
# rollback-phase-1-after-inventory
# rollback-phase-2-after-web-ui-migration
# rollback-phase-3-after-ai-integration
# rollback-phase-4-after-ux-unification
```

#### 2. استعادة الكود المصدري
```bash
# الاستعادة إلى نقطة محددة
git checkout rollback-phase-2-after-web-ui-migration

# إنشاء branch جديد من النقطة
git checkout -b recovery-from-phase-2 rollback-phase-2-after-web-ui-migration
```

#### 3. استعادة Docker Containers
```bash
# إيقاف الحاويات الحالية
docker-compose down

# استعادة الصور من النقطة
docker run dragon-ui:rollback-phase-2
docker run dragon-gateway:rollback-phase-2

# أو استخدام docker-compose من النقطة
docker-compose -f docker-compose.rollback-phase-2.yml up -d
```

#### 4. التحقق من النظام
```bash
# اختبار الصحة
curl http://localhost:3000/health
curl http://localhost:8000/health

# اختبار الواجهة
npm run test:e2e -- --testNamePattern="critical paths"
```

---

## إجراءات الاستعادة الكاملة

### 🔥 متى تستخدم الاستعادة الكاملة؟
- مشاكل في قاعدة البيانات
- فقدان بيانات
- فشل متعدد في الخدمات
- **حالات الطوارئ الحرجة**

### 🛡️ الخطوات (10-15 دقيقة)

#### 1. إيقاف جميع الخدمات
```bash
# إيقاف كامل للنظام
docker-compose down --volumes

# التأكد من إيقاف جميع العمليات
docker ps -a
pkill -f "node|python|streamlit"
```

#### 2. استعادة قواعد البيانات

##### Neo4j
```bash
# إيقاف Neo4j
docker stop neo4j

# استعادة البيانات
docker run --rm \
  -v neo4j_data:/data \
  -v $(pwd)/backups/rollback-phase-2:/backup \
  neo4j:5.x \
  neo4j-admin database restore --from=/backup/neo4j-backup.dump

# إعادة تشغيل Neo4j
docker start neo4j
```

##### PostgreSQL
```bash
# إيقاف PostgreSQL
docker stop postgres

# استعادة البيانات
docker run --rm \
  -v postgres_data:/var/lib/postgresql/data \
  -v $(pwd)/backups/rollback-phase-2:/backup \
  postgres:15 \
  pg_restore -d dragon_db /backup/postgres-backup.sql

# إعادة تشغيل PostgreSQL
docker start postgres
```

##### Redis
```bash
# إيقاف Redis
docker stop redis

# استعادة البيانات
docker run --rm \
  -v redis_data:/data \
  -v $(pwd)/backups/rollback-phase-2:/backup \
  redis:7-alpine \
  redis-cli --rdb /backup/redis-backup.rdb

# إعادة تشغيل Redis
docker start redis
```

#### 3. استعادة الكود والإعدادات
```bash
# استعادة الكود
git checkout rollback-phase-2-after-web-ui-migration

# استعادة ملفات الإعدادات
cp backups/rollback-phase-2/.env.production .env.production
cp backups/rollback-phase-2/docker-compose.yml docker-compose.yml

# استعادة node_modules إذا لزم الأمر
rm -rf node_modules
npm ci
```

#### 4. إعادة بناء وتشغيل الخدمات
```bash
# إعادة بناء الصور
docker-compose build

# تشغيل جميع الخدمات
docker-compose up -d

# مراقبة السجلات
docker-compose logs -f
```

#### 5. التحقق الشامل
```bash
# اختبار قواعد البيانات
npm run test:db

# اختبار APIs
npm run test:api

# اختبار E2E
npm run test:e2e

# اختبار الأداء
npm run lighthouse
```

---

## التحقق من نجاح الاستعادة

### ✅ قائمة التحقق

#### 1. الخدمات الأساسية
```bash
# التحقق من تشغيل جميع الحاويات
docker ps | grep -E "dragon-ui|gateway|neo4j|postgres|redis"

# يجب أن تظهر جميع الخدمات بحالة "Up"
```

#### 2. قواعد البيانات
```bash
# Neo4j
curl http://localhost:7474/db/data/

# PostgreSQL
docker exec postgres psql -U postgres -d dragon_db -c "SELECT COUNT(*) FROM users;"

# Redis
docker exec redis redis-cli PING
# يجب أن يرجع: PONG
```

#### 3. الواجهات
```bash
# dragon-ui
curl http://localhost:3000/
# يجب أن يرجع: 200 OK

# Gateway API
curl http://localhost:8000/health
# يجب أن يرجع: {"status": "healthy"}
```

#### 4. الوظائف الحرجة
```bash
# اختبار تسجيل الدخول
npm run test:e2e -- --testNamePattern="login"

# اختبار إنشاء مهمة
npm run test:e2e -- --testNamePattern="create task"

# اختبار عرض التقارير
npm run test:e2e -- --testNamePattern="view reports"
```

#### 5. البيانات
```bash
# التحقق من عدد السجلات
npm run verify:data-integrity

# مقارنة مع snapshot
npm run compare:data-snapshot --baseline rollback-phase-2
```

---

## سيناريوهات الطوارئ

### 🚨 السيناريو 1: فشل الاستعادة السريعة

**المشكلة:** الاستعادة السريعة لم تحل المشكلة

**الحل:**
```bash
# 1. الانتقال للاستعادة الكاملة
./scripts/full_rollback.sh --phase 2

# 2. إذا فشلت، الرجوع لنقطة أقدم
./scripts/full_rollback.sh --phase 1

# 3. في أسوأ الحالات، الرجوع للـ baseline
./scripts/full_rollback.sh --phase 0
```

---

### 🚨 السيناريو 2: فقدان نقطة الاستعادة

**المشكلة:** نقطة الاستعادة المطلوبة غير موجودة أو تالفة

**الحل:**
```bash
# 1. التحقق من النسخ الاحتياطية البديلة
ls -la backups/

# 2. استخدام نقطة أقدم
git tag -l "rollback-phase-*" | sort -V | tail -n 3

# 3. استعادة من النسخ الاحتياطية اليدوية
./scripts/restore_from_manual_backup.sh --date 2026-02-27
```

---

### 🚨 السيناريو 3: تلف قاعدة البيانات

**المشكلة:** قاعدة البيانات تالفة ولا يمكن استعادتها

**الحل:**
```bash
# 1. محاولة إصلاح قاعدة البيانات
docker exec neo4j neo4j-admin check-consistency
docker exec postgres pg_dump -U postgres -d dragon_db > recovery.sql

# 2. إذا فشل الإصلاح، استعادة من أقدم نسخة سليمة
./scripts/restore_database.sh --source rollback-phase-1

# 3. إعادة تطبيق التغييرات يدوياً إذا لزم الأمر
./scripts/replay_migrations.sh --from phase-1 --to phase-2
```

---

### 🚨 السيناريو 4: فشل متعدد في الخدمات

**المشكلة:** عدة خدمات لا تعمل بعد الاستعادة

**الحل:**
```bash
# 1. إيقاف كامل للنظام
docker-compose down --volumes
pkill -9 -f "node|python|streamlit"

# 2. تنظيف شامل
docker system prune -a --volumes -f
rm -rf node_modules
rm -rf .next

# 3. استعادة كاملة من الصفر
git checkout rollback-phase-0-baseline
./scripts/full_system_restore.sh --phase 0

# 4. إعادة البناء الكامل
npm ci
docker-compose build --no-cache
docker-compose up -d
```

---

### 🚨 السيناريو 5: فقدان بيانات المستخدمين

**المشكلة:** بيانات المستخدمين مفقودة أو تالفة

**الحل:**
```bash
# 1. إيقاف فوري لجميع الخدمات
docker-compose down

# 2. استعادة من آخر نسخة احتياطية سليمة
./scripts/restore_user_data.sh --from rollback-phase-2

# 3. التحقق من سلامة البيانات
npm run verify:user-data

# 4. إعادة تشغيل الخدمات
docker-compose up -d

# 5. إخطار المستخدمين المتأثرين
npm run notify:affected-users
```

---

## أدوات الاستعادة

### 📜 السكريبتات المتاحة

#### 1. create_rollback_point.sh
```bash
# إنشاء نقطة استعادة جديدة
./scripts/create_rollback_point.sh \
  --phase <رقم المرحلة> \
  --name <اسم النقطة> \
  --description <وصف>

# مثال:
./scripts/create_rollback_point.sh \
  --phase 2 \
  --name "after-web-ui-migration" \
  --description "After webapp and pentagi migration"
```

#### 2. quick_rollback.sh
```bash
# استعادة سريعة (كود فقط)
./scripts/quick_rollback.sh --phase 2

# مع إعادة تشغيل الخدمات
./scripts/quick_rollback.sh --phase 2 --restart
```

#### 3. full_rollback.sh
```bash
# استعادة كاملة (كود + قواعد بيانات + حاويات)
./scripts/full_rollback.sh --phase 2

# مع التحقق التلقائي
./scripts/full_rollback.sh --phase 2 --verify
```

#### 4. verify_rollback.sh
```bash
# التحقق من نجاح الاستعادة
./scripts/verify_rollback.sh --phase 2

# تقرير مفصل
./scripts/verify_rollback.sh --phase 2 --detailed
```

---

## أفضل الممارسات

### ✅ قبل كل مرحلة
1. إنشاء نقطة استعادة جديدة
2. التحقق من سلامة النقطة
3. اختبار إجراءات الاستعادة
4. توثيق الحالة الحالية

### ✅ أثناء التطوير
1. عمل commit متكرر للكود
2. نسخ احتياطية يومية لقواعد البيانات
3. مراقبة سجلات الأخطاء
4. اختبار مستمر

### ✅ بعد كل مرحلة
1. التحقق من نجاح المرحلة
2. إنشاء نقطة استعادة جديدة
3. توثيق التغييرات
4. إخطار الفريق

---

## جهات الاتصال في حالات الطوارئ

### 🆘 فريق الدعم

**قائد المشروع:**
- الاسم: [اسم قائد المشروع]
- الهاتف: [رقم الهاتف]
- البريد: [البريد الإلكتروني]

**مهندس DevOps:**
- الاسم: [اسم مهندس DevOps]
- الهاتف: [رقم الهاتف]
- البريد: [البريد الإلكتروني]

**مهندس قواعد البيانات:**
- الاسم: [اسم مهندس قواعد البيانات]
- الهاتف: [رقم الهاتف]
- البريد: [البريد الإلكتروني]

---

## السجل

### تاريخ التحديثات

| التاريخ | الإصدار | التغييرات | المسؤول |
|---------|---------|-----------|---------|
| 2026-02-27 | 1.0 | إنشاء الوثيقة الأولية | Kiro AI |
| - | - | - | - |

---

**آخر تحديث:** 27 فبراير 2026  
**الحالة:** جاهز للاستخدام ✅  
**المراجعة التالية:** بعد كل مرحلة
