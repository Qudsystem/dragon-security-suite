# تعليمات إعادة تشغيل البوابة (Gateway Restart Instructions)

## المشكلة (Problem)
نقطة النهاية `/api/v1/validation/rules` موجودة في الكود لكن البوابة الحالية لم تحملها بعد.

The `/api/v1/validation/rules` endpoint exists in the code but the running gateway hasn't loaded it yet.

## الحل (Solution)
يجب إعادة تشغيل البوابة لتحميل التغييرات الجديدة.

The gateway needs to be restarted to load the new changes.

## خطوات إعادة التشغيل (Restart Steps)

### Windows:
```powershell
# 1. إيقاف البوابة الحالية (Stop current gateway)
# اضغط Ctrl+C في نافذة البوابة أو:
taskkill /F /IM python.exe /FI "WINDOWTITLE eq *uvicorn*"

# 2. الانتقال إلى مجلد البوابة (Navigate to gateway folder)
cd Cerberus\gateway

# 3. تفعيل البيئة الافتراضية (Activate virtual environment)
.venv\Scripts\activate

# 4. تشغيل البوابة (Start gateway)
python main.py
# أو (or):
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

### Linux/Mac:
```bash
# 1. إيقاف البوابة الحالية (Stop current gateway)
# اضغط Ctrl+C في نافذة البوابة أو:
pkill -f "python.*main.py"

# 2. الانتقال إلى مجلد البوابة (Navigate to gateway folder)
cd Cerberus/gateway

# 3. تفعيل البيئة الافتراضية (Activate virtual environment)
source .venv/bin/activate

# 4. تشغيل البوابة (Start gateway)
python main.py
# أو (or):
uvicorn main:app --reload --host 0.0.0.0 --port 8000
```

## التحقق من نجاح إعادة التشغيل (Verify Restart Success)

بعد إعادة التشغيل، تحقق من أن النقطة تعمل:

After restart, verify the endpoint works:

```powershell
# Windows PowerShell:
curl http://localhost:8000/api/v1/validation/rules

# Linux/Mac:
curl http://localhost:8000/api/v1/validation/rules | jq
```

يجب أن ترى مصفوفة من قواعد التحقق (You should see an array of validation rules).

## إعادة تشغيل الاختبارات (Re-run Tests)

بعد إعادة تشغيل البوابة، شغل الاختبارات مرة أخرى:

After restarting the gateway, run the tests again:

```bash
python -m pytest Cerberus/gateway/tests/test_chat_response_bug_exploration.py -v
```

يجب أن تنجح جميع الاختبارات الآن (All tests should pass now).

## ملاحظات (Notes)

- التغييرات في الكود تتطلب إعادة تشغيل البوابة لتصبح فعالة
- Code changes require gateway restart to take effect

- إذا كنت تستخدم `--reload` مع uvicorn، سيتم إعادة التحميل تلقائياً
- If using `--reload` with uvicorn, it will auto-reload on file changes

- تأكد من عدم وجود أخطاء في السجلات عند بدء التشغيل
- Check for errors in logs during startup
