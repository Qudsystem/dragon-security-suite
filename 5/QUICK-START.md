# 🎯 Dragon-R1 Chat - Quick Reference

## 🚀 تشغيل سريع (30 ثانية)

### Windows
```bash
cd c:\Users\Hossam Galal\Desktop\RED
START-DRAGON-R1.bat
# افتح http://localhost:3000/chat
```

### Linux/Mac
```bash
cd ~/Desktop/RED
./start-dragon-r1.sh
# افتح http://localhost:3000/chat
```

---

## ✅ ما تم إصلاحه

| المشكلة | الحل | الملف |
|--------|------|------|
| ❌ No `/api/chat/history` | ✅ Created | `dragon-ui/src/app/api/chat/history/route.ts` |
| ❌ No `/api/chat/send` | ✅ Created | `dragon-ui/src/app/api/chat/send/route.ts` |
| ❌ No `/api/chat/upload` | ✅ Created | `dragon-ui/src/app/api/chat/upload/route.ts` |
| ❌ No `/api/chat/process-file` | ✅ Created | `dragon-ui/src/app/api/chat/process-file/route.ts` |
| ❌ No startup script | ✅ Created | `START-DRAGON-R1.bat` / `start-dragon-r1.sh` |
| ❌ No documentation | ✅ Created | `DRAGON-R1-CHAT-SETUP.md` / `API-REFERENCE.md` |

---

## 🔗 اختبار الاتصال

```bash
# 1. فحص Gateway
curl http://localhost:8000/api/v1/health
# ✅ {"status": "healthy"}

# 2. فحص Frontend
curl http://localhost:3000
# ✅ HTML response

# 3. فحص Chat API
curl http://localhost:3000/api/chat/history
# ✅ {"messages": [...]}
```

---

## 📍 الـ URLs

| Service | URL | Port |
|---------|-----|------|
| Chat UI | http://localhost:3000/chat | 3000 |
| Backend API | http://localhost:8000/api/v1 | 8000 |
| WebSocket | ws://localhost:8000/ws/{id} | 8000 |
| Health Check | http://localhost:8000/api/v1/health | 8000 |

---

## 🧪 اختبار Chat

1. **Open:** http://localhost:3000/chat
2. **Type:** `scan example.com` (أي رسالة)
3. **Send:** اضغط Enter
4. **Wait:** 2-3 ثواني
5. **See:** الرسائل تظهر في Sidebar

---

## 📋 API Endpoints

### POST /api/chat/send
```javascript
const res = await fetch('/api/chat/send', {
  method: 'POST',
  body: JSON.stringify({ message: 'scan example.com' })
});
// { task_id: "task_123", message: "OK", suggestions: [...] }
```

### GET /api/chat/history
```javascript
const res = await fetch('/api/chat/history?task_id=task_123');
// { messages: [{role: "agent", content: "...", timestamp: "..."}] }
```

### WS /ws/{client_id}
```javascript
const ws = new WebSocket('ws://localhost:8000/ws/client-123');
ws.onmessage = (e) => console.log(JSON.parse(e.data));
// { type: "agent_message", content: "Scanning..." }
```

---

## 🐛 الأخطاء الشائعة

### "Cannot POST /api/chat/send"
```bash
# تأكد من الملفات:
ls -la Cerberus/dragon-ui/src/app/api/chat/
# يجب ترى 4 ملفات: history, send, upload, process-file
```

### "WebSocket refused"
```bash
# شغّل Gateway:
cd Cerberus/gateway
python main.py
```

### "Port 8000 in use"
```bash
# Windows
taskkill /F /IM python.exe

# Linux/Mac
lsof -i :8000 | kill -9 $(awk 'NR==2 {print $2}')
```

---

## 📖 التفاصيل الكاملة

- **Setup Guide:** `DRAGON-R1-CHAT-SETUP.md`
- **API Docs:** `API-REFERENCE.md`
- **Final Report:** `DRAGON-R1-CHAT-FIX-FINAL.md`

---

## ✨ Architecture

```
User Browser (localhost:3000)
    ↓ HTTP/WebSocket
    ↓
Gateway (localhost:8000)
    ├─ /api/v1/tasks
    ├─ /api/chat/message
    └─ /ws/{client_id}
```

---

**🎉 Ready to Use!**

```bash
START-DRAGON-R1.bat
# Then: http://localhost:3000/chat
```
