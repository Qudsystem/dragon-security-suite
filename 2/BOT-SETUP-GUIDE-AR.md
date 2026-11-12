<!-- Original Path: C:\Users\Hossam Galal\Desktop\RED\BOT-SETUP-GUIDE-AR.md -->

# 🤖 دليل إعداد البوتات - Dragon-R1

## 📱 نظرة عامة

Dragon-R1 يدعم الآن التحكم الكامل عبر:
- ✅ **Telegram Bot** - تحكم كامل عبر تيليجرام
- ✅ **WhatsApp Bot** - تحكم عبر واتساب
- ✅ **Web Interface** - واجهة ويب احترافية
- ✅ **REST API** - API كامل للتكامل

---

## 🚀 الإعداد السريع

### الخطوة 1: تثبيت المتطلبات

```cmd
cd Cerberus\gateway
pip install -r requirements-bots.txt
```

### الخطوة 2: إنشاء Telegram Bot

1. افتح Telegram وابحث عن [@BotFather](https://t.me/botfather)
2. أرسل `/newbot`
3. اختر اسم للبوت: `Dragon R1 Bot`
4. اختر username: `dragon_r1_bot` (يجب أن ينتهي بـ _bot)
5. احفظ الـ **Token** الذي سيعطيك إياه

**مثال على Token:**
```
1234567890:ABCdefGHIjklMNOpqrsTUVwxyz
```

### الخطوة 3: إعداد WhatsApp Bot (اختياري)

#### الطريقة 1: Twilio (موصى بها)

1. سجل في [Twilio](https://www.twilio.com/try-twilio)
2. اذهب إلى [WhatsApp Sandbox](https://console.twilio.com/us1/develop/sms/try-it-out/whatsapp-learn)
3. اتبع التعليمات لربط رقم WhatsApp
4. احصل على:
   - Account SID
   - Auth Token
   - WhatsApp Number

#### الطريقة 2: WhatsApp Business API (للإنتاج)

1. سجل في [WhatsApp Business Platform](https://business.whatsapp.com/)
2. أكمل عملية التحقق
3. احصل على API credentials

### الخطوة 4: تكوين البيئة

انسخ ملف `.env.example` إلى `.env`:

```cmd
copy .env.example .env
```

افتح `.env` وأضف بياناتك:

```env
# Telegram
TELEGRAM_BOT_TOKEN=1234567890:ABCdefGHIjklMNOpqrsTUVwxyz

# WhatsApp (Twilio)
TWILIO_ACCOUNT_SID=ACxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
TWILIO_AUTH_TOKEN=your_auth_token_here
TWILIO_WHATSAPP_NUMBER=whatsapp:+14155238886
```

### الخطوة 5: تشغيل البوتات

**الطريقة السهلة:**
```cmd
start-bots.bat
```

**أو يدوياً:**

**Telegram Bot:**
```cmd
python integrations/telegram_bot.py
```

**WhatsApp Bot:**
```cmd
python integrations/whatsapp_bot.py
```

---

## 📱 استخدام Telegram Bot

### الأوامر الأساسية

#### 1. البدء
```
/start
```
سيرحب بك البوت ويعرض الأوامر المتاحة.

#### 2. بدء فحص جديد
```
/scan example.com
/scan 192.168.1.100
/scan https://example.com/app
```

#### 3. عرض المهام
```
/tasks
```

#### 4. حالة مهمة محددة
```
/status task_abc123
```

#### 5. إيقاف مهمة
```
/stop task_abc123
```

#### 6. تحميل التقرير
```
/report task_abc123
```

#### 7. الموافقة على مرحلة
```
/approve task_abc123 exploitation
/approve task_abc123 post_exploitation
```

#### 8. حالة النظام
```
/health
```

#### 9. المساعدة
```
/help
```

### مثال على سيناريو كامل

```
1. /start
   👉 البوت يرحب بك

2. /scan example.com
   👉 يبدأ الفحص في مرحلة Informational
   👉 تستلم تحديثات تلقائية

3. /status task_abc123
   👉 تشاهد التقدم والنتائج

4. /approve task_abc123 exploitation
   👉 توافق على مرحلة الاستغلال
   👉 يبدأ الاستغلال النشط

5. /report task_abc123
   👉 تحصل على ملخص التقرير
```

---

## 💬 استخدام WhatsApp Bot

### نفس الأوامر تماماً!

```
/start
/scan example.com
/tasks
/status task_123
/stop task_123
/report task_123
/approve task_123 exploitation
/health
/help
```

### إعداد Webhook (للإنتاج)

1. احصل على URL عام لخادمك (استخدم ngrok للتجربة):
```cmd
ngrok http 8082
```

2. في Twilio Console، اذهب إلى:
   - Messaging → Settings → WhatsApp Sandbox Settings
   - أو: Phone Numbers → Your WhatsApp Number → Messaging

3. ضع Webhook URL:
```
https://your-domain.com/whatsapp/webhook
```

---

## 🌐 الوصول من الهاتف

### 1. معرفة IP الكمبيوتر

```cmd
ipconfig
```
ابحث عن IPv4 Address (مثال: `192.168.1.100`)

### 2. تشغيل الخوادم

**الواجهة (Next.js):**
```cmd
cd Cerberus\dragon-ui
npm run dev
```
يعمل على: `http://0.0.0.0:3000`

**Backend (FastAPI):**
```cmd
cd Cerberus\gateway
python main.py
```
يعمل على: `http://0.0.0.0:8081`

### 3. الوصول من الهاتف

تأكد أن هاتفك على **نفس الواي-فاي**، ثم افتح:

```
http://192.168.1.100:3000
```

---

## 🔐 الأمان

### تفعيل المصادقة

في `telegram_bot.py` و `whatsapp_bot.py`، عدّل:

```python
def _is_authorized(self, user_id: int) -> bool:
    # أضف قائمة المستخدمين المصرح لهم
    AUTHORIZED_USERS = [123456789, 987654321]  # Telegram IDs
    return user_id in AUTHORIZED_USERS
```

### تشفير الاتصالات

- ✅ كل الاتصالات مع Telegram مشفرة تلقائياً
- ✅ WhatsApp يستخدم End-to-End Encryption
- ✅ API Gateway يدعم HTTPS (استخدم SSL certificate)

---

## 🎯 المميزات المتقدمة

### 1. التحديثات التلقائية

البوت يرسل تحديثات تلقائية عند:
- ✅ بدء مهمة جديدة
- ✅ تغيير المرحلة
- ✅ اكتشاف ثغرة
- ✅ اكتمال المهمة
- ✅ حدوث خطأ

### 2. الأزرار التفاعلية (Telegram)

عند إنشاء مهمة، تحصل على أزرار:
- 📊 الحالة
- 🛑 إيقاف
- 📄 تقرير

### 3. الموافقات الآمنة

قبل الانتقال لمرحلة خطرة:
- ⚠️ يطلب البوت موافقتك
- 🔒 لا يمكن تجاوز الموافقة
- 📝 كل شيء مسجل للمراجعة

---

## 🐛 حل المشاكل

### البوت لا يرد

1. تحقق من Token:
```cmd
echo %TELEGRAM_BOT_TOKEN%
```

2. تحقق من الاتصال:
```cmd
curl https://api.telegram.org/bot<YOUR_TOKEN>/getMe
```

3. تحقق من Logs:
```cmd
python integrations/telegram_bot.py
```

### WhatsApp لا يعمل

1. تحقق من Twilio credentials
2. تحقق من Webhook URL
3. راجع Twilio Logs في Console

### لا يمكن الوصول من الهاتف

1. تأكد من نفس الشبكة
2. أوقف Firewall مؤقتاً للتجربة
3. تحقق من IP:
```cmd
ipconfig
ping 192.168.1.100
```

---

## 📊 مقارنة مع الأنظمة الأخرى

| الميزة | OpenClaw | Dragon-R1 | Metasploit |
|--------|----------|-----------|------------|
| Telegram Bot | ✅ | ✅ | ❌ |
| WhatsApp Bot | ❌ | ✅ | ❌ |
| Autonomous Agent | ✅ | ✅ | ❌ |
| Multi-Phase Approval | ❌ | ✅ | ❌ |
| Real-time Updates | ✅ | ✅ | ❌ |
| Attack Graph | ❌ | ✅ (Neo4j) | ❌ |
| Self-Healing | ❌ | ✅ | ❌ |
| Arabic Support | ❌ | ✅ | ❌ |
| Mobile Web UI | ❌ | ✅ | ❌ |

---

## 🎓 أمثلة عملية

### مثال 1: فحص موقع

```
User: /scan example.com

Bot: ✅ تم إنشاء المهمة!
     🎯 الهدف: example.com
     🆔 المعرف: task_abc123
     🔄 المرحلة: Informational

[بعد 5 دقائق]

Bot: 🔄 تحديث المهمة
     📊 التقدم: 45%
     🔍 تم اكتشاف 12 subdomain

[بعد 10 دقائق]

Bot: ⚠️ تم اكتشاف ثغرة!
     🔴 SQL Injection في /login.php
     📊 Severity: High

[بعد 15 دقيقة]

Bot: 🟢 المهمة انتهت!
     📊 النتائج:
     • 45 findings
     • 3 vulnerabilities (1 critical, 2 high)

User: /report task_abc123

Bot: [يرسل ملخص التقرير]
```

### مثال 2: الموافقة على الاستغلال

```
User: /scan vulnerable-site.com

Bot: ✅ المهمة بدأت
     🔄 مرحلة Informational

[بعد فترة]

Bot: ⚠️ تم اكتشاف ثغرات قابلة للاستغلال
     هل تريد الموافقة على مرحلة Exploitation؟

User: /approve task_xyz789 exploitation

Bot: ✅ تمت الموافقة
     ⚠️ تحذير: عمليات هجومية نشطة!
     🔄 بدء مرحلة Exploitation...

[البوت يبدأ الاستغلال النشط]
```

---

## 🔮 المميزات القادمة

- [ ] أوامر صوتية (Voice Commands)
- [ ] إشعارات Push للهاتف
- [ ] Discord Bot Integration
- [ ] Slack Integration
- [ ] تقارير PDF عبر البوت
- [ ] لوحة تحكم تفاعلية في Telegram
- [ ] Multi-language support
- [ ] AI Assistant للمساعدة

---

## 📞 الدعم

إذا واجهت أي مشكلة:

1. راجع هذا الدليل
2. تحقق من Logs
3. ابحث في Issues على GitHub
4. اسأل في المجتمع

---

## ⚖️ تحذير قانوني

⚠️ **مهم جداً:**

- استخدم هذه الأدوات فقط على أنظمة لديك تصريح باختبارها
- الاستخدام غير المصرح به غير قانوني
- أنت مسؤول عن استخدامك للأداة
- Dragon-R1 للأغراض التعليمية والاختبار الأخلاقي فقط

---

## 🎉 ابدأ الآن!

```cmd
cd Cerberus\gateway
start-bots.bat
```

ثم افتح Telegram وابحث عن بوتك وأرسل `/start`! 🚀

**استمتع بالتحكم الكامل في منصة الأمن الهجومي من هاتفك!** 📱🐉
