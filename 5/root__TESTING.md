# Dragon-R1 Testing Guide

This document provides comprehensive testing instructions for the Dragon-R1 project, including TestSprite integration.

## Overview

Dragon-R1 has a comprehensive test suite that includes:
- **Backend Tests**: Python/FastAPI tests using pytest
- **Frontend Tests**: Next.js E2E tests using Playwright
- **TestSprite Integration**: AI-powered test automation based on your existing test plan

## Quick Start

### 1. Run All Tests
```bash
# Using Python script
python run_dragon_tests.py all

# Using batch script (Windows)
run_dragon_tests.bat all

# Using npm scripts (from frontend directory)
cd Cerberus/dragon-ui
npm run test:all
```

### 2. Run Frontend Tests Only
```bash
# All frontend tests
python run_dragon_tests.py frontend

# Specific category (chat, dashboard, projects, reports, graph)
python run_dragon_tests.py frontend chat
python run_dragon_tests.py frontend dashboard
```

### 3. Run Backend Tests Only
```bash
python run_dragon_tests.py backend
```

### 4. Run TestSprite Integration Tests
```bash
python run_dragon_tests.py testsprite
```

## Test Categories

Based on your TestSprite test plan, tests are organized into these categories:

### 1. Interactive Agent Chat (TC001-TC005)
- Real-time chat functionality
- WebSocket connection status
- Message validation and ordering

### 2. Security Dashboard (TC006-TC012)
- Dashboard overview
- Task management
- Error handling and retry logic

### 3. Task & Project Management (TC013-TC016)
- Project creation and validation
- Task management workflows
- Form validation

### 4. Vulnerability Reporting (TC017-TC023)
- Report generation and viewing
- Error handling for generation failures
- Report navigation

### 5. Agent Graph Visualization (TC024-TC030)
- Graph rendering and interaction
- Node details and navigation
- Filtering and zoom controls

## TestSprite Integration

### Existing Test Plan
Your TestSprite test plan is located at:
```
Cerberus/dragon-ui/testsprite_tests/testsprite_frontend_test_plan.json
```

This contains 30+ comprehensive test cases covering all major features.

### TestSprite MCP Configuration
TestSprite MCP is configured in `.kiro/settings/mcp.json` with your API key.

To use TestSprite MCP:
1. Ensure TestSprite is enabled in MCP settings
2. Use the TestSprite tools through Kiro's MCP interface
3. Run tests using the TestSprite CLI or API

### Converting TestSprite Tests to Playwright
The existing Playwright tests in `Cerberus/dragon-ui/e2e/` are based on your TestSprite test plan. Each test file corresponds to a test category.

## Running Tests in Different Environments

### Development Environment
```bash
# Start the full stack first
.\START-DRAGON-R1.bat  # Windows
./start-dragon-r1.sh   # Linux/Mac

# Then run tests
python run_dragon_tests.py all
```

### CI/CD Pipeline
```bash
# Install dependencies
cd Cerberus/dragon-ui && npm install
cd Cerberus/gateway && pip install -r requirements.txt

# Run tests
python run_dragon_tests.py all
```

### Headless Mode (for CI)
```bash
cd Cerberus/dragon-ui
npm run test:e2e  # Runs Playwright in headless mode
```

### Debug Mode
```bash
cd Cerberus/dragon-ui
npm run test:e2e:debug  # Opens Playwright debug UI
npm run test:e2e:ui     # Opens Playwright test runner UI
```

## Test Reports

### Playwright Reports
After running E2E tests, view the HTML report:
```bash
cd Cerberus/dragon-ui
npm run test:e2e:report
```

### TestSprite Reports
TestSprite generates comprehensive reports including:
- Test execution results
- Screenshots and videos
- Performance metrics
- Accessibility scores

## Test Data Management

### Prerequisites
Before running tests, ensure:
1. Gateway is running on port 8000
2. Frontend is running on port 3000
3. Database services are available (if needed)
4. Test data is seeded (if required)

### Test Data Setup
```bash
# Setup test databases
.\setup-databases.bat  # Windows
./setup-databases.sh   # Linux/Mac

# Seed test data (if applicable)
cd Cerberus/gateway
python scripts/seed_test_data.py
```

## Troubleshooting

### Common Issues

1. **Frontend not running**
   ```bash
   cd Cerberus/dragon-ui
   npm run dev
   ```

2. **Gateway not running**
   ```bash
   cd Cerberus/gateway
   python main.py
   # or
   uvicorn main:app --reload --host 0.0.0.0 --port 8000
   ```

3. **Playwright browsers not installed**
   ```bash
   cd Cerberus/dragon-ui
   npx playwright install
   ```

4. **Python dependencies missing**
   ```bash
   cd Cerberus/gateway
   python -m venv .venv
   .venv\Scripts\activate  # Windows
   source .venv/bin/activate  # Linux/Mac
   pip install -r requirements.txt
   ```

### TestSprite Issues

1. **TestSprite MCP not connecting**
   - Check API key in `.kiro/settings/mcp.json`
   - Ensure TestSprite server is not disabled
   - Verify internet connection

2. **Test plan not found**
   - Check path to test plan file
   - Ensure test plan JSON is valid

## Best Practices

1. **Run tests before committing**
   ```bash
   python run_dragon_tests.py all
   ```

2. **Test specific features when developing**
   ```bash
   python run_dragon_tests.py frontend chat
   ```

3. **Use TestSprite for comprehensive testing**
   - Leverage AI-powered test generation
   - Use visual regression testing
   - Generate detailed test reports

4. **Maintain test data separately**
   - Use fixtures for test data
   - Clean up after tests
   - Avoid hardcoding test data

## Advanced Testing

### Property-Based Testing
Dragon-R1 includes property-based tests using FastCheck:
```bash
cd Cerberus/dragon-ui
npm test  # Runs Jest with property-based tests
```

### Performance Testing
```bash
# Run performance tests
cd Cerberus/gateway
python -m pytest tests/test_performance.py
```

### Security Testing
```bash
# Run security scans
cd Cerberus/gateway
python -m pytest tests/test_security.py
```

## Support

For testing issues:
1. Check the troubleshooting section
2. Review test logs in `test-reports/` directory
3. Consult the AGENTS.md file for project architecture
4. Check TestSprite documentation at https://docs.testsprite.com

## Test Coverage Goals

- Backend: 80%+ code coverage
- Frontend: 80%+ code coverage
- E2E: All critical user journeys
- TestSprite: Comprehensive feature coverage