# Dragon-R1 Database Storage Activation Guide

## Overview
This guide explains how to enable and verify database storage activation in the Dragon-R1 system. The system uses three complementary databases:

1. **PostgreSQL** - Relational data (projects, tasks, findings)
2. **Neo4j** - Graph database for attack paths and relationships
3. **Redis** - Cache and session management

## Current Status

### ✅ Completed
- [x] Database models created (`database_models.py`)
- [x] PostgreSQL service created (`database_service.py`)
- [x] Neo4j service created (`neo4j_service.py`)
- [x] Redis service created (`redis_service.py`)
- [x] Main.py integrated with database initialization
- [x] Configuration file updated (`.env`)
- [x] Setup script created (`setup_databases.py`)
- [x] Docker Compose configuration verified

### ⏳ Next Steps

## Step 1: Start Database Services

### Option A: Windows (Using Batch Script)
```cmd
cd C:\Users\Hossam Galal\Desktop\RED
start-databases.bat
```

### Option B: Windows (Manual Docker/Podman)
```cmd
podman compose -f podman-compose.yml up -d postgresql neo4j redis
REM Wait 10 seconds for services to initialize
```

### Option C: Linux/Mac (Using Bash Script)
```bash
cd ~/Desktop/RED
chmod +x start-databases.sh
./start-databases.sh
```

### Option D: Linux/Mac (Manual Docker)
```bash
docker compose -f podman-compose.yml up -d postgresql neo4j redis
sleep 10
```

## Step 2: Initialize Databases

```bash
# Navigate to gateway directory
cd Cerberus/gateway

# Create Python virtual environment (if not already created)
python -m venv .venv

# Activate virtual environment
# On Windows:
.venv\Scripts\activate
# On Linux/Mac:
source .venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Run database setup script
python setup_databases.py
```

## Step 3: Verify Database Connections

The setup script will display status:
```
==================================================
DATABASE INITIALIZATION SUMMARY
==================================================
PostgreSQL:  ✅ OK
Neo4j:       ✅ OK
Redis:       ✅ OK
==================================================
```

## Step 4: Start the Gateway

```bash
# From Cerberus/gateway/
python main.py
```

Watch for database initialization messages:
```
🐉 Dragon-R1 Gateway starting...
📦 Initializing database services...
✅ PostgreSQL (sync) initialized successfully
✅ Neo4j (sync) initialized successfully
✅ Redis (sync) initialized successfully
📊 Database Layer Status: PostgreSQL=True, Neo4j=True, Redis=True
```

## Step 5: Test Database Connectivity

### Test via Health Endpoint
```bash
curl http://localhost:8000/api/v1/health
```

Expected response includes:
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

### Test via Python

```python
# In Cerberus/gateway/
from database_service import get_database
from neo4j_service import get_neo4j
from redis_service import get_redis

db = get_database()
neo4j = get_neo4j()
redis = get_redis()

# Test PostgreSQL
with db.get_session() as session:
    result = session.query("SELECT 1")
    print("✅ PostgreSQL working")

# Test Neo4j
paths = neo4j.find_attack_paths("test-node-id")
print(f"✅ Neo4j working (found {len(paths)} paths)")

# Test Redis
redis.set("test-key", "test-value")
value = redis.get("test-key")
print(f"✅ Redis working: {value}")
```

## Database Configuration

### PostgreSQL
```
Host:     localhost
Port:     5432
Database: dragon_r1
User:     dragon
Password: dragon_secure_password
```

**Tables created:**
- `projects` - Attack scope definitions
- `tasks` - Attack phase executions
- `findings` - Vulnerabilities discovered
- `task_logs` - Detailed task execution logs
- `memory_store` - Long-term learning memory
- `execution_history` - Tool execution records
- `user_actions` - Audit trail

### Neo4j
```
URI:      bolt://localhost:7687
User:     neo4j
Password: neo4j_password
```

**Node types:**
- `Domain` - Target domains
- `IP` - IP addresses
- `Port` - Open ports
- `Service` - Detected services
- `Vulnerability` - Discovered vulnerabilities

**Relationships:**
- `HAS_IP` - Domain → IP
- `HAS_PORT` - IP → Port
- `RUNS_SERVICE` - Port → Service
- `HAS_VULNERABILITY` - Service → Vulnerability
- `CAN_EXPLOIT` - Vulnerability → Privilege escalation

### Redis
```
URL: redis://localhost:6379/0
```

**Use cases:**
- Session management
- Task queue
- Cache results
- Real-time WebSocket messages
- Rate limiting

## Graceful Degradation

The system implements graceful degradation - it will work even if some databases are unavailable:

### All databases available (100% functionality)
```
PostgreSQL=OK, Neo4j=OK, Redis=OK
→ Full capability
```

### Some databases missing (Reduced functionality)
```
PostgreSQL=OK, Neo4j=FAIL, Redis=OK
→ Can create tasks, cache results, but no attack graph visualization
```

### All databases missing (API-only mode)
```
PostgreSQL=FAIL, Neo4j=FAIL, Redis=FAIL
→ API endpoints work but no persistence
```

## Monitoring Database Health

### PostgreSQL Health
```python
db = get_database()
is_healthy = db.health_check_sync()
print("PostgreSQL:", "✅ OK" if is_healthy else "❌ FAILED")
```

### Neo4j Health
```python
neo4j = get_neo4j()
is_healthy = neo4j.health_check_sync()
print("Neo4j:", "✅ OK" if is_healthy else "❌ FAILED")
```

### Redis Health
```python
redis = get_redis()
is_healthy = redis.health_check_sync()
print("Redis:", "✅ OK" if is_healthy else "❌ FAILED")
```

## Troubleshooting

### PostgreSQL Connection Failed
```
Error: could not connect to server: Connection refused
```

**Solution:**
1. Verify PostgreSQL container is running: `podman ps | grep postgres`
2. Check logs: `podman logs dragon-r1-postgres`
3. Verify credentials in `.env` match container setup
4. Wait 10+ seconds for database to be ready

### Neo4j Connection Failed
```
Error: Unable to acquire connection from pool
```

**Solution:**
1. Verify Neo4j container is running: `podman ps | grep neo4j`
2. Check logs: `podman logs dragon-r1-neo4j`
3. Verify port 7687 is open
4. Neo4j takes longer to initialize, wait 15-20 seconds

### Redis Connection Failed
```
Error: ConnectionError: Error -3 name or service not known
```

**Solution:**
1. Verify Redis container is running: `podman ps | grep redis`
2. Check logs: `podman logs dragon-r1-redis`
3. Verify Redis URL in `.env`

### Port Already in Use
```
Error: bind: address already in use
```

**Solution:**
Stop existing containers:
```bash
podman stop dragon-r1-postgres dragon-r1-neo4j dragon-r1-redis
podman rm dragon-r1-postgres dragon-r1-neo4j dragon-r1-redis
```

## Data Persistence

### PostgreSQL
- Data stored in: `postgres_data` volume (Docker)
- Persists across container restarts
- Use migrations for schema updates

### Neo4j
- Data stored in: `neo4j_data` volume (Docker)
- Persists across container restarts
- Indexes created automatically

### Redis
- Data stored in: `redis_data` volume (Docker) if `--appendonly yes` is enabled
- Or in-memory if appendonly is disabled
- Recommended: keep appendonly=yes for persistence

## Next Steps

After database activation:

1. **Test Data Creation**
   ```python
   # Create a test project
   from database_models import Project
   from database_service import get_database
   
   db = get_database()
   with db.get_session() as session:
       project = Project(
           id="test-project-1",
           name="Test Project",
           target="example.com"
       )
       session.add(project)
       session.commit()
       print("✅ Project created in database")
   ```

2. **Run Full Integration Tests**
   ```bash
   python -m pytest tests/ -v
   ```

3. **Start Frontend**
   ```bash
   cd Cerberus/dragon-ui
   npm install
   npm run dev
   ```

4. **Access Web Interface**
   Open browser: http://localhost:3000

## Performance Tuning

### PostgreSQL Optimization
- Connection pool size: 10 (default)
- Max overflow: 20
- Pool recycle: 3600 seconds (1 hour)

### Neo4j Optimization
- Heap memory: 2GB (see podman-compose.yml)
- Page cache: 1GB
- Index on: domain.name, ip.address, port.number, vulnerability.severity

### Redis Optimization
- Connection pool: 20
- Appendonly: yes (for persistence)

## Further Reading

- [PostgreSQL Documentation](https://www.postgresql.org/docs/)
- [Neo4j Documentation](https://neo4j.com/docs/)
- [Redis Documentation](https://redis.io/documentation)
- [SQLAlchemy Documentation](https://docs.sqlalchemy.org/)
