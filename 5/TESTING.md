# Dragon-R1 Testing Guide

This document provides comprehensive guidance on testing the Dragon-R1 platform.

## Table of Contents

1. [Overview](#overview)
2. [Test Structure](#test-structure)
3. [Running Tests](#running-tests)
4. [Coverage Reports](#coverage-reports)
5. [Writing Tests](#writing-tests)
6. [CI/CD Integration](#cicd-integration)

## Overview

Dragon-R1 uses a multi-layered testing approach:

- **Unit Tests**: Test individual functions and components in isolation
- **Integration Tests**: Test interactions between components and external services
- **Property-Based Tests**: Test correctness properties using Hypothesis (Python) and fast-check (TypeScript)
- **End-to-End Tests**: Test complete user workflows (Playwright)

### Coverage Thresholds

- **Backend (Python)**: Target 80%+ coverage
- **Frontend (TypeScript)**: Target 80%+ coverage (enforced in jest.config.js)

## Test Structure

### Backend (Cerberus/gateway)

```
tests/
├── unit/                    # Unit tests
├── integration/             # Integration tests
├── property/                # Property-based tests
├── exploration/             # Bug exploration tests
└── preservation/            # Regression prevention tests
```

### Frontend (Cerberus/dragon-ui)

```
src/__tests__/
├── unit/                    # Component and utility tests
├── integration/             # API integration tests
└── property/                # Property-based tests

e2e/                         # Playwright end-to-end tests
```

## Running Tests

### Quick Start

Run all tests across the entire project:

```bash
# Linux/Mac
./run-all-tests.sh

# Windows
run-all-tests.bat
```

### Backend Tests

```bash
cd Cerberus/gateway

# Run all tests with coverage
./run-tests.sh              # Linux/Mac
run-tests.bat               # Windows

# Or use pytest directly
pytest                      # All tests
pytest -m unit              # Unit tests only
pytest -m integration       # Integration tests only
pytest -m property          # Property-based tests only

# With coverage
pytest --cov=. --cov-report=html --cov-report=term
```

### Frontend Tests

```bash
cd Cerberus/dragon-ui

# Run all tests with coverage
./scripts/run-tests.sh      # Linux/Mac
scripts\run-tests.bat       # Windows

# Or use npm scripts
npm test                    # All tests
npm run test:unit           # Unit tests only
npm run test:integration    # Integration tests only
npm run test:coverage       # With coverage report
npm run test:watch          # Watch mode
npm run test:ci             # CI mode (no watch, with coverage)
```

### End-to-End Tests

```bash
cd Cerberus/dragon-ui

npm run test:e2e            # Run Playwright tests
npm run test:e2e:ui         # Run with UI
npm run test:e2e:headed     # Run in headed mode
npm run test:e2e:debug      # Debug mode
```

## Coverage Reports

### Viewing Coverage Reports

After running tests with coverage, reports are generated in:

**Backend:**
- HTML: `Cerberus/gateway/htmlcov/index.html`
- XML: `Cerberus/gateway/coverage.xml`
- Terminal: Displayed after test run

**Frontend:**
- HTML: `Cerberus/dragon-ui/coverage/index.html`
- LCOV: `Cerberus/dragon-ui/coverage/lcov.info`
- Terminal: Displayed after test run

### Serving Coverage Reports

```bash
# Backend
cd Cerberus/gateway
python -m http.server 8080 --directory htmlcov
# Open http://localhost:8080

# Frontend
cd Cerberus/dragon-ui
npx http-server coverage -p 8081
# Open http://localhost:8081
```

## Writing Tests

### Backend (Python)

#### Unit Test Example

```python
import pytest
from my_module import my_function

@pytest.mark.unit
def test_my_function():
    """Test that my_function works correctly"""
    result = my_function(input_value)
    assert result == expected_value
```

#### Integration Test Example

```python
import pytest
import httpx

@pytest.mark.integration
@pytest.mark.requires_redis
@pytest.mark.asyncio
async def test_redis_integration():
    """Test Redis integration"""
    import redis.asyncio as redis
    client = redis.Redis(host='localhost', port=6379)
    await client.ping()
    await client.close()
```

#### Property-Based Test Example

```python
import pytest
from hypothesis import given, strategies as st

@pytest.mark.property
@given(st.integers(), st.integers())
def test_addition_commutative(a, b):
    """Test that addition is commutative"""
    assert a + b == b + a
```

### Frontend (TypeScript)

#### Unit Test Example

```typescript
import { render, screen } from '@testing-library/react';
import { MyComponent } from './MyComponent';

describe('MyComponent', () => {
  it('should render correctly', () => {
    render(<MyComponent />);
    expect(screen.getByText('Hello')).toBeInTheDocument();
  });
});
```

#### Integration Test Example

```typescript
describe('API Integration', () => {
  it('should fetch data from API', async () => {
    const response = await fetch('http://localhost:8000/api/v1/health');
    expect(response.ok).toBe(true);
    const data = await response.json();
    expect(data).toHaveProperty('status');
  });
});
```

#### Property-Based Test Example

```typescript
import fc from 'fast-check';

describe('String utilities', () => {
  it('should reverse strings correctly', () => {
    fc.assert(
      fc.property(fc.string(), (str) => {
        const reversed = reverseString(str);
        const doubleReversed = reverseString(reversed);
        return doubleReversed === str;
      })
    );
  });
});
```

## Test Markers (Backend)

Use pytest markers to categorize tests:

- `@pytest.mark.unit` - Unit tests
- `@pytest.mark.integration` - Integration tests
- `@pytest.mark.property` - Property-based tests
- `@pytest.mark.slow` - Slow-running tests
- `@pytest.mark.requires_redis` - Requires Redis
- `@pytest.mark.requires_neo4j` - Requires Neo4j
- `@pytest.mark.requires_ollama` - Requires Ollama
- `@pytest.mark.requires_postgres` - Requires PostgreSQL

## CI/CD Integration

### GitHub Actions

Tests run automatically on push and pull requests via `.github/workflows/test.yml`:

- Backend tests run on Python 3.10 and 3.11
- Frontend tests run on Node.js 18.x and 20.x
- Integration tests run with Redis and PostgreSQL services
- Coverage reports are uploaded to Codecov

### Local CI Simulation

Run tests in CI mode locally:

```bash
# Backend
cd Cerberus/gateway
pytest --cov=. --cov-report=xml --maxfail=5

# Frontend
cd Cerberus/dragon-ui
npm run test:ci
```

## Troubleshooting

### Common Issues

**Backend:**

1. **Import errors**: Ensure virtual environment is activated and dependencies installed
2. **Service connection failures**: Check that required services (Redis, Neo4j, etc.) are running
3. **Async test failures**: Ensure `pytest-asyncio` is installed and `@pytest.mark.asyncio` is used

**Frontend:**

1. **Memory crashes**: Jest is configured with `maxWorkers: 2` and `workerIdleMemoryLimit: 512MB`
2. **Module resolution**: Check that `@/` alias is properly configured in `jest.config.js`
3. **Timeout errors**: Increase `testTimeout` in `jest.config.js` for slow tests

### Performance Optimization

**Backend:**
- Use `pytest-xdist` for parallel test execution: `pytest -n auto`
- Skip slow tests during development: `pytest -m "not slow"`
- Use `--maxfail=N` to stop after N failures

**Frontend:**
- Use `--maxWorkers=2` to limit memory usage
- Run specific test files: `npm test -- path/to/test.ts`
- Use watch mode for development: `npm run test:watch`

## Best Practices

1. **Write tests first** (TDD) when fixing bugs or adding features
2. **Keep tests isolated** - no shared state between tests
3. **Use descriptive test names** - explain what is being tested
4. **Mock external dependencies** in unit tests
5. **Test edge cases** and error conditions
6. **Maintain high coverage** but focus on meaningful tests
7. **Run tests before committing** code
8. **Review coverage reports** to identify untested code paths

## Resources

- [Pytest Documentation](https://docs.pytest.org/)
- [Jest Documentation](https://jestjs.io/)
- [Hypothesis Documentation](https://hypothesis.readthedocs.io/)
- [fast-check Documentation](https://fast-check.dev/)
- [Playwright Documentation](https://playwright.dev/)
- [Testing Library Documentation](https://testing-library.com/)
