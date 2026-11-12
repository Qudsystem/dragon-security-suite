# 🚀 Quick Setup & Testing Guide

## ⚠️ IMPORTANT: Backend Setup Required

The new API endpoints require the backend to be **restarted** after the code changes.

### Step 1: Start the Backend

**Windows PowerShell:**
```powershell
cd Cerberus\gateway
python -m uvicorn main:app --host 0.0.0.0 --port 8000
```

**Linux/Mac:**
```bash
cd Cerberus/gateway
uvicorn main:app --host 0.0.0.0 --port 8000
```

Expected output should show:
```
INFO:     Uvicorn running on http://0.0.0.0:8000
```

### Step 2: Wait for Backend to Start

Wait 2-3 seconds for the backend to fully initialize and load all routes.

### Step 3: Run Tests in a New Terminal

**Windows PowerShell (New Terminal):**
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
.\test-endpoints.bat
```

**Linux/Mac (New Terminal):**
```bash
cd ~/Desktop/RED
chmod +x test-endpoints.sh
./test-endpoints.sh
```

---

## 🔑 Test Credentials

**Username:** `admin`  
**Password:** `dragon_admin_123`

These credentials are now set as defaults in `Cerberus/gateway/config.py`

---

## ✅ What to Expect

When the tests run successfully, you should see:

```
[✓] Gateway is running (HTTP 200)
[✓] LOGIN TEST - access tokens received
[✓] GET /api/v1/auth/me - User info retrieved
[✓] GET /api/v1/chat/history - Chat history retrieved
[✓] GET /api/v1/notifications - Notifications retrieved
[✓] GET /api/agents/registry - Agent registry retrieved
[✓] POST /api/v1/auth/refresh - New token issued
[✓] POST /api/v1/auth/logout - Logged out successfully
[✓] Token correctly revoked
[✓] Authorization required
```

---

## 🐛 Troubleshooting

### Issue: "Connection refused"
**Solution:** Make sure backend is running on port 8000. Check Step 1.

### Issue: "Invalid credentials"
**Solution:** The password might not match. Verify you're using `dragon_admin_123`.

### Issue: "Not Found" (404 errors)
**Solution:** The backend needs to be restarted to load new routes. Kill the backend (Ctrl+C) and restart it.

### Issue: "Not authenticated"
**Solution:** This is correct behavior - it means Authorization header is required. The test should pass.

---

## 📊 Test Files

- **test-endpoints.bat** - Interactive Windows testing script
- **test-endpoints.sh** - Automated Linux/Mac testing script
- **ENDPOINT_VERIFICATION.md** - Complete documentation
- **API-VERIFICATION-REPORT.md** - Technical report
- **TESTING-QUICK-REFERENCE.md** - Quick reference guide

---

## 🎯 Next Steps

1. ✅ Start backend (see Step 1)
2. ✅ Run tests (see Step 3)
3. ✅ Check all tests pass
4. ✅ Review test responses
5. → Integrate with frontend
6. → Deploy to production

---

## 📝 Configuration

Default credentials are set in: `Cerberus/gateway/config.py`

```python
ADMIN_USERNAME: str = "admin"
ADMIN_PASSWORD: str = "dragon_admin_123"
ADMIN_EMAIL: str = "admin@dragon-r1.io"
```

To change credentials, modify these values before running backend.

---

## 💡 Tips

- Keep terminal open while running backend
- Use separate terminal for running tests
- Check logs in backend terminal for detailed error info
- Responses are pretty-printed in JSON format
- All endpoint responses are saved to files

---

**Status: Ready to Test! ✅**
