---
description: Activate and use the multi-agent smart hooks system to coordinate Antigravity, Gemini CLI, and Qwen CLI
---

# Multi-Agent Smart Hooks Workflow

## Overview
This workflow activates and uses the Dragon-R1 Smart Hooks system that coordinates work between Antigravity (Claude Opus), Gemini CLI, and Qwen CLI.

// turbo-all

## Steps

### 1. Initialize the Hook System
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.setup
```
This creates all config files, generates Gemini/Qwen instructions, and installs git hooks.

### 2. Check Swarm Status
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli status
```

### 3. Route a Task to Best Agent
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli route "YOUR_TASK_DESCRIPTION"
```

### 4. Delegate to Gemini CLI
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli delegate gemini_cli "YOUR_TASK"
```

### 5. Delegate to Qwen CLI
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli delegate qwen_cli "YOUR_TASK"
```

### 6. Start the File Watcher (Background)
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
Start-Process -NoNewWindow python -ArgumentList "-m", "hooks.watcher"
```

### 7. List All Agents
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli agents
```

### 8. Share Context Between Agents
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli context share --category finding --key "key" --value "value"
```

### 9. Read Shared Context
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli context get
```

### 10. Send a Message to Other Agents
```powershell
cd C:\Users\Hossam Galal\Desktop\RED
python -m hooks.cli message post --content "Your message here"
```
