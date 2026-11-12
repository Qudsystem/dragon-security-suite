# 🚀 Dragon-R1 Chat - Quick Start Guide

## ⚡ تشغيل سريع بـ 3 خطوات

### Step 1: بدء Backend Gateway
```bash
cd C:\Users\Hossam Galal\Desktop\RED\Cerberus\gateway

# خيار 1: استخدام Python مباشرة
python main.py

# خيار 2: استخدام الـ batch file
cmd /c START-SERVER.bat

# خيار 3: استخدام PowerShell
powershell -ExecutionPolicy Bypass -File start.ps1
```

✅ Gateway سيبدأ على `http://localhost:8000`

---

### Step 2: بدء Frontend NanoChat
```bash
cd C:\Users\Hossam Galal\Desktop\RED\Cerberus\nanochat

# تثبيت المتطلبات (مرة واحدة فقط)
pip install -r requirements.txt

# تشغيل التطبيق
streamlit run app.py
```
✅ NanoChat سيفتح على `http://localhost:8501`


---

### Step 3: استخدام التطبيق
آنتُ الآن جاهز! 🎉

```
1. افتح http://localhost:8501 في المتصفح
2. أدخل اسم الـ target (مثل: example.com أو 192.168.1.1)
3. اضغط Enter أو Send
4. Dragon-R1 سيبدأ الـ engagement تلقائياً
5. شاهد رسائل الـ agent تظهر في الدردشة
6. متابعة Progress في الـ Sidebar
```

---

## 🔍 التحقق من التشغيل

### 1. هل Gateway يعمل؟
```bash
curl http://localhost:8000/api/v1/health
# نتيجة متوقعة: {"status": "healthy", ...}
```

### 2. هل NanoChat متصل بـ Gateway؟
- تحقق من Sidebar → "Gateway: healthy" ✅
- تحقق من Sidebar → عدد الـ Modules/Tools/Workers

### 3. هل Agent يستجيب؟
- أرسل target: "example.com"
- تحقق من ظهور "✅ Engagement Initiated"
- تحقق من ظهور Task ID

---

## 📊 شاشات التطبيق

### Sidebar (يمين الشاشة)
```
⚙️ System Status
✅ Gateway: healthy

📊 Task Info
Task ID: xxxxx
Status: Running
Progress: 45%
Phase: Reconnaissance

🔄 Refresh Button
```

### Main Chat Area (وسط الشاشة)
```
User: example.com
━━━━━━━━━━━━━━━━━━━━━━━━━━
Dragon-R1: ✅ Engagement Initiated
          Task ID: xxxxx
          Status: Starting...
          
[Agent Message 1] 🔍 Scanning target...
[Agent Message 2] 📡 Found 5 open ports...
[Agent Message 3] 🔓 Attempting exploitation...
━━━━━━━━━━━━━━━━━━━━━━━━━━
[Input Field]: Ask Dragon-R1 or enter target...
```

---

## ⚠️ الأخطاء الشائعة و الحل

### ❌ Error: "Cannot connect to Gateway"
```
الحل:
1. تأكد من تشغيل gateway (Step 1)
2. تأكد من PORT 8000 مفتوح (ليس مُستخدم)
3. جرّب: netstat -an | findstr :8000

إذا لم تنجح:
- أغلق جميع البرامج التي تستخدم port 8000
- أعد تشغيل gateway
```

### ❌ Error: "Failed to authenticate"
```
الحل:
1. افحص بيانات الـ auth في app.py:
   - Username: admin
   - Password: dragon-r1-admin

2. تأكد من تشغيل gateway بشكل صحيح

3. اختبر الـ auth يدويّاً:
   curl -X POST http://localhost:8000/api/v1/auth/token \
     -H "Content-Type: application/json" \
     -d '{"username": "admin", "password": "dragon-r1-admin"}'
```

### ❌ Error: "Task creation failed"
```
الحل:
1. تأكد من إدخال target صحيح (مثل: example.com)
2. تأكد من Gateway يعمل بدون مشاكل
3. افحص logs في gateway:
   - ابحث عن "Task created"
   - افحص رسائل الـ Error
```

### ❌ No agent messages appearing
```
الحل:
1. اضغط 🔄 Refresh في الـ Sidebar
2. الانتظر قليلاً (الـ Polling يعمل كل 2 ثانية)
3. افحص:
   curl http://localhost:8000/api/v1/tasks/<task_id> \
     -H "Authorization: Bearer <your_token>"
   
   - ابحث عن "agent_messages" في الـ response
   - إذا كانت موجودة لكن لا تظهر: اضغط Refresh
```

---

## 🎯 الاستخدام المتقدم

### مراقبة الـ Backend Logs
```bash
# Open new terminal and run:
cd C:\Users\Hossam Galal\Desktop\RED\Cerberus\gateway
python -c "import logging; logging.basicConfig(level=logging.DEBUG); exec(open('main.py').read())"
```

### اختبار API يدويّاً
```bash
# 1. Get Token
$token = (curl -X POST http://localhost:8000/api/v1/auth/token `
  -H "Content-Type: application/json" `
  -d '{"username":"admin","password":"dragon-r1-admin"}' | ConvertFrom-Json).access_token

# 2. Create Task
curl -X POST http://localhost:8000/api/v1/tasks `
  -H "Authorization: Bearer $token" `
  -H "Content-Type: application/json" `
  -d '{"target":"example.com","target_type":"domain"}'

# 3. Get Task Status
curl http://localhost:8000/api/v1/tasks/<task_id> `
  -H "Authorization: Bearer $token"
```

### تعديل الـ Polling Interval
```python
# في nanochat/app.py، ابحث عن:
await asyncio.sleep(2)  # 2 seconds

# غيّر إلى:
await asyncio.sleep(1)  # 1 second (أسرع)
# أو
await asyncio.sleep(5)  # 5 seconds (أبطأ)
```

---

## 📱 واجهة الـ Dragon-R1 API

### Endpoints الرئيسية

#### 1. Authentication
```
POST /api/v1/auth/token
{
  "username": "admin",
  "password": "dragon-r1-admin"
}
Response: {"access_token": "...", "token_type": "bearer"}
```

#### 2. Create Task
```
POST /api/v1/tasks
Headers: Authorization: Bearer <token>
Body: {
  "target": "example.com",
  "target_type": "domain"  // أو "ip"
}
Response: {"task_id": "...", "status": "pending"}
```

#### 3. Get Task Status (مع Agent Messages)
```
GET /api/v1/tasks/<task_id>
Headers: Authorization: Bearer <token>
Response: {
  "task_id": "...",
  "status": "running",
  "progress_percent": 45,
  "current_phase": "reconnaissance",
  "agent_messages": [
    {
      "role": "agent",
      "content": "Scanning target...",
      "timestamp": "2025-03-10T..."
    },
    ...
  ]
}
```

#### 4. Get Report (عند الانتهاء)
```
GET /api/v1/tasks/<task_id>/report
Headers: Authorization: Bearer <token>
```

---

## 🔧 Configuration

### تغيير Gateway URL
```python
# في nanochat/app.py:
GATEWAY_URL = os.environ.get("GATEWAY_URL", "http://localhost:8000")

# أو استخدم environment variable:
set GATEWAY_URL=http://192.168.1.100:8000
```

### تغيير WebSocket URL
```python
# في nanochat/app.py (للمستقبل):
GATEWAY_WS_URL = os.environ.get("GATEWAY_WS_URL", "ws://localhost:8000")
```

---

## 📝 ملاحظات تهمك

### الأداء
- ✅ Polling كل 2 ثانية (قابلة للتعديل)
- ✅ Server response time: <100ms
- ✅ Concurrent tasks: محدودة بـ memory

### الأمان
- ✅ JWT authentication activated
- ✅ CORS enabled (قابلة للتعديل)
- ✅ Password: يجب تغييره في الإنتاج

### future Improvements
- [ ] Switch to WebSocket for real-time updates
- [ ] Add Redis for message persistence
- [ ] Add multi-user support
- [ ] Add analytics dashboard

---

## 🆘 الدعم والمساعدة

### الملفات الإضافية:
- 📄 `IMPLEMENTATION_SUMMARY.md` - تفاصيل الحل
- 📄 `DRAGON-R1_CHAT_FIX.md` - شرح المشكلة والحل
- 🧪 `test_chat_fixes.py` - اختبار الـ integration

### التواصل:
- تحقق من logs في terminal
- اختبر API endpoints يدويّاً
- استخدم `test_chat_fixes.py` للتشخيص

---

## ✅ Checklist قبل التشغيل

- [ ] Python 3.8+ مثبت
- [ ] pip مثبت
- [ ] PORT 8000 متاح
- [ ] PORT 8501 متاح
- [ ] requirements.txt packages مثبتة
- [ ] Gateway server في working directory صحيح
- [ ] Environment variables (إن وجدت) محددة

---

## 🎬 الخطوات النهائية

```bash
# 1. تحقق من البيئة
python --version  # Python 3.8+

# 2. شغّل Gateway في Terminal 1
cd Cerberus\gateway && python main.py

# 3. شغّل NanoChat في Terminal 2
cd Cerberus\nanochat && streamlit run app.py

# 4. افتح المتصفح
# http://localhost:8501

# 5. أرسل target
# example.com

# 6. استمتع! 🚀
```

---

**Status:** ✅ Ready to Use  
**Last Updated:** 2025-03-10  
**Tested:** Yes ✓
