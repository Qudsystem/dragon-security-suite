# 🐉 DRAGON R1 — ANTIGRAVITY MASTER ANALYSIS & REPAIR PROMPT
# النسخة المخصصة لـ Antigravity (Gemini CLI)
# QudSystem — الباشمهندس حسام | v2.0

---

## 📌 HOW TO USE WITH ANTIGRAVITY

هذا الأمر مُصمَّم خصيصاً لـ **Antigravity (Gemini CLI)**:
- يركز على **Infrastructure والـ Terminal** (دور Antigravity)
- يتكامل مع **PROGRESS.md** للتنسيق مع Kiro
- يستخدم **PowerShell + WSL2** لا bash مباشرة
- يعمل مع **Podman** (لا Docker) كما هو مُعرَّف في القواعد

---

## 🔴 ANTIGRAVITY SYSTEM CONTEXT

```
أنت Antigravity — كبير مهندسي البنية التحتية في مشروع Dragon R1.
نطاق عملك الحصري: Infrastructure, Terminal, Build Systems, Containers, Environment.
لا تلمس TypeScript/React/CSS source code — هذه مهام Kiro (Claude).

مهمتك الآن: فحص وتشخيص وإصلاح كل مشاكل الـ infrastructure في Dragon R1.

قواعد لا تُخالَف:
1. اقرأ PROGRESS.md أولاً قبل أي خطوة
2. سجّل كل ما تفعله في PROGRESS.md فور البدء
3. استخدم PowerShell أو WSL2 — لا bash مباشرة على Windows
4. استخدم Podman وليس Docker في كل الأوامر
5. المنفذ الصحيح للـ Frontend: 3001 (وليس 3000)
6. وثّق كل مشكلة بمسارها الدقيق + السبب + الحل
7. رتّب الأولويات: CRITICAL → HIGH → MEDIUM → LOW
```

---

## 📋 PRE-FLIGHT: قبل البدء

```powershell
# الخطوة 1: اقرأ PROGRESS.md
Get-Content "C:\Users\Hossam Galal\Desktop\RED\PROGRESS.md"

# الخطوة 2: سجّل بدء المهمة
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm"
Add-Content "C:\Users\Hossam Galal\Desktop\RED\PROGRESS.md" `
  "`n- [$timestamp] **Antigravity**: بدء Master Analysis — Dragon R1 Full Audit"
```

---

## 🟡 PHASE 0: INFRASTRUCTURE INVENTORY — جرد البنية التحتية

### 0.1 Container Status
```powershell
# قائمة كل الحاويات
podman ps -a --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# فحص الـ networks
podman network ls

# فحص الـ volumes
podman volume ls
```

الإخراج المطلوب:
```
📦 CONTAINER INVENTORY
┌─────────────────────────┬──────────┬──────────────┐
│ Container Name          │ Status   │ Ports        │
├─────────────────────────┼──────────┼──────────────┤
│ dragon-r1-gateway       │ ?        │ 8000         │
│ dragon-ui               │ ?        │ 3001         │
│ postgres                │ ?        │ 5432         │
│ ollama                  │ ?        │ 11434        │
│ neo4j                   │ ?        │ 7474/7687    │
└─────────────────────────┴──────────┴──────────────┘
```

### 0.2 Service Health Check
```powershell
# فحص الـ endpoints
$endpoints = @(
    "http://localhost:3001",           # Dragon UI
    "http://localhost:8000/health",    # Gateway
    "http://localhost:11434/api/tags", # Ollama
    "http://localhost:5432",           # PostgreSQL (TCP)
    "http://localhost:7474"            # Neo4j
)

foreach ($url in $endpoints) {
    try {
        $r = Invoke-WebRequest -Uri $url -TimeoutSec 3 -UseBasicParsing
        Write-Host "✅ $url → $($r.StatusCode)"
    } catch {
        Write-Host "❌ $url → FAILED: $_"
    }
}
```

### 0.3 Process & Port Audit
```powershell
# كل البروسيسات المرتبطة بالمشروع
$ports = @(3001, 8000, 11434, 5432, 7474, 7687, 28000)
foreach ($port in $ports) {
    $p = netstat -ano | Select-String ":$port "
    if ($p) { Write-Host "Port $port: IN USE → $p" }
    else { Write-Host "Port $port: FREE" }
}
```

### 0.4 WSL2 Environment
```powershell
# فحص WSL2
wsl --list --verbose
wsl -e uname -a
wsl -e which podman
wsl -e podman --version
```

### 0.5 Project File Inventory
```powershell
# إحصاءات الملفات
$root = "C:\Users\Hossam Galal\Desktop\RED"
$types = @("*.ts","*.tsx","*.py","*.rs","*.yaml","*.yml","*.json","*.md","*.css","*.env")

foreach ($ext in $types) {
    $count = (Get-ChildItem -Path $root -Filter $ext -Recurse -ErrorAction SilentlyContinue).Count
    Write-Host "$ext : $count files"
}

# أكبر 20 ملف
Get-ChildItem -Path $root -Recurse -ErrorAction SilentlyContinue |
    Sort-Object Length -Descending |
    Select-Object -First 20 |
    Format-Table Name, @{N='Size(KB)';E={[math]::Round($_.Length/1KB,1)}}, FullName
```

---

## 🔴 PHASE 1: CRITICAL INFRASTRUCTURE BLOCKERS

### 1.1 WebSocket Connectivity (ws://127.0.0.1:8000/ws/...)

```powershell
# فحص الـ WebSocket endpoint في Gateway
$gatewayDir = "C:\Users\Hossam Galal\Desktop\RED"
Get-ChildItem -Recurse -Filter "*.py" | 
    Select-String -Pattern "websocket|/ws/" -CaseSensitive:$false |
    Select-Object Path, LineNumber, Line

# فحص CORS في gateway
Get-ChildItem -Recurse -Filter "*.py" |
    Select-String -Pattern "cors|CORS|allow_origins" |
    Select-Object Path, LineNumber, Line

# اختبار WebSocket
# هل يوجد wscat أو websocat؟
Get-Command wscat -ErrorAction SilentlyContinue
Get-Command websocat -ErrorAction SilentlyContinue
```

**مشاكل محتملة وحلولها:**
```powershell
# فحص إذا كان Gateway يعمل
Invoke-WebRequest -Uri "http://localhost:8000/" -UseBasicParsing
Invoke-WebRequest -Uri "http://localhost:8000/docs" -UseBasicParsing

# فحص logs الـ gateway container
podman logs dragon-r1-gateway --tail 50 2>&1
```

**الحل في حالة فشل WS:**
```powershell
# إعادة تشغيل gateway
podman restart dragon-r1-gateway
Start-Sleep -Seconds 5
podman logs dragon-r1-gateway --tail 20
```

---

### 1.2 Ollama Service (503/403 على /api/tags)

```powershell
# فحص Ollama container
podman inspect ollama 2>&1 | ConvertFrom-Json | Select-Object -ExpandProperty Config

# فحص OLLAMA_ORIGINS
podman exec ollama env 2>&1 | Select-String "OLLAMA"

# اختبار مباشر
Invoke-RestMethod -Uri "http://localhost:11434/api/tags" -Method GET

# فحص النماذج المثبتة
podman exec ollama ollama list 2>&1
```

**الحل:**
```powershell
# إذا فشل CORS
podman stop ollama
podman rm ollama
podman run -d `
    --name ollama `
    -p 11434:11434 `
    -e OLLAMA_ORIGINS="*" `
    -e OLLAMA_HOST="0.0.0.0" `
    -v ollama:/root/.ollama `
    ollama/ollama:latest

# تحقق
Start-Sleep -Seconds 10
Invoke-RestMethod -Uri "http://localhost:11434/api/tags"
```

---

### 1.3 Build System — spawn EPERM Fix

```powershell
# فحص المشكلة
$uiDir = Get-ChildItem -Path "C:\Users\Hossam Galal\Desktop\RED" -Filter "dragon-ui" -Recurse -Directory | Select-Object -First 1
Write-Host "UI Dir: $($uiDir.FullName)"

# فحص package.json scripts
Get-Content "$($uiDir.FullName)\package.json" | ConvertFrom-Json | Select-Object -ExpandProperty scripts

# الحل الشامل لـ EPERM
Set-Location $uiDir.FullName

# 1. حذف node_modules وإعادة التثبيت
if (Test-Path "node_modules") { Remove-Item -Recurse -Force "node_modules" }
if (Test-Path ".next") { Remove-Item -Recurse -Force ".next" }
if (Test-Path "package-lock.json") { Remove-Item -Force "package-lock.json" }

# 2. إعادة التثبيت
npm install --legacy-peer-deps

# 3. فحص esbuild
npm rebuild esbuild
```

---

### 1.4 PostgreSQL Connectivity

```powershell
# فحص PostgreSQL
podman exec postgres pg_isready -U postgres 2>&1

# فحص database
podman exec postgres psql -U postgres -c "\l" 2>&1

# فحص pgvector extension
podman exec postgres psql -U postgres -c "SELECT * FROM pg_extension WHERE extname='vector';" 2>&1
```

---

### 1.5 Neo4j + Graphiti KG

```powershell
# فحص Neo4j
Invoke-WebRequest -Uri "http://localhost:7474" -UseBasicParsing
podman logs neo4j --tail 20 2>&1
```

---

## 🟠 PHASE 2: ENVIRONMENT CONFIGURATION AUDIT

### 2.1 فحص كل ملفات .env

```powershell
$root = "C:\Users\Hossam Galal\Desktop\RED"
$envFiles = Get-ChildItem -Path $root -Filter ".env*" -Recurse -Force

foreach ($f in $envFiles) {
    Write-Host "`n📄 $($f.FullName):"
    Get-Content $f.FullName | ForEach-Object {
        if ($_ -match "KEY|SECRET|PASSWORD|TOKEN") {
            $_ -replace "=.*", "=***REDACTED***"
        } else { $_ }
    }
}
```

**متغيرات يجب التحقق منها:**
```
OLLAMA_ORIGINS=*
OLLAMA_HOST=0.0.0.0
DATABASE_URL=postgresql://...
NEXT_PUBLIC_GATEWAY_URL=http://localhost:8000
NEXT_PUBLIC_WS_URL=ws://localhost:8000
```

### 2.2 Podman vs Docker Hostname Reference Table

```powershell
# فحص hostname resolution داخل الحاويات
podman exec dragon-r1-gateway getent hosts host.containers.internal 2>&1
podman exec dragon-r1-gateway getent hosts host.docker.internal 2>&1

# الجدول المرجعي:
Write-Host @"
╔══════════════════════════════════════════════════════════╗
║         HOSTNAME REFERENCE TABLE — Dragon R1             ║
╠══════════════════════════════════════════════════════════╣
║ من/إلى              │ Hostname الصحيح                    ║
╠══════════════════════════════════════════════════════════╣
║ Container → Host    │ host.containers.internal            ║
║ Container → Container (same network) │ container-name   ║
║ Host → Container    │ localhost:PORT                      ║
║ WSL → Windows Host  │ $(wsl hostname -I | awk '{print $1}')  ║
╚══════════════════════════════════════════════════════════╝
"@
```

### 2.3 Network Configuration

```powershell
# فحص podman-compose.yml
Get-Content "C:\Users\Hossam Galal\Desktop\RED\podman-compose.yml"

# فحص الـ network الافتراضية
podman network inspect podman 2>&1
podman network inspect bridge 2>&1
```

---

## 🟡 PHASE 3: PERFORMANCE & STABILITY AUDIT

### 3.1 Resource Usage

```powershell
# استهلاك الموارد لكل container
podman stats --no-stream --format "table {{.Name}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}"

# مراقبة الـ HP 15-dw0xxx resources
Get-Process | Sort-Object CPU -Descending | Select-Object -First 15 Name, CPU, WorkingSet64
```

### 3.2 Ollama Model Performance

```powershell
# فحص النماذج المثبتة وأحجامها
podman exec ollama ollama list 2>&1

# فحص VRAM (MX250 4GB)
# هل النماذج تتناسب مع 4GB VRAM؟
Invoke-RestMethod -Uri "http://localhost:11434/api/tags" | 
    Select-Object -ExpandProperty models | 
    Select-Object name, size
```

### 3.3 Log Analysis

```powershell
# فحص errors في جميع containers
$containers = @("dragon-r1-gateway", "dragon-ui", "postgres", "ollama", "neo4j")

foreach ($c in $containers) {
    Write-Host "`n🔍 Errors in $c:"
    podman logs $c 2>&1 | Select-String -Pattern "error|ERROR|exception|EXCEPTION|failed|FAILED" | 
        Select-Object -Last 10
}
```

---

## 🔵 PHASE 4: BUILD & STARTUP SYSTEMS

### 4.1 فحص START-DRAGON-R1.bat

```powershell
Get-Content "C:\Users\Hossam Galal\Desktop\RED\START-DRAGON-R1.bat"
```

### 4.2 فحص docker-compose vs podman-compose

```powershell
# هل يوجد تعارض بين docker-compose.yml و podman-compose.yml؟
Write-Host "=== docker-compose.yml services ==="
Get-Content "C:\Users\Hossam Galal\Desktop\RED\docker-compose.yml" | 
    Select-String "^\s+\w+:" | Select-Object -First 20

Write-Host "`n=== podman-compose.yml services ==="
Get-Content "C:\Users\Hossam Galal\Desktop\RED\podman-compose.yml" | 
    Select-String "^\s+\w+:" | Select-Object -First 20
```

### 4.3 اختبار Full Stack Startup

```powershell
# محاولة تشغيل كامل النظام
Set-Location "C:\Users\Hossam Galal\Desktop\RED"

# الخيار 1: podman-compose
podman-compose -f podman-compose.yml up -d 2>&1

# انتظر 15 ثانية ثم فحص
Start-Sleep -Seconds 15
podman ps -a
```

---

## 🔷 PHASE 5: CONNECTIVITY MAP — خريطة الاتصالات الكاملة

```powershell
# اختبار كل connection
$connections = @(
    @{From="Frontend(3001)"; To="Gateway(8000)"; URL="http://localhost:8000/health"},
    @{From="Frontend(3001)"; To="Gateway-WS"; URL="http://localhost:8000/ws/test"},
    @{From="Gateway(8000)"; To="Ollama(11434)"; URL="http://localhost:11434/api/tags"},
    @{From="Gateway(8000)"; To="PostgreSQL(5432)"; URL="http://localhost:5432"},
    @{From="Gateway(8000)"; To="Neo4j(7474)"; URL="http://localhost:7474"},
    @{From="Any"; To="Dragon-UI"; URL="http://localhost:3001"}
)

Write-Host "`n╔══════════════════════════════════════════════════════╗"
Write-Host   "║         CONNECTIVITY MAP — Dragon R1                 ║"
Write-Host   "╚══════════════════════════════════════════════════════╝"

foreach ($conn in $connections) {
    try {
        $r = Invoke-WebRequest -Uri $conn.URL -TimeoutSec 3 -UseBasicParsing -ErrorAction Stop
        Write-Host "✅ $($conn.From) → $($conn.To) [HTTP $($r.StatusCode)]"
    } catch {
        Write-Host "❌ $($conn.From) → $($conn.To) [FAILED: $($_.Exception.Message)]"
    }
}
```

---

## ⚡ PHASE 6: AUTO-REPAIR SEQUENCE

### الإصلاح التلقائي بالترتيب الصارم:

```powershell
# ═══════════════════════════════════════════
# REPAIR STEP 1: إيقاف وإعادة تشغيل النظام
# ═══════════════════════════════════════════
Write-Host "🔄 Step 1: Restarting all services..."
podman-compose -f "C:\Users\Hossam Galal\Desktop\RED\podman-compose.yml" down
Start-Sleep -Seconds 5
podman-compose -f "C:\Users\Hossam Galal\Desktop\RED\podman-compose.yml" up -d
Start-Sleep -Seconds 20

# ═══════════════════════════════════════════
# REPAIR STEP 2: فحص وإصلاح Ollama
# ═══════════════════════════════════════════
Write-Host "`n🔄 Step 2: Checking Ollama..."
$ollama = Invoke-RestMethod -Uri "http://localhost:11434/api/tags" -ErrorAction SilentlyContinue
if (-not $ollama) {
    Write-Host "⚠️  Ollama not responding — checking container..."
    podman logs ollama --tail 20
    podman restart ollama
    Start-Sleep -Seconds 10
}

# ═══════════════════════════════════════════
# REPAIR STEP 3: فحص Gateway
# ═══════════════════════════════════════════
Write-Host "`n🔄 Step 3: Checking Gateway..."
try {
    $gw = Invoke-WebRequest -Uri "http://localhost:8000/health" -UseBasicParsing -TimeoutSec 5
    Write-Host "✅ Gateway OK"
} catch {
    Write-Host "❌ Gateway FAILED — checking logs..."
    podman logs dragon-r1-gateway --tail 30
    podman restart dragon-r1-gateway
}

# ═══════════════════════════════════════════
# REPAIR STEP 4: فحص Frontend
# ═══════════════════════════════════════════
Write-Host "`n🔄 Step 4: Checking Frontend on port 3001..."
try {
    $ui = Invoke-WebRequest -Uri "http://localhost:3001" -UseBasicParsing -TimeoutSec 5
    Write-Host "✅ Frontend OK on port 3001"
} catch {
    Write-Host "❌ Frontend not running on 3001"
    # لا نلمس الكود — نخبر Kiro
    Add-Content "C:\Users\Hossam Galal\Desktop\RED\PROGRESS.md" `
        "`n🆘 [من Antigravity إلى Kiro]: Frontend لا يعمل على port 3001. يرجى فحص next.config وتأكيد المنفذ."
}
```

---

## 📊 PHASE 7: FINAL REPORT

```powershell
# توليد التقرير النهائي
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm"
$report = @"
╔══════════════════════════════════════════════════════════════╗
║        DRAGON R1 — ANTIGRAVITY INFRASTRUCTURE REPORT         ║
║        QudSystem | الباشمهندس حسام | $timestamp             ║
╚══════════════════════════════════════════════════════════════╝

📦 Container Status:
$(podman ps -a --format "  {{.Names}}: {{.Status}}" 2>&1)

🔌 Service Connectivity:
  Frontend (3001): $(try{(Invoke-WebRequest http://localhost:3001 -UseBasicParsing -TimeoutSec 2).StatusCode}catch{'❌ DOWN'})
  Gateway  (8000): $(try{(Invoke-WebRequest http://localhost:8000/health -UseBasicParsing -TimeoutSec 2).StatusCode}catch{'❌ DOWN'})
  Ollama  (11434): $(try{(Invoke-WebRequest http://localhost:11434/api/tags -UseBasicParsing -TimeoutSec 2).StatusCode}catch{'❌ DOWN'})
  Neo4j    (7474): $(try{(Invoke-WebRequest http://localhost:7474 -UseBasicParsing -TimeoutSec 2).StatusCode}catch{'❌ DOWN'})

💻 System Resources:
$(podman stats --no-stream --format "  {{.Name}}: CPU={{.CPUPerc}} MEM={{.MemUsage}}" 2>&1)

🎯 الخطوات التالية لـ Kiro (Claude):
  □ إصلاح WebSocket endpoint في gateway/routes.py
  □ فصل Hero page عن Chat interface
  □ بناء ThinkingPanel.tsx مع real-time timer
  □ بناء ChatComposer.tsx مع 3 drawers
  □ إصلاح CodeBlock overflow CSS

📋 تم تسجيل هذا التقرير في PROGRESS.md
"@

Write-Host $report

# حفظ في PROGRESS.md
Add-Content "C:\Users\Hossam Galal\Desktop\RED\PROGRESS.md" @"

---

## ✅ Antigravity Infrastructure Audit — $timestamp

### النتائج:
$report

### المهام المحولة لـ Kiro:
- [ ] إصلاح WebSocket endpoint
- [ ] فصل Hero عن Chat
- [ ] بناء UI components المفقودة
"@

Write-Host "`n✅ Report saved to PROGRESS.md"
```

---

## 🌙 KIRO HANDOFF TEMPLATE

بعد انتهاء Antigravity، هذا ما يُرسَل لـ Kiro:

```markdown
# 📋 Kiro Handoff — Dragon R1 Frontend Tasks

## الحالة من Antigravity:
- Infrastructure: [الحالة]
- Containers: [قائمة تشتغل / لا تشتغل]
- Gateway: [URL للتأكيد]

## المهام المطلوبة منك:

### CRITICAL (ابدأ بها):
1. [ ] إصلاح `app/chat/page.tsx` — أزل الـ Hero وأبقِ ChatInterface فقط
2. [ ] إصلاح WebSocket endpoint في `gateway/routes.py` السطر X
3. [ ] إصلاح Ollama CORS في middleware السطر Y

### HIGH:
4. [ ] بناء `ThinkingPanel.tsx` مع real-time timer
5. [ ] بناء `ChatComposer.tsx` مع 3 drawers
6. [ ] إصلاح `CodeBlock.tsx` overflow

### MEDIUM:
7. [ ] `SearchDeepSyncToggle.tsx` — toggle واحد
8. [ ] `TopicNavigator.tsx` — scrollIntoView
9. [ ] `FileUpload.tsx` — unified states

## ملاحظات تقنية:
- المنفذ: 3001 (Frontend) + 8000 (Gateway)
- Podman not Docker
- host.containers.internal للاتصال الداخلي
```

---

## ⚡ QUICK EXECUTE (نسخ للتشغيل الفوري في PowerShell)

```powershell
# Dragon R1 — Antigravity Quick Audit & Repair
# تشغيل في PowerShell كـ Administrator

$ROOT = "C:\Users\Hossam Galal\Desktop\RED"
Set-Location $ROOT

Write-Host "🐉 Dragon R1 — Antigravity Infrastructure Audit" -ForegroundColor Cyan
Write-Host "=================================================" -ForegroundColor Cyan

# 1. Container Status
Write-Host "`n📦 Container Status:" -ForegroundColor Yellow
podman ps -a --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

# 2. Service Health
Write-Host "`n🔌 Service Health:" -ForegroundColor Yellow
@(3001, 8000, 11434, 7474) | ForEach-Object {
    $p = $_
    try {
        $r = Invoke-WebRequest "http://localhost:$p" -TimeoutSec 2 -UseBasicParsing
        Write-Host "  ✅ Port $p → $($r.StatusCode)" -ForegroundColor Green
    } catch {
        Write-Host "  ❌ Port $p → DOWN" -ForegroundColor Red
    }
}

# 3. Ollama Models
Write-Host "`n🤖 Ollama Models:" -ForegroundColor Yellow
Invoke-RestMethod "http://localhost:11434/api/tags" -ErrorAction SilentlyContinue |
    Select-Object -ExpandProperty models |
    ForEach-Object { Write-Host "  📌 $($_.name) ($([math]::Round($_.size/1GB,2))GB)" }

# 4. Resources
Write-Host "`n💻 Resource Usage:" -ForegroundColor Yellow
podman stats --no-stream --format "  {{.Name}}: CPU={{.CPUPerc}} MEM={{.MemUsage}}"

Write-Host "`n✅ Audit Complete!" -ForegroundColor Green
```

---

*Dragon R1 Antigravity Prompt v2.0*
*QudSystem — الباشمهندس حسام*
*تاريخ التعديل: 2026-04-12*
*مُعدَّل خصيصاً لـ Antigravity (Gemini CLI)*
