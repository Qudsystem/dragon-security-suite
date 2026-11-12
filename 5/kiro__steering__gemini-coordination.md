---
inclusion: auto
---

# Gemini CLI Coordination Protocol

You are working alongside Gemini CLI in this workspace. This steering file helps you coordinate activities to accelerate task completion without conflicts.

## Gemini CLI Role

Gemini CLI focuses on:
- Infrastructure setup and configuration
- API endpoint refactoring and fixes
- Database and service orchestration (Podman/Docker)
- System-level troubleshooting

## Coordination Mechanisms

### 1. Presence Detection

**Before starting any task:**
- Check for `GEMINI.md` or `.gemini_context` in workspace root
- If found, read it to understand what Gemini is currently working on
- If timestamp is < 5 minutes old, Gemini is actively working

### 2. Shared Task Lists

**When working with tasks.md:**
- Check task status before starting: `[ ]` = available, `[-]` = in progress, `[x]` = done
- If a task is `[-]` for > 10 minutes, ask user before taking over
- Update status to `[-]` when you start, `[x]` when done
- Add comment: `<!-- Updated by Kiro at [timestamp] -->`

### 3. Terminal Log Awareness

**Before executing commands:**
- Check `.terminal_log` or recent terminal output for Gemini's activities
- If Gemini just ran a command that failed, analyze the error before retrying
- Don't repeat commands Gemini already executed successfully

### 4. File Conflict Prevention

**Before modifying files:**
- Check if file is listed in `GEMINI.md` active files section
- Check file's last modified time - if < 30 seconds, warn user
- If conflict detected, ask: "Gemini CLI is working on [file]. Proceed anyway?"

### 5. Work Area Coordination

**When receiving a task:**
- If Gemini's presence file shows same work area (e.g., "API Refactoring"), inform user
- Suggest complementary work: "Gemini is fixing APIs. Should I work on tests/frontend/docs instead?"
- Update your own presence in `KIRO.md` with your current focus area

### 6. Communication Protocol

**For complex handoffs:**
- Write to `.agent_messages.json` with format:
  ```json
  {
    "sender": "Kiro",
    "timestamp": "ISO-8601",
    "type": "completion_notice|handoff_request|status_update",
    "payload": { "task": "...", "status": "...", "details": "..." }
  }
  ```
- Check this file periodically for messages from Gemini

## Coordination Workflow

1. **Task Start:**
   - Check GEMINI.md presence
   - Check tasks.md status
   - Update KIRO.md with your focus
   - Mark task as `[-]` in tasks.md

2. **During Execution:**
   - Monitor for file conflicts
   - Check terminal logs before running commands
   - Update KIRO.md every 2-3 minutes

3. **Task Completion:**
   - Mark task as `[x]` in tasks.md
   - Update KIRO.md with completion status
   - Write completion_notice if Gemini depends on this

## Graceful Degradation

- If no GEMINI.md exists, work normally (solo mode)
- Don't create coordination files unless Gemini is detected
- Add < 100ms overhead for presence checks
- Never block on coordination - always ask user if uncertain

## Example Scenarios

**Scenario 1: Gemini fixing API endpoints**
```
User: "Fix the authentication bug"
You: "I see Gemini CLI is currently fixing API endpoints in api.ts. 
      Should I wait for that to complete, or work on the auth logic separately?"
```

**Scenario 2: Podman error after Gemini's work**
```
User: "The Podman container won't start"
You: [Check terminal logs]
     "I see Gemini just modified podman-compose.yml 2 minutes ago. 
      Let me review those changes and the error code 125..."
```

**Scenario 3: Parallel work**
```
GEMINI.md shows: "Working on: Infrastructure - Database setup"
You: "Gemini is setting up databases. I'll work on the frontend components 
      to avoid conflicts. Updating KIRO.md..."
```

## Priority Rules

1. **User safety first**: Always ask before overwriting Gemini's recent work
2. **Speed over perfection**: If no conflict, proceed immediately
3. **Transparency**: Always inform user when coordination happens
4. **Fail gracefully**: If coordination fails, continue in solo mode

---

This protocol enables you and Gemini CLI to work as a coordinated team, accelerating development while preventing conflicts.
