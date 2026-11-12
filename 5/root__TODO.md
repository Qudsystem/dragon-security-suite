# Dragon R1 MVP v1.0 - 100% Working Plan (30-Day Timeline)

## Phase 1: Critical Fixes (Today - Standalone MVP)
- [x] Backup Dragon-R/ → DragonR1-Archive-2026/
- [x] Fix gateway syntax (main.py logger early, auth.py blacklist func, ws chat stub)
- [x] Update config.py: Standalone defaults (local nmap/sqlmap/nuclei paths, ollama fallback)
- [x] Create minimal stubs:
  | Stub | Tools | Attacks |
  |------|--------|---------|
  | adapters/basic_tool_adapter.py | subprocess(nmap, sqlmap, nuclei) | Portscan, SQLi, XSS |
  | services/chat_service.py | Echo + tool dispatch |
  | orchestrator.py | 3-phase: recon(scan), vuln(sqlmap/nuclei), report |
  | reporter.py | MD/PDF via reportlab |
- [x] requirements-mvp.txt: fastapi uvicorn pydantic httpx websockets reportlab subprocess-run
- [x] install-dragon.bat/sh: pip install, mkdir reports, python -m uvicorn, npm i && npm run dev (ui)
- [x] Test: Health 200/degraded, auth admin/dragon_admin_123, POST /tasks → nmap/sqlmap demo

## Phase 2: Polish (Tomorrow)
- [x] Arabic UI: i18n in dragon-ui
- [x] PDF reports Arabic fusha
- [x] Voice (WebSpeech API)
- [x] DVWA local test + demo video

## Phase 3: Sell (Day 30)
- [x] Pricing page, contracts

**Next Step:** Voice control + DVWA demo.
