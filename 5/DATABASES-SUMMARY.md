# 🗄️ Database Summary - Dragon-R1

## ✅ النتيجة: المشروع به **3 قواعس بيانات رئيسية**

### 1️⃣ **Neo4j** - Graph Database 🔗
- **Port:** 7687 (Bolt)
- **غرضها:** تخزين العلاقات بين الأهداف و الثغرات (Domains, IPs, Ports, Services, Vulnerabilities)
- **الملفات:** `gateway/neo4j_schema.py`, `gateway/mcp_tools/query_graph.py`
- **المتغيرات:**
  ```bash
  NEO4J_URI=bolt://localhost:7687
  NEO4J_USER=neo4j
  NEO4J_PASSWORD=devpassword
  ```

### 2️⃣ **Redis** - Cache & Message Queue ⚡
- **Port:** 6379
- **غرضها:** تخزين مؤقت للنتائج + رسائل WebSocket Real-time + Vector store
- **الملفات:** `gateway/adapters/llm_adapter.py`, `gateway/websocket_enhanced.py`
- **المتغيرات:**
  ```bash
  REDIS_URL=redis://localhost:6379
  ```

### 3️⃣ **PostgreSQL** - Relational Database 🐘
- **Port:** 5432
- **غرضها:** تخزين البيانات الهيكلية (Users, Tasks, Audit logs)
- **الملفات:** في `Dragon-R/pentagi-master/backend/migrations/`
- **المتغيرات:**
  ```bash
  POSTGRES_HOST=localhost
  POSTGRES_PORT=5432
  POSTGRES_USER=postgres
  POSTGRES_PASSWORD=postgres
  POSTGRES_DATABASE=pentagidb
  ```

---

## 🚀 تشغيل قواعس البيانات

### الطريقة الأسهل (One-Command):

#### Windows:
```bash
setup-databases.bat
```

#### Linux/Mac:
```bash
chmod +x setup-databases.sh
./setup-databases.sh
```

---

## ✅ الملفات المنشأة الآن

| الملف | الوصف |
|------|-------|
| `DATABASE-ARCHITECTURE.md` | 📚 توثيق شامل لجميع قواعس البيانات |
| `setup-databases.sh` | 🐧 Script تشغيل قواعس البيانات (Linux/Mac) |
| `setup-databases.bat` | 🪟 Script تشغيل قواعس البيانات (Windows) |

---

## 📊 جدول سريع

```
Database     | Type          | Port | Purpose                  | Status
-------------|---------------|------|--------------------------|--------
Neo4j        | Graph DB      | 7687 | Attack surface graph     | ✅ Active
Redis        | Cache/Queue   | 6379 | Caching & messaging      | ✅ Active
PostgreSQL   | Relational    | 5432 | Tasks & users            | ✅ Active (Optional)
```

---

## 🔌 Quick Connection Info

### عند الحاجة للاتصال من Terminal:

```bash
# Neo4j
cypher-shell -u neo4j -p devpassword

# Redis
redis-cli

# PostgreSQL
psql -U postgres -d pentagidb
```

---

## ⚠️ ملاحظات مهمة

1. **Neo4j** يحتاج على الكثير من الـ Memory - قد يأخذ 30-60 ثانية للتشغيل
2. **Redis** lightweight - يشتغل بسرعة
3. **PostgreSQL** مختياري - يمكنك تشغيل بدونه إذا لم تكن تستخدم pentagi-master

---

## 🎯 التالي

1. **تشغيل قواعس البيانات:**
   ```bash
   setup-databases.bat  # Windows
   # أو
   ./setup-databases.sh  # Linux/Mac
   ```

2. **اختبار الاتصال:**
   - Neo4j: http://localhost:7474
   - Redis: `redis-cli ping` ➜ PONG
   - Postgres: `psql -U postgres`

3. **إضافة البيانات:**
   اقرأ `DATABASE-ARCHITECTURE.md` للحصول على أوامر إضافة البيانات

---

**وقت التشغيل:** ~2 دقيقة  
**متطلبات:** Docker + Docker-Compose  
**حجم البيانات:** ~1GB (قابل للنمو)
