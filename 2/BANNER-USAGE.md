# Dragon-R1 Banner Template Usage Guide

## 📋 Overview

This guide explains how to use the standardized Dragon-R1 banner in all scripts and tools.

## 🎨 Banner Template

The banner template is located at: `Cerberus/scripts/banner-template.bat`

## 📝 How to Use

### For New BAT Scripts:

1. Copy the template:
```cmd
copy Cerberus\scripts\banner-template.bat your-new-script.bat
```

2. Edit the script title:
```batch
echo │ [SCRIPT TITLE HERE]                                                      │
```
Replace `[SCRIPT TITLE HERE]` with your script name.

3. Add your script logic after the banner section.

### Manual Implementation:

If you want to add the banner to an existing script, copy this code block:

```batch
@echo off
REM Enable UTF-8 encoding for proper display
chcp 65001 >nul 2>&1

echo.
echo ╔═══════════════════════════════════════════════════════════════════════════╗
echo ║                                                                           ║
echo ║    ██████╗ ██████╗  █████╗  ██████╗  ██████╗ ███╗   ██╗    ██████╗   ██╗  ║
echo ║    ██╔══██╗██╔══██╗██╔══██╗██╔════╝ ██╔═══██╗████╗  ██║    ██╔══██╗ ╚██╗  ║
echo ║    ██║  ██║██████╔╝███████║██║  ███╗██║   ██║██╔██╗ ██║    ██████╔╝ ╚██╗  ║
echo ║    ██║  ██║██╔══██╗██╔══██║██║   ██║██║   ██║██║╚██╗██║    ██╔══██╗  ██╔╝ ║
echo ║    ██████╔╝██║  ██║██║  ██║╚██████╔╝╚██████╔╝██║ ╚████║    ██║  ██║  ██╔╝ ║
echo ║    ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝    ╚═╝  ╚═╝  ╚═╝  ║
echo ║                                                                           ║
echo ║              🐉 Autonomous Security Platform - Full Stack 🐉             ║
echo ║                                                                           ║
echo ╚═══════════════════════════════════════════════════════════════════════════╝
echo.
echo ┌───────────────────────────────────────────────────────────────────────────┐
echo │ Your Script Title Here                                                   │
echo └───────────────────────────────────────────────────────────────────────────┘
echo.
```

## 🔧 Customization

### Script Title
Change this line to match your script's purpose:
```batch
echo │ Your Script Title Here                                                   │
```

### Important Notes:
- Keep the title under 70 characters for proper alignment
- Use spaces to pad the title if needed
- The banner uses UTF-8 encoding (`chcp 65001`)

## 📚 Examples

### Example 1: Database Backup Script
```batch
echo │ Dragon-R1 Database Backup Tool                                           │
```

### Example 2: Log Analyzer
```batch
echo │ Dragon-R1 Log Analysis & Reporting Tool                                  │
```

### Example 3: Configuration Manager
```batch
echo │ Dragon-R1 Configuration Manager                                          │
```

## ✅ Scripts Using This Banner

Current scripts with the standardized banner:

1. ✅ `Cerberus/scripts/banner-template.bat`

> **Note:** Docker-based startup scripts have been removed. The project now runs natively on WSL Ubuntu Linux.

## 🎯 Best Practices

1. **Always use UTF-8 encoding** - Include `chcp 65001 >nul 2>&1` at the start
2. **Keep titles descriptive** - Users should understand the script's purpose immediately
3. **Maintain consistency** - Use the same banner format across all scripts
4. **Test display** - Run the script to ensure the banner displays correctly
5. **Update this list** - Add new scripts to the list above when created

## 🚀 Quick Start for New Scripts

```cmd
REM 1. Copy template
copy Cerberus\scripts\banner-template.bat my-new-tool.bat

REM 2. Edit the title
notepad my-new-tool.bat

REM 3. Add your code after the banner section

REM 4. Test it
my-new-tool.bat
```

## 📞 Support

If you encounter display issues:
- Ensure your terminal supports UTF-8
- Check that `chcp 65001` is executed
- Verify the terminal font supports Unicode characters

---

**Dragon-R1** - Autonomous Security Platform 🐉
