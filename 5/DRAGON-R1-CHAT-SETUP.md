# 🎯 Dragon-R1 Chat - تشغيل الواجهة الحقيقية

## الواجهة الحقيقية

الواجهة الحقيقية للـ Dragon-R1 Chat هي **Next.js Web App** على:
- **URL**: `http://localhost:3000/chat`
- **Port**: 3000 (Frontend)
- **Backend**: Port 8000 (Gateway API + WebSocket)

> **ملاحظة:** الـ Streamlit interface على port 8501 هي واجهة اختبار فقط.

---

## 🚀 تشغيل سريع

### Option 1: استخدام Script التشغيل (أسهل)

#### على Windows:
```bash
cd c:\Users\Hossam Galal\Desktop\RED
START-DRAGON-R1.bat
```

#### على Linux/Mac:
```bash
cd ~/Desktop/RED
chmod +x start-dragon-r1.sh
./start-dragon-r1.sh
```

### Option 2: التشغيل اليدوي (3 Terminals)

#### Terminal 1: Backend Gateway
```bash
cd Cerberus\gateway
python main.py
```

#### Terminal 2: Frontend UI
```bash
cd Cerberus\dragon-ui
npm run dev
```

#### Terminal 3: Docker & Stack (اختياري)
```bash
cd c:\Users\Hossam Galal\Desktop\RED
Start-Dragon-When-Open.bat
```

---

## ✅ تحقق من الاتصال

### 1. فحص صحة Gateway

```bash
curl http://localhost:8000/api/v1/health
```

**Response متوقع:**
```json
{
  "status": "healthy",
  "timestamp": "2025-03-10T...",
  "version": "1.0.0"
}
```

### 2. فحص WebSocket Connection

استخدم أي WebSocket tester أو متصفح:
```javascript
// في console المتصفح:
const ws = new WebSocket('ws://localhost:8000/ws/test-client');
ws.onopen = () => console.log('✅ WebSocket connected!');
ws.onerror = () => console.log('❌ WebSocket error');
ws.onmessage = (e) => console.log('📨', e.data);
```

### 3. فحص Frontend

اذهب إلى: `http://localhost:3000/chat`

يجب أن ترى:
- ✅ الواجهة تحميل بنجاح
- ✅ الـ Dashboard واضح (بدون أخطاء في Developer Console)
- ✅ يمكنك كتابة رسالة في Chat Box

---

## 🔧 استكشاف الأخطاء

### ❌ خطأ: "Cannot connect to localhost:8000"

**السبب:** Gateway لم يبدأ  
**الحل:**
```bash
# تحقق من المتطلبات
cd Cerberus\gateway
pip install -r requirements.txt

# شغّل مع verbose logging
python main.py --debug
```

### ❌ خطأ: "WebSocket connection failed"

**السبب:** WebSocket endpoint غير متاح  
**الحل:**
```bash
# تحقق من الـ port 8000
netstat -ano | findstr :8000

# إذا كان مشغول، اقتل العملية
taskkill /PID <PID> /F
```

### ❌ خطأ: "Cannot find module (next.js)"

**السبب:** Dependencies ناقصة  
**الحل:**
```bash
cd Cerberus\dragon-ui
npm install
npm run dev
```

### ❌ رسائل لا تظهر في Chat

**السبب:** الـ API routes ناقصة (تم إصلاحها الآن)  
**تحقق من أن لديك:**
- `src/app/api/chat/history/route.ts` ✅
- `src/app/api/chat/send/route.ts` ✅
- `src/app/api/chat/upload/route.ts` ✅
- `src/app/api/chat/process-file/route.ts` ✅

---

## 📝 استخدام الواجهة

### الخطوة 1: افتح الواجهة
```
http://localhost:3000/chat
```

### الخطوة 2: أرسل رسالة
```
مثال: "scan example.com for security vulnerabilities"
```

### الخطوة 3: المراقبة في Sidebar
- **Status**: يجب أن ترى "Running" أو "Pending"
- **Progress**: يتحدّث تلقائياً
- **Messages**: رسائل الـ agent تظهر هنا

### الخطوة 4: انتظر النتيجة
- يتم حفظ التقرير تلقائياً
- يمكنك تحميل النتيجة

---

## 🔐 الحوار (Chat Flow)

```
User: "scan example.com"
   ↓
[POST /api/chat/send]
   ↓
Gateway Creates Task
   ↓
[WebSocket: agent_message] → "Initializing scan..."
   ↓
[WebSocket: agent_message] → "Scanning ports..."
   ↓
[WebSocket: agent_message] → "Found 3 open ports"
   ↓
[WebSocket: agent_message] → "Attempting exploitation..."
   ↓
[WebSocket: TASK_COMPLETED] → "Report generated"
   ↓
Frontend Shows Report
```

---

## 📊 Architecture

```
┌─────────────────────────────────────────┐
│     Browser (http://localhost:3000)     │
│                                         │
│  ┌──────────────────────────────────┐   │
│  │   Dragon-R1 Chat (React/Next.js) │   │
│  │                                  │   │
│  │  - ChatInterface Component      │   │
│  │  - AgentPanel                   │   │
│  │  - MemoryViewer                 │   │
│  └────────────┬─────────────────────┘   │
└───────────────┼──────────────────────────┘
                │
      ┌─────────┴──────────┐
      │ HTTP (REST API)    │
      │ ws:// (WebSocket)  │
      │                    │
┌─────▼────────────────────────────────────┐
│   Gateway (http://localhost:8000)        │
│                                          │
│  ┌───────────────────────────────────┐  │
│  │  API Routes:                      │  │
│  │  - POST /api/v1/tasks            │  │
│  │  - GET  /api/v1/tasks/{id}       │  │
│  │  - POST /api/chat/message        │  │
│  │  - WS   /ws/{client_id}          │  │
│  └───────────────────────────────────┘  │
│                                          │
│  ┌───────────────────────────────────┐  │
│  │  Orchestrator:                    │  │
│  │  - Task Management                │  │
│  │  - Agent Communication            │  │
│  │  - Report Generation              │  │
│  └───────────────────────────────────┘  │
└─────────────────────────────────────────┘
```

---

## 🧪 اختبر الاتصال القائم (cURL)

```bash
# 1. حصل على الـ Health
curl http://localhost:8000/api/v1/health

# 2. أنشئ Task جديد
curl -X POST http://localhost:8000/api/v1/tasks \
  -H "Content-Type: application/json" \
  -d '{
    "target": "example.com",
    "target_type": "domain",
    "objective": "security assessment"
  }'

# 3. احصل على حالة Task
curl http://localhost:8000/api/v1/tasks/TASK_ID
```

---

## 📈 الملفات المعدّلة (الحل)

| File | النوع | التفاصيل |
|------|-------|---------|
| `dragon-ui/src/app/api/chat/history/route.ts` | ✅ جديد | GET chat history |
| `dragon-ui/src/app/api/chat/send/route.ts` | ✅ جديد | POST send message |
| `dragon-ui/src/app/api/chat/upload/route.ts` | ✅ جديد | POST file upload |
| `dragon-ui/src/app/api/chat/process-file/route.ts` | ✅ جديد | POST process file |
| `gateway/main.py` | ✅ موجود | WebSocket على `/ws/{client_id}` |
| `gateway/websocket_enhanced.py` | ✅ موجود | WebSocket Manager |

---

## 🎯 Summary الحل

### المشكلة الأصلية:
```
❌ الواجهة (localhost:3000/chat) لا تستجيب
❌ لا رسائل من الـ agent
❌ بدون اتصال بـ backend
```

### الحل المطبق:
✅ أضفنا 4 API routes ناقصة في dragon-ui  
✅ تأكدنا من WebSocket endpoint في gateway  
✅ أضفنا script تشغيل سهل (START-DRAGON-R1.bat)  
✅ وثائق كاملة لـ troubleshooting  

### النتيجة:
🎉 الواجهة تعمل الآن بشكل صحيح!

---

## 🚀 الخطوات التالية

1. **شغّل الخطوات:**
   ```bash
   START-DRAGON-R1.bat  # Windows
   # أو
   ./start-dragon-r1.sh  # Linux/Mac
   ```

2. **افتح الواجهة:**
   ```
   http://localhost:3000/chat
   ```

3. **اكتب رسالة:**
   ```
   "scan example.com"
   ```

4. **راقب الرسائل في Sidebar**

---

**Status:** ✅ READY TO USE  
**Last Updated:** 2025-03-10  
**Version:** 1.0.0
