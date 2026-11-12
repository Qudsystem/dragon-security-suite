# 🧪 Quick Testing Reference

**Status: All endpoints verified and working ✅**

---

## ⚡ Fast Test (2 minutes)

### 1. Start Backend
```bash
cd Cerberus/gateway
uvicorn main:app --host 0.0.0.0 --port 8000
```

### 2. Run Single Test
```bash
# Windows
test-endpoints.bat

# Linux/Mac
./test-endpoints.sh
```

### 3. Check output for ✓ marks

---

## 🔑 Manual Test with Token

### Step 1: Login
```bash
curl -X POST "http://localhost:8000/api/v1/auth/token" \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"dragon_admin"}'
```

**Save the access_token!**

### Step 2: Use Token
```bash
export TOKEN="<your_access_token>"

# Test any endpoint
curl -X GET "http://localhost:8000/api/v1/auth/me" \
  -H "Authorization: Bearer $TOKEN"
```

---

## 📋 All Endpoints

| # | Endpoint | Method | Auth | Purpose |
|---|----------|--------|------|---------|
| 1 | /api/v1/auth/token | POST | ❌ | Get tokens |
| 2 | /api/v1/auth/me | GET | ✅ | Current user |
| 3 | /api/v1/auth/refresh | POST | ❌ | New token |
| 4 | /api/v1/auth/logout | POST | ✅ | Logout |
| 5 | /api/v1/chat/history | GET | ✅ | Chat msgs |
| 6 | /api/v1/notifications | GET | ✅ | Alerts |
| 7 | /api/agents/registry | GET | ✅ | Agents |

✅ = Requires Bearer token  
❌ = No token needed

---

## 🐛 Quick Troubleshooting

### "Connection refused"
→ Start backend on port 8000

### "Invalid credentials"
→ Use username: `admin`, password: `dragon_admin`

### "Missing credentials"
→ Add Authorization header with Bearer token

### "Token expired"
→ Use refresh endpoint to get new token

### "Token has been revoked"
→ Login again to get fresh tokens

---

## 📁 Test Files

- **test-endpoints.bat** - Interactive Windows testing
- **test-endpoints.sh** - Automated Linux/Mac testing
- **ENDPOINT_VERIFICATION.md** - Full documentation (2500+ lines)
- **API-VERIFICATION-REPORT.md** - Complete report

---

## ✅ Checklist

- [ ] Backend running on localhost:8000
- [ ] Able to login and get tokens
- [ ] Can access /api/v1/auth/me with token
- [ ] Can refresh token with refresh_token
- [ ] Can logout and token is revoked
- [ ] Can access /api/v1/chat/history
- [ ] Can access /api/v1/notifications
- [ ] Can access /api/agents/registry

**If all checked: ✅ System is ready!**

---

## 📞 Next Steps

1. ✅ All endpoints implemented
2. ✅ All endpoints tested
3. ✅ All endpoints documented
4. → Deploy to test server
5. → Integrate with frontend
6. → Monitor in production

---

**Everything is ready to use! 🚀**
