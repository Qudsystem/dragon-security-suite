# Dragon-R1 Testing Quick Reference

## Quick Commands

### Run All Tests
```bash
# Linux/Mac
./run-all-tests.sh

# Windows
run-all-tests.bat
```

### Backend Only
```bash
cd Cerberus/gateway
./run-tests.sh              # Linux/Mac
run-tests.bat               # Windows
```

### Frontend Only
```bash
cd Cerberus/dragon-ui
./scripts/run-tests.sh      # Linux/Mac
scripts\run-tests.bat       # Windows
```

## Coverage Reports

- **Backend**: `Cerberus/gateway/htmlcov/index.html`
- **Frontend**: `Cerberus/dragon-ui/coverage/index.html`

## Test Categories

### Backend (pytest markers)
- `-m unit` - Unit tests
- `-m integration` - Integration tests
- `-m property` - Property-based tests
- `-m "not slow"` - Skip slow tests

### Frontend (npm scripts)
- `npm test` - All tests
- `npm run test:unit` - Unit tests only
- `npm run test:coverage` - With coverage
- `npm run test:watch` - Watch mode

## CI/CD

Tests run automatically on push/PR via GitHub Actions (`.github/workflows/test.yml`)

For full documentation, see `docs/TESTING.md`
