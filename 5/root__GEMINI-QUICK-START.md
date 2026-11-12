# Gemini CLI - Quick Start Guide

## File Locations (IMPORTANT!)

All coordination files are in the **workspace root** (not in Cerberus/dragon-ui):

```
C:\Users\Hossam Galal\Desktop\RED\
├── GEMINI-INSTRUCTIONS.md          ← Full instructions (read this)
├── GEMINI.md                        ← Your presence file (create this)
├── KIRO.md                          ← Kiro's presence file (read this)
├── .agent_messages.json             ← Communication channel
├── .terminal_log                    ← Terminal logging
├── .gemini/
│   ├── watch-kiro.sh               ← Auto-watcher script
│   └── auto-trigger.sh             ← Manual trigger script
└── .kiro/
    ├── steering/
    │   └── gemini-coordination.md  ← Kiro's coordination rules
    └── specs/
        └── */tasks.md              ← Task lists
```

## Quick Commands (Run from workspace root)

### 1. Navigate to workspace root
```bash
cd "C:\Users\Hossam Galal\Desktop\RED"
```

### 2. Create your presence file
```bash
cat > GEMINI.md << 'EOF'
# Gemini CLI Status

**Status:** Active
**Timestamp:** $(date -Iseconds)
**Work Area:** Infrastructure / API Refactoring
**Current Task:** Setting up coordination with Kiro

## Active Files
- None yet

## Recent Activities
- Read coordination instructions
- Created presence file
EOF
```

### 3. Check if Kiro is active
```bash
cat KIRO.md 2>/dev/null || echo "Kiro not active yet"
```

### 4. Start auto-watcher (optional)
```bash
bash .gemini/watch-kiro.sh &
```

### 5. Manual coordination check
```bash
bash .gemini/auto-trigger.sh
```

## Windows PowerShell Alternative

If bash doesn't work, use PowerShell:

```powershell
# Navigate to workspace root
cd "C:\Users\Hossam Galal\Desktop\RED"

# Create presence file
@"
# Gemini CLI Status

**Status:** Active
**Timestamp:** $(Get-Date -Format o)
**Work Area:** Infrastructure / API Refactoring
**Current Task:** Setting up coordination with Kiro

## Active Files
- None yet

## Recent Activities
- Read coordination instructions
- Created presence file
"@ | Out-File -FilePath GEMINI.md -Encoding UTF8

# Check Kiro status
if (Test-Path KIRO.md) { Get-Content KIRO.md } else { "Kiro not active yet" }
```

## IMPORTANT: Command Execution Settings

**When prompted "Allow execution":**
- **Best option:** Choose **"3. Allow this command for all future sessions"**
- This permanently approves safe commands (Get-Date, Set-Content, etc.)
- You won't be prompted again for coordination tasks

**Alternative:** Load the auto-approve config:
```powershell
# If your CLI supports config files
gemini --config .gemini/config.json
```

The config file `.gemini/config.json` contains pre-approved safe commands for coordination.

## Test the Setup

1. Create GEMINI.md in workspace root
2. Ask Kiro: "Check if Gemini is active"
3. Kiro should detect your presence and respond

## Next Steps

Once setup is complete:
- Read full instructions: `GEMINI-INSTRUCTIONS.md`
- Update GEMINI.md every 2-3 minutes while working
- Check KIRO.md before modifying files
- Use tasks.md for task coordination

---

**CRITICAL:** Always work from workspace root `C:\Users\Hossam Galal\Desktop\RED`, not from subdirectories!
