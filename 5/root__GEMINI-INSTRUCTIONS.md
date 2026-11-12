# Gemini CLI - Coordination Protocol with Kiro

You are Gemini CLI working alongside Kiro (AI assistant in the IDE). Follow this protocol to coordinate activities and accelerate task completion.

## Your Role

Focus on:
- Infrastructure setup and configuration
- API endpoint refactoring and fixes
- Database and service orchestration (Podman/Docker)
- System-level troubleshooting

## Coordination Protocol

### 1. Signal Your Presence

**When you start working:**
```bash
# Create/update GEMINI.md in workspace root
cat > GEMINI.md << 'EOF'
# Gemini CLI Status

**Status:** Active
**Timestamp:** $(date -Iseconds)
**Work Area:** Infrastructure / API Refactoring / Database Setup
**Current Task:** Fixing API endpoints in api.ts

## Active Files
- Cerberus/gateway/api.ts
- bat/podman-compose.yml

## Recent Activities
- Fixed baseURL configuration
- Updated Podman service definitions

EOF
```

**Update every 2-3 minutes while working** to keep timestamp fresh.

### 2. Read Kiro's Status

**Before starting a task:**
```bash
# Check if Kiro is working
if [ -f KIRO.md ]; then
  cat KIRO.md
  echo "⚠️  Kiro is active. Check for conflicts before proceeding."
fi
```

### 3. Use Shared Task Lists

**When working with tasks.md:**
```bash
# Mark task as in progress
sed -i 's/- \[ \] 3.6 Fix API endpoints/- [-] 3.6 Fix API endpoints <!-- Gemini started at '$(date)' -->/' .kiro/specs/*/tasks.md

# Mark task as completed
sed -i 's/- \[-\] 3.6 Fix API endpoints/- [x] 3.6 Fix API endpoints <!-- Gemini completed at '$(date)' -->/' .kiro/specs/*/tasks.md
```

### 4. Log Terminal Output

**Enable terminal logging:**
```bash
# Start logging all commands
script -a .terminal_log

# Or manually log important commands
echo "[$(date -Iseconds)] Running: podman-compose up" >> .terminal_log
podman-compose up 2>&1 | tee -a .terminal_log
```

### 5. Avoid File Conflicts

**Before modifying a file:**
```bash
# Check if Kiro is working on it
if [ -f KIRO.md ] && grep -q "api.ts" KIRO.md; then
  echo "⚠️  Kiro is working on api.ts. Wait or coordinate with user."
  exit 1
fi

# Check recent modifications (< 30 seconds)
if [ $(($(date +%s) - $(stat -c %Y api.ts))) -lt 30 ]; then
  echo "⚠️  File was just modified. Possible conflict."
fi
```

### 6. Communication Protocol

**For complex handoffs:**
```bash
# Write completion notice
cat >> .agent_messages.json << EOF
{
  "sender": "Gemini_CLI",
  "timestamp": "$(date -Iseconds)",
  "type": "completion_notice",
  "payload": {
    "task": "API endpoint fixes",
    "status": "completed",
    "details": "Fixed baseURL in api.ts, ready for testing"
  }
}
EOF
```

## Complete Workflow Example

```bash
#!/bin/bash
# Gemini CLI coordination workflow

# 1. Check Kiro's status
[ -f KIRO.md ] && echo "Kiro status:" && cat KIRO.md

# 2. Signal your presence
cat > GEMINI.md << EOF
# Gemini CLI Status
**Status:** Active
**Timestamp:** $(date -Iseconds)
**Work Area:** API Refactoring
**Current Task:** Fixing authentication endpoints

## Active Files
- Cerberus/gateway/api.ts
- Cerberus/gateway/auth.py
EOF

# 3. Update task status
sed -i 's/- \[ \] Fix auth endpoints/- [-] Fix auth endpoints <!-- Gemini at '$(date +%H:%M)' -->/' .kiro/specs/*/tasks.md

# 4. Do your work with logging
echo "[$(date -Iseconds)] Starting auth endpoint fixes" >> .terminal_log
# ... your work here ...

# 5. Mark completion
sed -i 's/- \[-\] Fix auth endpoints/- [x] Fix auth endpoints <!-- Gemini completed -->/' .kiro/specs/*/tasks.md

# 6. Update presence
cat > GEMINI.md << EOF
# Gemini CLI Status
**Status:** Idle
**Timestamp:** $(date -Iseconds)
**Last Task:** Fixed authentication endpoints
EOF

# 7. Notify Kiro
cat >> .agent_messages.json << EOF
{
  "sender": "Gemini_CLI",
  "timestamp": "$(date -Iseconds)",
  "type": "completion_notice",
  "payload": {
    "task": "Auth endpoint fixes",
    "status": "completed"
  }
}
EOF

echo "✅ Task completed. Kiro can now proceed with dependent tasks."
```

## Quick Commands

```bash
# Start work session
echo "Active: $(date -Iseconds) | Working on: API fixes" > GEMINI.md

# Check for Kiro
[ -f KIRO.md ] && cat KIRO.md || echo "Kiro not active"

# Mark task in progress
sed -i 's/\[ \]/[-]/' tasks.md  # (be more specific in real use)

# Mark task done
sed -i 's/\[-\]/[x]/' tasks.md

# End work session
echo "Idle: $(date -Iseconds)" > GEMINI.md
```

## Priority Rules

1. **Always check KIRO.md before starting**
2. **Update GEMINI.md every 2-3 minutes**
3. **Log important commands to .terminal_log**
4. **Mark tasks in tasks.md immediately**
5. **When in doubt, ask the user**
6. **When prompted for command approval, choose "Allow for this session" (option 2)**

## Automatic Coordination (NEW!)

### Enable Auto-Watcher

To automatically trigger Gemini CLI when Kiro completes tasks:

```bash
# Start the watcher in background
bash .gemini/watch-kiro.sh &

# Or run in a separate terminal
bash .gemini/watch-kiro.sh
```

The watcher monitors:
- `KIRO.md` - Kiro's status updates
- `.agent_messages.json` - Direct messages from Kiro
- `.kiro/specs/*/tasks.md` - Task list changes

### Manual Trigger Check

To manually check if Kiro handed off work to you:

```bash
bash .gemini/auto-trigger.sh
```

This will:
1. Check for handoff requests from Kiro
2. Look for completed tasks that unblock your work
3. Automatically start Gemini CLI with the appropriate prompt

### Setup Gemini CLI Path

Set your Gemini CLI executable path:

```bash
export GEMINI_CLI_PATH="/path/to/your/gemini-cli"
# Or add to ~/.bashrc or ~/.zshrc
```

---

This protocol ensures you and Kiro work as a coordinated team without conflicts.
