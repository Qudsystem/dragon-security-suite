# 🐉 Dragon-R1 - تشغيل سريع

## ✅ الخلاصة
- **قواعس البيانات**: 3 فقط (لا توجد زيادة)
  1. **PostgreSQL** - تخزين البيانات الأساسي (مشاريع، مهام، محادثات، نتائج)
  2. **Neo4j** - خرائط الهجوم والعلاقات
  3. **Redis** - ذاكرة التخزين المؤقت وإدارة الجلسات

- **البيانات المحفوظة**:
  ✅ سجل المحادثات (PostgreSQL + Redis cache)
  ✅ بيانات جلسات المستخدم (PostgreSQL)
  ✅ المشاريع والنتائج (PostgreSQL)
  ✅ خرائط الهجوم (Neo4j)
  ✅ سجل التدقيق (PostgreSQL)

## 🚀 خطوات التشغيل

### الخطوة 1: بدء قواعس البيانات

**على Windows (PowerShell/CMD):**
```cmd
cd C:\Users\Hossam Galal\Desktop\RED
start-databases.bat
```

**على Linux/Mac:**
```bash
cd ~/Desktop/RED
chmod +x start-databases.sh
./start-databases.sh
```

**أو يدوياً:**
```bash
# استخدم Podman أو Docker
podman compose -f podman-compose.yml up -d postgresql neo4j redis

# انتظر 20 ثانية حتى تبدأ البيانات
```

### الخطوة 2: الذهاب لمجلد الـ Gateway

```bash
cd Cerberus/gateway
```

### الخطوة 3: تثبيت المتطلبات (مرة واحدة فقط)

```bash
# إنشاء بيئة افتراضية
python -m venv .venv

# تفعيلها
# Windows:
.venv\Scripts\activate
# Linux/Mac:
source .venv/bin/activate

# تثبيت المكتبات
pip install -r requirements.txt
```

### الخطوة 4: التحقق من الجاهزية

```bash
python startup_check.py
```

**النتيجة المتوقعة:**
```
🐉🐉🐉🐉🐉🐉🐉🐉🐉🐉 DRAGON-R1 PRODUCTION STARTUP

STEP 1: ENVIRONMENT CONFIGURATION
✅ .env file is valid

STEP 2: MODULE IMPORTS
✅ config.get_settings
✅ database_models.Project
✅ database_service.DatabaseService
✅ neo4j_service.Neo4jService
✅ redis_service.RedisService

STEP 3: DATABASE CONTAINERS
✅ Podman detected
✅ PostgreSQL: dragon-r1-postgres is running
✅ Neo4j: dragon-r1-neo4j is running
✅ Redis: dragon-r1-redis is running

STEP 4: DATABASE CONNECTIVITY
[1/3] Testing PostgreSQL...
    ✅ PostgreSQL connected and healthy
[2/3] Testing Neo4j...
    ✅ Neo4j connected and healthy
[3/3] Testing Redis...
    ✅ Redis connected and healthy

STEP 5: DATA PERSISTENCE VERIFICATION
✅ Projects → PostgreSQL `projects` table
✅ Chat Sessions → PostgreSQL + Redis cache
✅ Chat Messages → PostgreSQL `chat_messages` table
✅ All user data will be persisted across sessions

STEP 6: DATABASE ANALYSIS
✅ NO REDUNDANT DATABASES FOUND
✅ Each database serves a distinct critical purpose

🚀 STATUS: READY TO LAUNCH
```

### الخطوة 5: تشغيل الـ Gateway

**الخيار أ - Python مباشر:**
```bash
python main.py
```

**الخيار ب - Uvicorn مع Live Reload:**
```bash
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

**النتيجة المتوقعة:**
```
🐉 Dragon-R1 Gateway starting...
📦 Initializing database services...
✅ PostgreSQL (sync) initialized successfully
✅ Neo4j (sync) initialized successfully  
✅ Redis (sync) initialized successfully
🚀 Event-driven updates enabled

INFO: Uvicorn running on http://0.0.0.0:8000 [Press CTRL+C to quit]
```

### الخطوة 6: الوصول إلى النظام

**API Gateway:**
- 🌐 http://localhost:8000
- 📚 Docs: http://localhost:8000/docs
- 🔄 ReDoc: http://localhost:8000/redoc

**Web Interface (في نافذة أخرى):**
```bash
cd Cerberus/dragon-ui
npm install
npm run dev
```
- 🌐 http://localhost:3000

**قواعس البيانات:**
- 🗄️ PostgreSQL: localhost:5432
- 📊 Neo4j Browser: http://localhost:7474
- ⚡ Redis: localhost:6379

## 🔍 التحقق من البيانات المحفوظة

### اختبار حفظ المحادثة

```bash
# 1. افتح المتصفح وسجل الدخول
http://localhost:3000

# 2. ابدأ محادثة
"مرحبا، كيف يمكنك مساعدتي؟"

# 3. أغلق المتصفح تماماً (اغلق التطبيق)

# 4. افتح المتصفح مجدداً
http://localhost:3000

# ✅ يجب أن ترى المحادثة السابقة!
```

### اختبار حفظ المشروع

```bash
# 1. أنشئ مشروع جديد
Project Name: "Test Project"
Target: "example.com"

# 2. أغلق التطبيق تماماً
# أغلق الـ Gateway (Ctrl+C)

# 3. أعد بدء الـ Gateway
python main.py

# 4. أنشئ مشروع جديد أو اعرض المشاريع
# ✅ المشروع القديم يجب أن يكون موجوداً!
```

### اختبار حفظ النتائج

```bash
# 1. شغّل فحصاً على هدف
# 2. انتظر حتى ينتهي
# 3. أغلق التطبيق

# 4. أعد البدء
# ✅ النتائج السابقة يجب أن تظهر في الرسوم البيانية
```

## 📊 قاعدة البيانات - ملخص

### البيانات المحفوظة في PostgreSQL:
```
✅ المشاريع (Projects)
✅ المهام (Tasks) 
✅ النتائج (Findings)
✅ سجلات التنفيذ (Task Logs)
✅ الذاكرة طويلة المدى (Memory Store)
✅ سجل الأدوات (Execution History)
✅ سجل الإجراءات (User Actions)
✅ جلسات المحادثة (Chat Sessions)
✅ رسائل المحادثة (Chat Messages)
```

### البيانات المحفوظة في Neo4j:
```
✅ الدومينات (Domains)
✅ عناوين IP (IPs)
✅ المنافذ (Ports)
✅ الخدمات (Services)
✅ الثغرات (Vulnerabilities)
✅ مسارات الهجوم (Attack Paths)
```

### البيانات المخزنة مؤقتاً في Redis:
```
✅ بيانات الجلسة (Session Data)
✅ ردود LLM المخزنة (LLM Cache)
✅ صفوف المهام (Job Queues)
✅ الحالة المؤقتة (Temporary State)
```

## ⚠️ استكشاف الأخطاء

### المشكلة: "Connection refused" على قاعدة البيانات

```bash
# ✅ الحل:
# 1. تحقق من أن الحاويات تعمل
podman ps | grep dragon

# 2. إذا لم تكن موجودة، شغّلها
podman compose -f podman-compose.yml up -d postgresql neo4j redis

# 3. انتظر 15-20 ثانية
sleep 20

# 4. حاول مرة أخرى
python startup_check.py
```

### المشكلة: "ModuleNotFoundError"

```bash
# ✅ الحل:
# تأكد من تثبيت المتطلبات
pip install -r requirements.txt
```

### المشكلة: البيانات لا تُحفظ

```bash
# ✅ تحقق من حالة قاعدة البيانات
python startup_check.py

# إذا كانت تظهر "❌ PostgreSQL"، حاول:
docker logs dragon-r1-postgres
# أو
podman logs dragon-r1-postgres
```

## 🛡️ الأمان (قبل الإنتاج)

قبل نشر النظام في الإنتاج، غيّر هذه القيم في `.env`:

```bash
# امين قوي
JWT_SECRET_KEY=your-super-secure-random-key-at-least-32-chars

# كلمة مرور قوية
ADMIN_PASSWORD=complex_password_with_numbers_and_symbols

# قاعدة بيانات آمنة
POSTGRES_PASSWORD=strong_postgres_password
NEO4J_PASSWORD=strong_neo4j_password
```

## 📈 الأداء

إذا كان النظام بطيء:

1. **تحقق من استخدام Redis:**
   ```bash
   podman exec dragon-r1-redis redis-cli INFO memory
   ```

2. **تحقق من استخدام PostgreSQL:**
   ```bash
   podman exec dragon-r1-postgres pg_stat_statements
   ```

3. **أضف المزيد من الاتصالات:**
   في `database_service.py`:
   ```python
   pool_size=20,      # زيادة من 10
   max_overflow=40,   # زيادة من 20
   ```

## 🎯 ملخص النقاط المهمة

✅ **3 قواعس بيانات فقط (لا زيادة)**
✅ **جميع البيانات محفوظة - تاريخ المحادثات والمشاريع والنتائج**
✅ **لا يوجد تكرار أو قواعس بيانات زائدة**
✅ **نظام التدهور الرشيق - يعمل حتى لو بعض البيانات معطلة**
✅ **الأداء محسّن مع connection pooling**

---

**تم التطوير بواسطة:** GitHub Copilot CLI
**الحالة:** ✅ جاهز للإنتاج
**آخر تحديث:** 2026-03-13
