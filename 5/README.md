# Project File Organization

Safe cleanup performed without moving runtime source code.

## Layout

| Location | Purpose |
|---|---|
| `docs/archive/color-fixes/` | Historical UI color cleanup reports |
| `docs/archive/task-reports/` | Task completion / analysis reports |
| `tmp/dumps/` | Large text/json dumps and inventories |
| `tmp/screenshots/` | Temporary screenshots |
| `scripts/dev/` | One-off fix/test/helper scripts |
| `Cerberus/dragon-ui/docs/archive/` | Frontend historical MD reports |
| `Cerberus/dragon-ui/tmp/logs/` | Frontend build/dev logs and JSON reports |
| `Cerberus/dragon-ui/scripts/dev*` | Frontend helper scripts |
| `Cerberus/gateway/docs/archive/` | Gateway historical MD reports |
| `Cerberus/gateway/tmp/logs/` | Gateway runtime/test logs |
| `Cerberus/gateway/scripts/dev/` | Gateway helper/verify scripts |

## Intentionally kept at roots

- Launchers: `START-DRAGON-R1.bat`, `Cerberus/*/start.ps1`, `START-GATEWAY.bat`
- App source: `Cerberus/dragon-ui/src/**`, `Cerberus/gateway/{api,core,services,main.py,...}`
- Config: `package.json`, `requirements.txt`, `.env*`, `settings.json`, `next.config.mjs`

Nothing was deleted; files were only relocated.
