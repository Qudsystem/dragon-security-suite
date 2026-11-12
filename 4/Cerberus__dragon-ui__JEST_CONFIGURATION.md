# Jest Configuration Documentation

## Overview

This document describes the Jest test configuration for the Dragon-R1 frontend application.

## Configuration Files

### jest.config.js

Main Jest configuration file with the following key settings:

- **Test Environment**: `jest-environment-jsdom` for React component testing
- **Module Mapping**: `@/` alias maps to `src/` directory
- **Memory Management**:
  - `maxWorkers: 2` - Limits parallel test execution to prevent memory issues
  - `workerIdleMemoryLimit: 512MB` - Restarts workers when memory exceeds limit
  - `maxConcurrency: 5` - Limits concurrent tests per worker
- **Test Timeout**: 30 seconds for property-based tests
- **Coverage Threshold**: 80% for all metrics (branches, functions, lines, statements)

### jest.setup.js

Test environment setup with the following mocks:

1. **Global Polyfills**:
   - TextEncoder/TextDecoder for Next.js compatibility
   - Request/Response/Headers for Next.js API routes

2. **Browser APIs**:
   - fetch with proper response structure
   - localStorage and sessionStorage
   - WebSocket for real-time communication
   - IntersectionObserver and ResizeObserver
   - matchMedia for responsive design tests
   - Canvas API for xterm and graph visualization

3. **Console Suppression**:
   - Filters expected warnings and errors during tests
   - Reduces noise in test output

## Module Structure

### Index Files

Created index files for better module organization:

- `src/lib/auth/index.ts` - Authentication exports
- `src/lib/caching/index.ts` - Cache management exports
- `src/lib/hooks/index.ts` - Custom React hooks exports
- `src/lib/monitoring/index.ts` - Metrics and monitoring exports
- `src/lib/resilience/index.ts` - Circuit breaker and retry logic exports
- `src/lib/security/index.ts` - Security utilities exports

### Mock Files

Created mock implementations for testing:

- `src/lib/__mocks__/api.ts` - Mock DragonAPI class
- `src/lib/__mocks__/websocket.ts` - Mock DragonWebSocket class

## Test Execution

### Running Tests

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run in watch mode
npm run test:watch

# Run specific test file
npm test -- path/to/test.test.ts

# Run tests matching pattern
npm test -- --testNamePattern="pattern"
```

### Ignored Tests

The following tests are ignored as they require external services:

- `integration/end-to-end.test.ts` - Requires backend API
- `environment-bug-exploration.property.test.ts` - Requires Podman/Docker
- `integration-bug-exploration.property.test.ts` - Requires backend services

## Common Issues and Solutions

### Issue: Module Resolution Errors

**Solution**: Ensure index files exist in module directories and export all necessary components.

### Issue: Memory Crashes

**Solution**: Configuration already includes memory management settings. If issues persist:
- Reduce `maxWorkers` to 1
- Increase `workerIdleMemoryLimit`
- Run tests in smaller batches

### Issue: Timeout Errors

**Solution**: Increase `testTimeout` in jest.config.js or use `jest.setTimeout()` in specific tests.

### Issue: Missing Global APIs

**Solution**: Add mocks to jest.setup.js for any missing browser APIs.

## Best Practices

1. **Isolation**: Each test should be independent and not rely on other tests
2. **Mocking**: Mock external dependencies (API calls, WebSocket, etc.)
3. **Cleanup**: Use `afterEach` to clean up state between tests
4. **Descriptive Names**: Use clear, descriptive test names
5. **Property-Based Testing**: Use fast-check for comprehensive input validation

## Coverage Requirements

All code must maintain 80% coverage across:
- Branches
- Functions
- Lines
- Statements

Exceptions can be added to `collectCoverageFrom` in jest.config.js.

## Maintenance

When adding new modules:
1. Create index.ts file for exports
2. Add mocks in `__mocks__` directory if needed
3. Update jest.setup.js if new global APIs are required
4. Add to testPathIgnorePatterns if external services are needed
