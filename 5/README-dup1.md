# Integration Tests

This directory contains end-to-end integration tests for the Dragon-R1 Frontend Security system.

## Overview

The integration tests validate complete workflows across the application, including:

1. **Task Creation and Execution Flow** - Complete lifecycle from project creation to task execution
2. **Approval Workflow** - End-to-end approval process for exploitation actions
3. **Graph Visualization** - Real-time graph updates and node interactions
4. **Chat with File Upload** - Chat interface with file upload and agent responses
5. **Report Generation and Export** - Report creation and export to multiple formats
6. **Security Event Detection** - Prompt injection detection and security monitoring
7. **Performance and Caching** - Redis caching and performance optimization
8. **Circuit Breaker Protection** - Fault tolerance and resilience
9. **Authentication and Authorization** - JWT authentication and role-based access
10. **Error Handling** - Comprehensive error handling and validation
11. **WebSocket Resilience** - Connection management and reconnection
12. **Metrics Collection** - Prometheus metrics and monitoring

## Prerequisites

Before running integration tests, ensure the following services are running:

1. **FastAPI Gateway** - Backend API server (port 8000)
2. **Neo4j** - Graph database
3. **Redis** - Caching layer
4. **mem0** - Memory service
5. **Prometheus** - Metrics collection (optional)

### Starting Services

```bash
# From the Cerberus directory — start services natively
cd Cerberus/gateway
python main.py

# In another terminal, start the UI
cd Cerberus/dragon-ui
npm run dev
```

## Running Tests

### Run All Integration Tests

```bash
cd Cerberus/dragon-ui
npm test -- end-to-end.test.ts
```

### Run with Extended Timeout

```bash
npm test -- end-to-end.test.ts --testTimeout=60000
```

### Run Specific Test

```bash
npm test -- end-to-end.test.ts -t "Complete task creation"
```

### Run with Coverage

```bash
npm test -- end-to-end.test.ts --coverage
```

## Environment Configuration

Create a `.env.test` file in the `dragon-ui` directory:

```env
NEXT_PUBLIC_API_URL=http://localhost:8000
NEXT_PUBLIC_WS_URL=ws://localhost:8000
TEST_USERNAME=test_user
TEST_PASSWORD=test_password
```

## Test Data

The tests create temporary test data:
- Test projects
- Test tasks
- Test files
- Test reports

All test data is automatically cleaned up in the `afterAll` hook.

## Expected Results

When all services are running:
- **10-12 tests should pass**
- **0 tests should fail**
- **Total execution time: ~60-90 seconds**

### Common Issues

#### Authentication Failures
```
Skipping test: No auth token available
```
**Solution**: Ensure the backend is running and test credentials are correct.

#### Network Errors
```
AxiosError: Network Error
```
**Solution**: Verify the API_URL is correct and the backend is accessible.

#### WebSocket Connection Failures
```
WebSocket connection timeout
```
**Solution**: Check that WebSocket support is enabled in the backend.

#### Timeout Errors
```
Timeout - Async callback was not invoked
```
**Solution**: Increase test timeout with `--testTimeout=120000`

## Test Structure

Each test follows this pattern:

1. **Setup** - Authenticate and prepare test data
2. **Execute** - Perform the workflow actions
3. **Verify** - Assert expected outcomes
4. **Cleanup** - Remove test data

## Continuous Integration

To run these tests in CI/CD:

```yaml
# .github/workflows/integration-tests.yml
name: Integration Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    
    services:
      neo4j:
        image: neo4j:latest
        env:
          NEO4J_AUTH: neo4j/password
      redis:
        image: redis:latest
    
    steps:
      - uses: actions/checkout@v2
      - uses: actions/setup-node@v2
        with:
          node-version: '18'
      
      - name: Install dependencies
        run: |
          cd Cerberus/dragon-ui
          npm install
      
      - name: Start backend
        run: |
          cd Cerberus/gateway
          pip install -r requirements.txt
          python main.py &
          sleep 10
      
      - name: Run integration tests
        run: |
          cd Cerberus/dragon-ui
          npm test -- end-to-end.test.ts --testTimeout=120000
```

## Debugging

Enable verbose logging:

```bash
DEBUG=* npm test -- end-to-end.test.ts
```

View WebSocket messages:

```bash
npm test -- end-to-end.test.ts --verbose
```

## Performance Benchmarks

Expected performance metrics:

| Test | Expected Duration | Max Duration |
|------|------------------|--------------|
| Task Creation | < 5s | 10s |
| Approval Workflow | < 10s | 30s |
| Graph Visualization | < 5s | 15s |
| Chat with Upload | < 5s | 20s |
| Report Generation | < 10s | 30s |
| Security Detection | < 5s | 20s |
| Performance/Caching | < 5s | 15s |
| Circuit Breaker | < 3s | 10s |
| Authentication | < 3s | 10s |
| Error Handling | < 5s | 15s |
| WebSocket Resilience | < 5s | 10s |
| Metrics Collection | < 2s | 5s |

## Contributing

When adding new integration tests:

1. Follow the existing test structure
2. Include proper cleanup in `afterAll`
3. Add descriptive test names
4. Document expected behavior
5. Handle authentication gracefully
6. Use appropriate timeouts
7. Validate all requirements

## Support

For issues or questions:
- Check the main README.md
- Review the design document
- Contact the development team
