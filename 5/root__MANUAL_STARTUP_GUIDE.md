# Dragon-R1 Manual Startup Guide

## Issues Fixed ✅

1. **PSReadLine Warning**: Fixed by importing the module
2. **Ollama Port Conflict**: Ollama is already running (good!)
3. **npm Directory Error**: You were in wrong directory
4. **Project Structure**: Verified all components exist

## Quick Start (Recommended)

Run the automated startup script:
```powershell
.\START-DRAGON-R1.bat
```
Then select option **1** for Native Mode.

## Manual Startup (Step by Step)

### 1. Start Backend Gateway
```powershell
# Navigate to backend
cd Cerberus\gateway

# Create virtual environment (if not exists)
python -m venv .venv

# Activate virtual environment
.venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Start the gateway
python main.py
```

### 2. Start Frontend UI (New Terminal)
```powershell
# Navigate to frontend
cd Cerberus\dragon-ui

# Install dependencies (if not done)
npm install

# Start development server
npm run dev
```

## Access Points

- **Frontend UI**: http://localhost:3000
- **Backend API**: http://localhost:8000
- **API Documentation**: http://localhost:8000/docs
- **Health Check**: http://localhost:8000/api/v1/health

## Default Login
- **Username**: admin
- **Password**: dragon_admin_123

## Troubleshooting

### If Ollama Issues Persist
```powershell
# Check if Ollama is running
Get-Process -Name "ollama" -ErrorAction SilentlyContinue

# If not running, start it
ollama serve
```

### If npm Still Fails
Make sure you're in the correct directory:
```powershell
# Should show: C:\Users\Hossam Galal\Desktop\RED\Cerberus\dragon-ui
pwd
ls package.json  # Should exist
```

### If Backend Fails
Check Python and dependencies:
```powershell
python --version  # Should be 3.8+
pip list | findstr fastapi  # Should show FastAPI
```

## Next Steps

Once both services are running:
1. Open http://localhost:3000 in your browser
2. Login with admin/dragon_admin_123
3. Start using Dragon-R1 for penetration testing

## Support

If you encounter issues:
1. Check the logs in terminal windows
2. Verify all dependencies are installed
3. Ensure ports 3000 and 8000 are available
4. Confirm Ollama is running on port 11434