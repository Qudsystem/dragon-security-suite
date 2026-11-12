# Jest Memory Crash Fix Design

## Overview

The Jest test suite crashes due to memory exhaustion when running 303 tests across 39 test suites. The fix involves adding memory management controls to the Jest configuration: implementing `workerIdleMemoryLimit` to restart workers before memory exhaustion, optimizing the worker count, and enabling Node.js garbage collection flags. This approach prevents memory accumulation in long-running test workers while maintaining all existing test behavior and coverage requirements.

## Glossary

- **Bug_Condition (C)**: The condition that triggers the bug - when Jest workers accumulate memory beyond available system resources during extended test execution
- **Property (P)**: The desired behavior when running the full test suite - all tests complete successfully without memory-related crashes
- **Preservation**: Existing test logic, assertions, coverage thresholds, and test results that must remain unchanged by the configuration fix
- **workerIdleMemoryLimit**: Jest configuration option that restarts workers when they exceed a specified memory threshold while idle
- **maxWorkers**: Jest configuration option that controls the number of concurrent worker processes
- **Worker Process**: A separate Node.js process that Jest spawns to run tests in parallel
- **Memory Exhaustion**: When a worker process consumes all available memory and crashes with "ran out of memory" error
- **Property-Based Tests**: Tests using fast-check that generate many test cases, contributing to memory accumulation

## Bug Details

### Fault Condition

The bug manifests when Jest workers execute the full test suite (303 tests, 39 suites) without memory limits over an extended period (~20 minutes). The Jest configuration is either missing the `workerIdleMemoryLimit` setting, running too many concurrent workers without memory constraints, or not enabling garbage collection flags to manage memory lifecycle.

**Formal Specification:**
```
FUNCTION isBugCondition(input)
  INPUT: input of type TestExecutionContext
  OUTPUT: boolean
  
  RETURN input.testSuiteSize >= 303
         AND input.executionDuration >= 20 minutes
         AND input.config.workerIdleMemoryLimit IS NULL
         AND input.config.maxWorkers allows unbounded memory growth
         AND NOT workerMemoryManagementEnabled(input.config)
END FUNCTION
```

### Examples

- **Full Suite Execution**: Running `npm test` executes all 39 test suites, after ~20 minutes workers crash with "Jest worker ran out of memory and crashed at ChildProcessWorker._onExit", resulting in 29 failed suites and 84 failed tests
- **Property-Based Test Execution**: Tests using fast-check generate thousands of test cases per property, accumulating memory in workers without garbage collection, eventually causing crashes
- **Parallel Worker Execution**: With maxWorkers: '50%' (typically 4-8 workers on modern machines), multiple workers accumulate memory simultaneously, exhausting system resources faster
- **Edge Case - Small Test Runs**: Running individual test files or small subsets completes successfully because workers don't accumulate enough memory to crash before tests finish

## Expected Behavior

### Preservation Requirements

**Unchanged Behaviors:**
- All 303 existing tests must continue to run with identical test logic and assertions
- Property-based tests must continue to use fast-check with the same generators and properties
- Coverage thresholds (80% for branches, functions, lines, statements) must remain enforced
- Test results and pass/fail status must remain the same for all tests (when memory doesn't crash)
- Test execution order and parallelization behavior must remain functionally equivalent
- All test environment setup (jsdom, setupFilesAfterEnv, moduleNameMapper) must remain unchanged

**Scope:**
All aspects of test execution that do NOT involve worker memory management should be completely unaffected by this fix. This includes:
- Test file discovery and matching patterns
- Test environment configuration (jsdom)
- Module resolution and transformation
- Coverage collection and reporting
- Test setup and teardown logic

## Hypothesized Root Cause

Based on the bug description and Jest configuration analysis, the most likely issues are:

1. **Missing workerIdleMemoryLimit**: The configuration lacks the `workerIdleMemoryLimit` option, which allows workers to accumulate unbounded memory until they crash. Jest workers are never restarted based on memory usage, leading to inevitable exhaustion during long test runs.

2. **Suboptimal Worker Count**: Using `maxWorkers: '50%'` may spawn too many concurrent workers (e.g., 8 workers on a 16-core machine), each accumulating memory independently. Reducing to a fixed lower number (e.g., 2-4 workers) would reduce total memory pressure.

3. **No Garbage Collection Optimization**: Node.js garbage collection is not explicitly enabled or optimized for test workers. Adding `--expose-gc` flag would allow explicit garbage collection, and `--max-old-space-size` would set memory limits per worker.

4. **Property-Based Test Memory Accumulation**: Fast-check generates many test cases per property test, and without proper memory management, these accumulate in worker memory. The combination of 303 tests with property-based testing creates significant memory pressure over time.

## Correctness Properties

Property 1: Fault Condition - Test Suite Completes Without Memory Crashes

_For any_ test execution where the full test suite (303 tests, 39 suites) runs for extended periods, the fixed Jest configuration SHALL complete all tests successfully without "Jest worker ran out of memory" errors by enforcing workerIdleMemoryLimit and optimizing worker count.

**Validates: Requirements 2.1, 2.2, 2.3, 2.4**

Property 2: Preservation - Test Behavior and Results Unchanged

_For any_ test execution configuration that does NOT involve worker memory management settings (test logic, assertions, coverage, environment setup), the fixed configuration SHALL produce exactly the same test results, coverage reports, and behavior as the original configuration, preserving all existing test functionality.

**Validates: Requirements 3.1, 3.2, 3.3, 3.4**

## Fix Implementation

### Changes Required

Assuming our root cause analysis is correct:

**File**: `Cerberus/dragon-ui/jest.config.js`

**Configuration Object**: `customJestConfig`

**Specific Changes**:

1. **Add workerIdleMemoryLimit**: Set to a conservative value (e.g., 512MB or 1024MB) to restart workers before memory exhaustion
   - Add `workerIdleMemoryLimit: '512MB'` to the configuration object
   - This will cause Jest to restart workers when they exceed this limit while idle
   - Value should be tuned based on available system memory (start conservative, increase if needed)

2. **Optimize maxWorkers**: Change from percentage-based to fixed count to limit total memory footprint
   - Change `maxWorkers: '50%'` to `maxWorkers: 2` or `maxWorkers: 4`
   - Reduces total concurrent memory usage at the cost of slightly longer test execution time
   - Fixed count provides predictable memory usage regardless of CPU count

3. **Add Node.js Memory Flags**: Configure `testEnvironmentOptions` or use a custom test runner script to pass Node.js flags
   - Add `--expose-gc` to enable explicit garbage collection
   - Add `--max-old-space-size=4096` to set per-worker memory limit (4GB)
   - These flags help Node.js manage memory more aggressively during test execution

4. **Optional: Add maxConcurrency**: Limit concurrent tests within each worker
   - Add `maxConcurrency: 5` to limit how many tests run simultaneously in a single worker
   - Reduces memory spikes from parallel test execution within workers

5. **Document Configuration**: Add comments explaining the memory management settings
   - Help future maintainers understand why these specific values were chosen
   - Document how to tune settings if memory issues recur

### Recommended Configuration

```javascript
const customJestConfig = {
  // ... existing config ...
  
  // Memory management settings to prevent worker crashes
  maxWorkers: 2, // Reduced from '50%' to limit total memory footprint
  workerIdleMemoryLimit: '512MB', // Restart workers before memory exhaustion
  
  // Optional: limit concurrent tests within each worker
  maxConcurrency: 5,
  
  // ... rest of existing config ...
}
```

### Alternative: Node.js Flags via package.json

If the above configuration is insufficient, add Node.js flags to the test script:

```json
{
  "scripts": {
    "test": "NODE_OPTIONS='--expose-gc --max-old-space-size=4096' jest"
  }
}
```

## Testing Strategy

### Validation Approach

The testing strategy follows a two-phase approach: first, surface counterexamples that demonstrate the bug on unfixed code by running the full test suite and observing memory crashes, then verify the fix works correctly by running the same suite with new configuration and confirming completion without crashes while preserving all test results.

### Exploratory Fault Condition Checking

**Goal**: Surface counterexamples that demonstrate the bug BEFORE implementing the fix. Confirm that the current configuration causes memory crashes during full test suite execution. If we don't observe crashes, we may need to re-hypothesize or the bug may be intermittent.

**Test Plan**: Run the full Jest test suite with the UNFIXED configuration (current jest.config.js) and monitor for memory crashes. Document the failure point, number of failed suites, and error messages. This confirms the bug exists and provides a baseline for comparison.

**Test Cases**:
1. **Full Suite Execution**: Run `npm test` with current config (will fail with memory crash after ~20 minutes)
2. **Memory Monitoring**: Use `node --inspect` or system monitoring to observe memory growth in workers (will show unbounded growth)
3. **Partial Suite Execution**: Run subsets of tests (e.g., 10 suites) to confirm they pass without memory issues (should pass, confirming bug is related to extended execution)
4. **Property-Based Test Focus**: Run only property-based tests to see if they contribute disproportionately to memory usage (may fail faster than other tests)

**Expected Counterexamples**:
- "Jest worker ran out of memory and crashed at ChildProcessWorker._onExit" error after ~20 minutes
- 29 failed test suites and 84 failed tests due to worker crashes
- Possible causes: missing workerIdleMemoryLimit, too many concurrent workers, no garbage collection optimization

### Fix Checking

**Goal**: Verify that for all inputs where the bug condition holds (full test suite execution), the fixed configuration produces the expected behavior (all tests complete without memory crashes).

**Pseudocode:**
```
FOR ALL testExecution WHERE isBugCondition(testExecution) DO
  result := runJestWithFixedConfig(testExecution)
  ASSERT result.allTestsCompleted = true
  ASSERT result.memoryErrors = 0
  ASSERT result.failedSuites = 0 (due to memory)
END FOR
```

**Test Plan**: Run the full Jest test suite with the FIXED configuration and verify:
- All 39 test suites complete execution
- No "Jest worker ran out of memory" errors occur
- All tests that previously passed still pass
- Test execution completes within reasonable time (may be slower due to reduced workers)

**Test Cases**:
1. **Full Suite with Fixed Config**: Run `npm test` with new configuration, verify completion without crashes
2. **Extended Execution**: Run tests multiple times in succession to verify workers are properly restarted
3. **Memory Monitoring**: Monitor worker memory usage to confirm it stays below workerIdleMemoryLimit
4. **Performance Baseline**: Measure test execution time to ensure it's acceptable (may increase slightly)

### Preservation Checking

**Goal**: Verify that for all inputs where the bug condition does NOT hold (test logic, assertions, coverage, environment), the fixed configuration produces the same result as the original configuration.

**Pseudocode:**
```
FOR ALL testAspect WHERE NOT isMemoryManagementSetting(testAspect) DO
  ASSERT runWithOriginalConfig(testAspect) = runWithFixedConfig(testAspect)
END FOR
```

**Testing Approach**: Property-based testing is recommended for preservation checking because:
- It generates many test cases automatically across the input domain
- It catches edge cases that manual unit tests might miss
- It provides strong guarantees that behavior is unchanged for all non-memory-management aspects

**Test Plan**: Observe behavior on UNFIXED code first for test results and coverage, then verify the FIXED configuration produces identical results (excluding memory crashes).

**Test Cases**:
1. **Test Results Preservation**: Compare test pass/fail status between original and fixed config for all 303 tests (should be identical when both complete)
2. **Coverage Preservation**: Verify coverage reports show same percentages for branches, functions, lines, statements (should meet 80% threshold in both)
3. **Property-Based Test Behavior**: Verify fast-check generates same test cases and produces same results (should be deterministic with same seed)
4. **Test Environment Preservation**: Verify jsdom, module resolution, and setup files work identically (should have no observable differences)

### Unit Tests

- Test that Jest configuration is valid and loads without errors
- Test that workerIdleMemoryLimit is properly set in the configuration
- Test that maxWorkers is set to the expected fixed value
- Test that all existing configuration options remain unchanged

### Property-Based Tests

- Generate random test execution scenarios and verify no memory crashes occur with fixed config
- Generate random test subsets and verify results are identical between original and fixed config
- Test that worker restart behavior works correctly across many test execution patterns

### Integration Tests

- Run full test suite with fixed configuration and verify completion without crashes
- Run tests multiple times in succession to verify worker lifecycle management
- Monitor system memory usage during test execution to verify it stays within bounds
- Verify CI/CD pipeline completes successfully with new configuration
