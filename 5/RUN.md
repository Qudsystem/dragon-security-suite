# تشغيل الويب آب وتجربة التابات

## 1. تشغيل الباكند (Gateway)

من مجلد المشروع:

```bash
cd Cerberus
docker compose up -d
```

أو تشغيل الـ gateway محلياً على البورت 8000 إذا كان متوفراً.

## 2. تشغيل الويب آب

```bash
cd Cerberus/webapp
npm run dev
```

ثم افتح: **http://localhost:3000**

## 3. تسجيل الدخول (مهم لفتح التابات)

- اذهب إلى: **http://localhost:3000/login**
- **Username:** `admin`
- **Password:** `dragon-r1-admin`

بعد تسجيل الدخول يمكنك فتح: Projects، Tools، Reports، Settings، Agent Chat بدون خطأ "Failed to fetch".

## 4. إذا استخدمت Docker للويب آب

```bash
docker compose up -d --build webapp
```

تأكد أن `.env` أو `docker-compose.yml` يحدد `NEXT_PUBLIC_API_URL=http://localhost:8000` حتى يصل المتصفح إلى الـ API من جهازك.
