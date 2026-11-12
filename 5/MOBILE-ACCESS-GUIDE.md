<!-- Original Path: C:\Users\Hossam Galal\Desktop\RED\MOBILE-ACCESS-GUIDE.md -->

# 📱 دليل الوصول من الهاتف المحمول

## 🌐 الوصول إلى Dragon-R1 من هاتفك

### الخطوة 1: معرفة عنوان IP للكمبيوتر

**على Windows:**
```cmd
ipconfig
```
ابحث عن `IPv4 Address` - مثال: `192.168.1.100`

**على Linux/Mac:**
```bash
ifconfig
# أو
ip addr show
```

### الخطوة 2: تشغيل الخوادم

**تشغيل Next.js (الواجهة):**
```cmd
cd Cerberus\dragon-ui
npm run dev
```
سيعمل على: `http://0.0.0.0:3000`

**تشغيل FastAPI Gateway (الـ Backend):**
```cmd
cd Cerberus\gateway
python main.py
```
سيعمل على: `http://0.0.0.0:8081`

### الخطوة 3: الوصول من الهاتف

تأكد أن هاتفك على **نفس شبكة الواي-فاي** ثم افتح المتصفح:

**الواجهة الرئيسية:**
```
http://192.168.1.100:3000
```

**API Gateway:**
```
http://192.168.1.100:8081
```

**API Documentation:**
```
http://192.168.1.100:8081/docs
```

> 💡 استبدل `192.168.1.100` بعنوان IP الفعلي لجهازك

---

## 💬 التواصل مع النظام عبر WhatsApp/Telegram

### نعم! يمكن إضافة ميزة مشابهة لـ OpenClaw

Dragon-R1 يمكن دمجه مع:
- ✅ WhatsApp Business API
- ✅ Telegram Bot API
- ✅ Discord Bot
- ✅ Slack Integration

### 🤖 إنشاء Telegram Bot للنظام

سأنشئ bot يتيح لك:
- إنشاء مهام penetration testing
- متابعة التقدم في الوقت الفعلي
- استلام التنبيهات
- تحميل التقارير
- الموافقة على مراحل الاختراق

### المميزات المتاحة:

#### 1. **إنشاء مهمة جديدة**
```
/scan example.com
```

#### 2. **متابعة المهام**
```
/tasks
/status task_123
```

#### 3. **الموافقة على المراحل**
```
/approve task_123 exploitation
```

#### 4. **تحميل التقارير**
```
/report task_123
```

#### 5. **إيقاف المهام**
```
/stop task_123
```

---

## 🔧 التكوين السريع

### 1. إنشاء Telegram Bot

1. افتح [@BotFather](https://t.me/botfather) في Telegram
2. أرسل `/newbot`
3. اختر اسم للبوت (مثال: `Dragon_R1_Bot`)
4. احفظ الـ **Token** الذي سيعطيك إياه

### 2. إنشاء WhatsApp Bot

1. سجل في [WhatsApp Business API](https://business.whatsapp.com/products/business-platform)
2. أو استخدم خدمة مثل [Twilio](https://www.twilio.com/whatsapp)
3. احصل على API credentials

---

## 📊 مقارنة مع OpenClaw

| الميزة | OpenClaw | Dragon-R1 |
|--------|----------|-----------|
| التحكم عبر Telegram | ✅ | ✅ (سأضيفها) |
| التحكم عبر WhatsApp | ❌ | ✅ (سأضيفها) |
| Autonomous Agent | ✅ | ✅ (LangGraph) |
| Real-time Updates | ✅ | ✅ (WebSocket) |
| Multi-phase Approval | ❌ | ✅ |
| Neo4j Attack Graph | ❌ | ✅ |
| 6-Phase Recon | ❌ | ✅ |
| Self-Healing | ❌ | ✅ |

---

## 🚀 الخطوات التالية

سأقوم الآن بإنشاء:

1. ✅ **Telegram Bot Integration** - للتحكم الكامل عبر Telegram
2. ✅ **WhatsApp Bot Integration** - للتحكم عبر WhatsApp
3. ✅ **Mobile-Optimized UI** - واجهة محسنة للهاتف
4. ✅ **Push Notifications** - إشعارات فورية
5. ✅ **Voice Commands** - أوامر صوتية (اختياري)

---

## 🔐 الأمان

- 🔒 كل الاتصالات مشفرة
- 🔑 مصادقة JWT للـ API
- 👤 التحقق من هوية المستخدم في Telegram/WhatsApp
- 📝 تسجيل كامل لجميع الأوامر
- ⚠️ تأكيد إضافي للعمليات الخطرة

---

## 📞 الدعم

إذا واجهت أي مشكلة:
1. تحقق من أن الهاتف والكمبيوتر على نفس الشبكة
2. تأكد من أن Firewall لا يحجب المنافذ 3000 و 8081
3. جرب إعادة تشغيل الخوادم

**هل تريد أن أبدأ بإنشاء Telegram Bot الآن؟** 🤖
