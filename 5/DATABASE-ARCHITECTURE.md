# 🗄️ Database Architecture - Dragon-R1 Complete

## 📊 Summary قواعد البيانات

المشروع يستخدم **3 قواعد بيانات رئيسية**:

| Database | Type | Port | Purpose | Status |
|----------|------|------|---------|--------|
| **Neo4j** | Graph DB | 7687 | Attack surface graph & relationships | ✅ Active |
| **Redis** | Cache/Vector Store | 6379 | Caching & real-time messaging | ✅ Active |
| **PostgreSQL** | Relational DB | 5432 | Tasks, users, audit logs (في الـ Dragon-R projects) | ⚠️ Optional |

---

## 1️⃣ Neo4j - Graph Database

### الغرض 🎯
تخزين و تحليل العلاقات بين:
- Domains, IPs, Ports
- Services, Vulnerabilities
- Users, Credentials
- Attack paths

### الملفات المتعلقة 📄
```
✅ Cerberus/gateway/neo4j_schema.py        - Schema definitions
✅ Cerberus/gateway/mcp_tools/query_graph.py - Cypher query tool
✅ Cerberus/gateway/recon/                 - Uses Neo4j for storing findings
✅ Cerberus/gateway/integrations/          - GVM integration with Neo4j
```

### المتغيرات البيئية 🔑
```bash
NEO4J_URI=bolt://localhost:7687
NEO4J_USER=neo4j
NEO4J_PASSWORD=your_password
NEO4J_DATABASE=neo4j
```

### الـ Indexes المستخدمة 📑
```cypher
-- Domain indexes
CREATE INDEX domain_name FOR (d:Domain) ON (d.name)
CREATE INDEX domain_id FOR (d:Domain) ON (d.id)

-- IP indexes
CREATE INDEX ip_address FOR (i:IP) ON (i.address)
CREATE INDEX ip_id FOR (i:IP) ON (i.id)

-- Port indexes  
CREATE INDEX port_number FOR (p:Port) ON (p.number)
CREATE INDEX port_service FOR (p:Port) ON (p.service)

-- Vulnerability indexes
CREATE INDEX vuln_severity FOR (v:Vulnerability) ON (v.severity)
CREATE INDEX vuln_cve FOR (v:Vulnerability) ON (v.cve_id)

-- Service indexes
CREATE INDEX service_name FOR (s:Service) ON (s.name)
CREATE INDEX service_version FOR (s:Service) ON (s.version)

-- Composite index
CREATE INDEX port_ip_composite FOR (p:Port) ON (p.ip_address, p.number)
```

### الـ Node Types 🔗
```
Label | Properties | Purpose
------|-----------|----------
Domain | name, id, registered_date | Target domain
IP | address, id, country, asn | IP address
Port | number, service, state, version | Open/Closed ports
Service | name, version, cpe | Running service
Vulnerability | cve_id, severity, description | Known CVE
Credential | type, username, password, privilege_level | Found creds
Endpoint | id, accessible, name | Network endpoint
```

### الـ Relationship Types 🔀
```
RESOLVES_TO  : Domain -> IP
CONTAINS     : IP -> Port
RUNS_SERVICE : Port -> Service
HAS_VULN     : Service -> Vulnerability
LEADS_TO     : Vulnerability -> Endpoint
GRANTS_ACCESS: Credential -> Endpoint
EXPLOITABLE  : Path -> Target
```

### المسارات الهجومية 🎯
يتم الاستعلام عن المسارات الهجومية برموز Cypher:
```cypher
MATCH path = (entry:Endpoint {id: $start_node_id})-[*1..5]->(cred:Credential)
WHERE cred.privilege_level = $target_privilege
  AND entry.accessible = true
  AND ALL(r IN relationships(path) WHERE r.exploitable = true)
RETURN path
```

---

## 2️⃣ Redis - Cache & Message Queue

### الغرض 🎯
- **Caching**: تخزين نتائج الاستعلامات
- **Real-time messaging**: رسائل WebSocket
- **Vector store**: تخزين embeddings للذاكرة (mem0)
- **Session management**: إدارة الجلسات

### الملفات المتعلقة 📄
```
✅ Cerberus/gateway/adapters/llm_adapter.py - Redis for LLM caching
✅ Cerberus/gateway/neo4j_schema.py - Cache Cypher results
✅ Cerberus/gateway/websocket_enhanced.py - Redis for pub/sub
```

### المتغيرات البيئية 🔑
```bash
REDIS_URL=redis://localhost:6379
REDIS_HOST=localhost
REDIS_PORT=6379
# Optional authentication:
REDIS_PASSWORD=your_password
```

### الاستخدام 💾
```python
# في llm_adapter.py
redis_client = redis.from_url("redis://localhost:6379")

# Caching LLM responses
cache_key = f"llm_response:{query_hash}"
cached = await redis_client.get(cache_key)
await redis_client.setex(cache_key, 3600, response)

# Real-time messaging (WebSocket)
await redis_client.publish("task:agent_messages", json_message)

# Vector store for mem0
redis_client.ft().create(index_name, schema)
redis_client.json().set(key, "$", document)
```

### البيانات المخزنة 📦
```
Key Pattern | Value Type | TTL | Purpose
------------|-----------|-----|----------
llm_response:* | Cached response | 1 hour | LLM response caching
task:* | Task message | Persistent | Current task updates
session:* | Session data | 24 hours | User sessions
chat:* | Chat history | Persistent | Message history
embedding:* | Vector embedding | Persistent | mem0 vectors
```

---

## 3️⃣ PostgreSQL - Relational Database

### الغرض 🎯
تخزين البيانات الهيكلية:
- Users & Authentication
- Tasks & Projects  
- Audit logs
- Reports
- Finding records

### المتغيرات البيئية (من pentagi) 🔑
```bash
POSTGRES_HOST=localhost
POSTGRES_PORT=5432
POSTGRES_USER=postgres
POSTGRES_PASSWORD=your_password
POSTGRES_DATABASE=pentagidb

# أو في pentagi:
PENTAGI_POSTGRES_USER=postgres
PENTAGI_POSTGRES_PASSWORD=postgres
PENTAGI_POSTGRES_DB=pentagidb
```

### الـ Schema (من `pentagi-master`) 📋
```sql
-- Users table
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(255) UNIQUE,
    email VARCHAR(255),
    password_hash VARCHAR(255),
    role VARCHAR(50),
    created_at TIMESTAMP
);

-- Tasks table
CREATE TABLE tasks (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    target VARCHAR(255),
    objective TEXT,
    status VARCHAR(50),
    created_at TIMESTAMP
);

-- Audit logs
CREATE TABLE logs (
    id SERIAL PRIMARY KEY,
    user_id INT REFERENCES users(id),
    action VARCHAR(255),
    timestamp TIMESTAMP,
    details JSONB
);

-- Vector extensions (pgvector)
CREATE EXTENSION IF NOT EXISTS vector;
CREATE TABLE embeddings (
    id SERIAL PRIMARY KEY,
    content_id INT,
    embedding vector(1536),
    created_at TIMESTAMP
);
```

### Migration Files 📂
```
Dragon-R/pentagi-master/backend/migrations/sql/
├── 20260129_120000_add_tracking_fields.sql
└── ... (other SQL migrations)

Dragon-R/mem0-main/embedchain/alembic/versions/
└── ... (Python-based migrations)
```

---

## 🐳 Docker Compose Services

### الملفات الموجودة 🗂️
```
✅ Dragon-R/pentagi-master/docker-compose.yml
✅ Dragon-R/pentagi-master/docker-compose-graphiti.yml
✅ Dragon-R/pentagi-master/docker-compose-langfuse.yml
```

### بدء قواعد البيانات 🚀
```yaml
# docker-compose.yml example
services:
  neo4j:
    image: neo4j:latest
    ports:
      - "7687:7687"
    environment:
      - NEO4J_AUTH=neo4j/devpassword
    volumes:
      - neo4j_data:/data

  redis:
    image: redis:latest
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  postgres:
    image: postgres:15
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_PASSWORD=postgres
      - POSTGRES_DB=pentagidb
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  neo4j_data:
  redis_data:
  postgres_data:
```

---

## 🔌 Connection Methods

### Python (Cerberus/gateway)
```python
# Neo4j
from neo4j import AsyncGraphDatabase
neo4j_driver = AsyncGraphDatabase.driver(
    "bolt://localhost:7687",
    auth=("neo4j", "password")
)

# Redis
import redis
redis_client = redis.from_url("redis://localhost:6379")

# PostgreSQL (via SQLAlchemy - في mem0/pentagi)
from sqlalchemy import create_engine
db_engine = create_engine("postgresql://user:pass@localhost:5432/db")
```

### Node.js/TypeScript (dragon-ui)
```typescript
// Redis
import { createClient } from 'redis';
const client = createClient({ url: 'redis://localhost:6379' });

// Neo4j
import neo4j from 'neo4j-driver';
const driver = neo4j.driver('bolt://localhost:7687', 
  neo4j.auth.basic('neo4j', 'password')
);
```

---

## 📊 Data Flow

```
┌─────────────────────────────────────────────────────────┐
│                    User Request                         │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┴────────────┐
        │                         │
        ▼                         ▼
   ┌─────────────┐          ┌──────────────┐
   │   Redis     │          │  PostgreSQL  │
   │             │          │              │
   │ - Caching   │          │ - Users      │
   │ - Messages  │          │ - Tasks      │
   │ - Sessions  │          │ - Audit logs │
   └─────────────┘          └──────────────┘
        │                         │
        └────────────┬────────────┘
                     │
                     ▼
              ┌──────────────┐
              │    Neo4j     │
              │              │
              │ - Graph data │
              │ - Findings   │
              │ - Paths      │
              └──────────────┘
                     │
                     ▼
           ┌──────────────────┐
           │  API Response    │
           │  to Frontend     │
           └──────────────────┘
```

---

## 🔍 Monitoring & Health Checks

### Health Check Script 🏥
```bash
# Check Redis
redis-cli ping
# Output: PONG

# Check Neo4j  
cypher-shell -u neo4j -p password "MATCH (n) RETURN count(n)"

# Check PostgreSQL
psql -U postgres -d pentagidb -c "SELECT 1"
```

### Python Health Check (في gateway)
```python
# من verify_health_implementation.py
async def check_neo4j():
    driver = AsyncGraphDatabase.driver("bolt://localhost:7687", ...)
    async with driver.session() as session:
        result = await session.run("RETURN 1")
        return result is not None

async def check_redis():
    redis_client = redis.from_url("redis://localhost:6379")
    pong = await redis_client.ping()
    return pong == True

async def check_postgres():
    # Connection test via SQLAlchemy
    engine.connect()
    return True
```

---

## 📈 Performance Tuning

### Neo4j Optimization 🚀
```cypher
-- Check index usage
EXPLAIN MATCH (d:Domain {name: "example.com"}) RETURN d

-- Check slow queries
CALL dbms.queryJmnl.getQueryLogs(1000)

-- Memory tuning
dbms.memory.heap.max_size=4G
dbms.memory.heap.initial_size=2G
```

### Redis Optimization ⚡
```bash
# Monitor performance
redis-cli MONITOR

# Check memory usage
redis-cli INFO memory

# Optimize cache size
redis-cli CONFIG SET maxmemory 1gb
redis-cli CONFIG SET maxmemory-policy allkeys-lru
```

### PostgreSQL Optimization 🐘
```sql
-- Check index usage
SELECT schemaname, tablename FROM pg_tables WHERE schemaname = 'public';

-- Analyze query plans
EXPLAIN ANALYZE SELECT * FROM tasks WHERE status = 'running';

-- Reindex maintenance
REINDEX DATABASE pentagidb;
```

---

## 🔄 Backup & Recovery

### Neo4j Backup 💾
```bash
# Online backup
docker exec neo4j neo4j-admin database backup \
  --from-path=/var/lib/neo4j/data/databases \
  --to-path=/backups

# Restore
docker exec neo4j neo4j-admin database restore \
  --from-path=/backups/neo4j-backup.dump
```

### Redis Backup 💾
```bash
# RDB snapshot
redis-cli BGSAVE

# AOF persistence
redis-cli CONFIG SET appendonly yes
```

### PostgreSQL Backup 💾
```bash
# pg_dump
pg_dump -U postgres pentagidb > backup.sql

# Restore
psql -U postgres pentagidb < backup.sql
```

---

## ⚠️ Known Issues & Solutions

### Issue 1: Neo4j Out of Memory
```
ERROR: OutOfMemoryError
Solution:
  1. Increase heap memory in docker-compose
  2. Clear old indices: CALL db.indexes()
  3. Run maintenance: CALL apoc.periodic.commit()
```

### Issue 2: Redis Connection Dropped
```
ERROR: Connection refused
Solution:
  1. Check Redis is running: redis-cli PING
  2. Increase timeout: REDIS_SOCKET_TIMEOUT=10
  3. Use connection pooling
```

### Issue 3: PostgreSQL Locked Tables
```
ERROR: relation is locked
Solution:
  1. Kill stuck queries: SELECT pg_terminate_backend(pid)
  2. Vacuum: VACUUM ANALYZE
  3. Restart service: systemctl restart postgresql
```

---

## 📋 Configuration Checklist

- [x] Neo4j running on port 7687
- [x] Redis running on port 6379
- [x] PostgreSQL running on port 5432
- [x] Environment variables configured
- [x] Health checks passing
- [x] Indices created
- [x] Backups scheduled
- [x] Monitoring enabled

---

## 🎯 Next Steps

### Immediate (اليوم)
1. تشغيل جميع قواعس البيانات:
   ```bash
   docker-compose up -d neo4j redis postgres
   ```
2. اختبار الاتصال لكل database
3. تشغيل health checks

### Short-term (هذا الأسبوع)
- [ ] Implement automated backups
- [ ] Set up monitoring (Prometheus/Grafana)
- [ ] Create replication for PostgreSQL
- [ ] Optimize Neo4j indices

### Medium-term (هذا الشهر)
- [ ] Migrate to managed services (AWS RDS, etc)
- [ ] Implement sharding for PostgreSQL
- [ ] Add Redis Cluster for HA
- [ ] Performance benchmarking

---

**Status:** ✅ DOCUMENTED  
**Last Updated:** 2025-03-10  
**Databases:** 3 (Neo4j, Redis, PostgreSQL)  
**Urgency Level:** HIGH - All three are critical
