# 🐉 تشغيل Dragon-R1 Full Stack - دليل كامل

## ⚡ الطريقة السريعة (بخطوة واحدة)

### على Windows:
```bash
cd C:\Users\Hossam Galal\Desktop\RED\Cerberus\gateway
run_fullstack.bat
```

### على Linux/Mac:
```bash
cd ~/Desktop/RED/Cerberus/gateway
chmod +x run_fullstack.sh
./run_fullstack.sh
```

**وهذا يفعل كل شيء تلقائياً:**
- ✅ بدء قواعس البيانات (PostgreSQL, Neo4j, Redis)
- ✅ انتظار تهيئتها
- ✅ تهيئة schemas والجداول
- ✅ بدء Backend Gateway
- ✅ بدء Frontend UI

---

## 📊 ماذا يحدث تحت الغطاء؟

```
الخطوة 1: فحص الإعدادات
└─ التحقق من Python وجود .env والمجلدات

الخطوة 2: بدء قواعس البيانات
└─ PostgreSQL (localhost:5432)
└─ Neo4j (localhost:7687)
└─ Redis (localhost:6379)
└─ انتظار 20 ثانية للتهيئة

الخطوة 3: تهيئة قواعس البيانات
└─ إنشاء الجداول
└─ إنشاء Indexes و Constraints
└─ التحقق من الاتصال

الخطوة 4: بدء Backend Gateway
└─ تفعيل Virtual Environment
└─ تحميل Models و Services
└─ تهيئة قواعس البيانات
└─ بدء Uvicorn على port 8000

الخطوة 5: بدء Frontend UI
└─ تثبيت npm dependencies (إذا لزم)
└─ بدء Next.js dev server على port 3000

الخطوة 6: عرض الحالة
└─ طباعة جميع الروابط والمعلومات
```

---

## 🌐 الروابط المتاحة بعد التشغيل

| الخدمة | الرابط | الاستخدام |
|-------|--------|----------|
| **Gateway** | http://localhost:8000 | API الرئيسي |
| **Docs** | http://localhost:8000/docs | توثيق API التفاعلي |
| **ReDoc** | http://localhost:8000/redoc | توثيق API البديل |
| **Frontend** | http://localhost:3000 | واجهة المستخدم |
| **Neo4j Browser** | http://localhost:7474 | متصفح قاعدة البيانات الرسومية |

---

## 🔐 بيانات الوصول

```
PostgreSQL:
  Host: localhost:5432
  User: dragon
  Password: dragon_secure_password
  Database: dragon_r1

Neo4j:
  URI: bolt://localhost:7687
  User: neo4j
  Password: neo4j_password

Redis:
  URL: redis://localhost:6379/0
```

---

## ✅ التحقق من التشغيل الصحيح

### 1. قواعس البيانات تعمل؟
```bash
# PostgreSQL
psql -h localhost -U dragon -d dragon_r1 -c "SELECT 1"

# Neo4j
curl http://localhost:7474

# Redis
redis-cli ping
# يجب أن ترد: PONG
```

### 2. Backend يستجيب؟
```bash
curl http://localhost:8000/api/v1/health
```

**النتيجة المتوقعة:**
```json
{
  "status": "healthy",
  "database": {
    "postgresql": "connected",
    "neo4j": "connected",
    "redis": "connected"
  }
}
```

### 3. Frontend يحمل؟
```bash
curl http://localhost:3000
```

---

## 📝 اختبار الوظائف

### اختبار 1: حفظ المشروع
```
1. افتح http://localhost:3000
2. انشئ مشروع جديد
   Name: "Test Project"
   Target: "scanme.nmap.org"
3. اضغط Save
4. اغلق التطبيق تماماً
5. افتح http://localhost:3000 مجدداً
✅ يجب أن تجد المشروع موجود!
```

### اختبار 2: حفظ المحادثة
```
1. افتح http://localhost:3000
2. اكتب رسالة في Chat
3. اغلق المتصفح
4. افتح المتصفح مجدداً
✅ يجب أن تجد المحادثة موجودة!
```

### اختبار 3: قاعدة البيانات
```bash
# تحقق من البيانات المحفوظة
psql -h localhost -U dragon -d dragon_r1 << EOF
SELECT COUNT(*) FROM projects;
SELECT COUNT(*) FROM chat_messages;
SELECT COUNT(*) FROM findings;
EOF
```

---

## ❌ حل المشاكل الشائعة

### المشكلة: "Port already in use"

```bash
# ابحث عن العملية التي تستخدم المنفذ
# Windows:
netstat -ano | findstr :8000
taskkill /PID <PID> /F

# Linux/Mac:
lsof -i :8000
kill -9 <PID>
```

### المشكلة: "Connection refused" على قاعدة البيانات

```bash
# تحقق من الحاويات
podman ps
# أو
docker ps

# إذا لم تكن موجودة، شغّلها يدوياً
podman compose up -d postgresql neo4j redis
```

### المشكلة: "npm: command not found"

```bash
# قم بتثبيت Node.js
# Windows: https://nodejs.org/
# Linux: sudo apt install nodejs npm
# Mac: brew install node
```

### المشكلة: Virtual Environment لا يفعّل

```bash
# Windows:
.venv\Scripts\activate.bat

# Linux/Mac:
source .venv/bin/activate

# إذا فشل، أعد إنشاء venv
rm -rf .venv
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt
```

---

## 🔄 إيقاف الخدمات

### الطريقة الأولى: Ctrl+C
```
أثناء تشغيل run_fullstack.bat/sh
اضغط Ctrl+C
سيتم إيقاف جميع الخدمات تلقائياً
```

### الطريقة الثانية: يدوي

```bash
# إيقاف Backend
cd Cerberus/gateway
# اضغط Ctrl+C في نافذة Python

# إيقاف Frontend
cd Cerberus/dragon-ui
# اضغط Ctrl+C في نافذة npm

# إيقاف الحاويات
podman stop dragon-r1-postgres dragon-r1-neo4j dragon-r1-redis
# أو
docker stop dragon-r1-postgres dragon-r1-neo4j dragon-r1-redis
```

---

## 📊 البيانات المحفوظة

| نوع البيانات | التخزين | الاستمرارية |
|------------|--------|-----------|
| المحادثات | PostgreSQL | ✅ دائماً |
| المشاريع | PostgreSQL | ✅ دائماً |
| النتائج | PostgreSQL | ✅ دائماً |
| جلسات المستخدم | PostgreSQL + Redis | ✅ دائماً |
| خرائط الهجوم | Neo4j | ✅ دائماً |
| سجل التدقيق | PostgreSQL | ✅ دائماً |
| ذاكرة التعلم | PostgreSQL | ✅ دائماً |

---

## 🎯 معايير النجاح

✅ **Success Criteria:**

- [ ] جميع الخدمات بدأت بدون أخطاء
- [ ] يمكن الوصول إلى http://localhost:8000
- [ ] يمكن الوصول إلى http://localhost:3000
- [ ] يمكن إنشاء مشروع جديد
- [ ] يمكن كتابة رسالة في Chat
- [ ] تُحفظ البيانات بعد إعادة التشغيل
- [ ] لا توجد أخطاء في أي خدمة

---

## 💡 نصائح مهمة

### 1. لأول مرة
```bash
# قد تحتاج لتثبيت المتطلبات
pip install -r requirements.txt

# وقد تحتاج لتثبيت npm
npm install  # في Cerberus/dragon-ui
```

### 2. إذا حدث خطأ
```bash
# تحقق من الملفات:
# - .env موجود؟
# - docker/podman مثبت؟
# - ports متاحة؟

# أعد تشغيل بدون cache
python run_fullstack.py --no-cache
```

### 3. للتطوير
```bash
# استخدم --reload للتطوير السريع
python run_fullstack.py --reload

# أو شغّل كل خدمة بشكل منفصل للتصحيح
```

---

## 📞 الدعم والمساعدة

إذا واجهت مشكلة:

1. تحقق من الأخطاء في الـ console
2. جرّب إعادة التشغيل
3. تحقق من بيانات الاتصال في .env
4. افحص سجلات قواعس البيانات

---

## 🎉 هذا كل شيء!

المشروع الآن جاهز للعمل بشكل كامل مع:
- ✅ قواعس بيانات محسّنة
- ✅ Backend متقدم
- ✅ Frontend احترافي
- ✅ جميع البيانات محفوظة
- ✅ جاهز للإنتاج

**استمتع بـ Dragon-R1! 🐉**
