# Implementation Plan

- [x] 1. Write bug condition exploration test
  - **Property 1: Fault Condition** - Full Test Suite Memory Crash
  - **CRITICAL**: This test MUST FAIL on unfixed code - failure confirms the bug exists
  - **DO NOT attempt to fix the test or the code when it fails**
  - **NOTE**: This test encodes the expected behavior - it will validate the fix when it passes after implementation
  - **GOAL**: Surface counterexamples that demonstrate the bug exists
  - **Scoped PBT Approach**: Scope the property to the concrete failing case - full test suite execution (303 tests, 39 suites)
  - Test that running `npm test` with current jest.config.js completes without "Jest worker ran out of memory" errors
  - The test assertions should verify: allTestsCompleted = true, memoryErrors = 0, failedSuites = 0 (due to memory)
  - Run test on UNFIXED code (current jest.config.js without workerIdleMemoryLimit)
  - **EXPECTED OUTCOME**: Test FAILS with "Jest worker ran out of memory and crashed" after ~20 minutes (this is correct - it proves the bug exists)
  - Document counterexamples found: number of failed suites (expected: 29), failed tests (expected: 84), error messages
  - Mark task complete when test is written, run, and failure is documented
  - _Requirements: 1.1, 1.2, 1.3, 1.4_

- [x] 2. Write preservation property tests (BEFORE implementing fix)
  - **Property 2: Preservation** - Test Behavior and Results Unchanged
  - **IMPORTANT**: Follow observation-first methodology
  - Observe behavior on UNFIXED code for non-memory-management aspects:
    - Run individual test files or small test subsets (e.g., 10 suites) that complete successfully
    - Record test pass/fail status for these subsets
    - Record coverage percentages (should meet 80% threshold)
    - Record property-based test behavior (fast-check generators and results)
  - Write property-based tests capturing observed behavior patterns:
    - For all test subsets that complete without memory crashes, verify test results are identical
    - For all coverage measurements, verify percentages remain at or above 80%
    - For all property-based tests, verify fast-check generates same test cases with same seed
    - For all test environment aspects (jsdom, module resolution), verify no observable differences
  - Property-based testing generates many test cases for stronger guarantees
  - Run tests on UNFIXED code (on test subsets that don't trigger memory crashes)
  - **EXPECTED OUTCOME**: Tests PASS (this confirms baseline behavior to preserve)
  - Mark task complete when tests are written, run, and passing on unfixed code
  - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [-] 3. Fix for Jest memory crash during full test suite execution

  - [x] 3.1 Implement the memory management configuration changes
    - Open `Cerberus/dragon-ui/jest.config.js`
    - Add `workerIdleMemoryLimit: '512MB'` to the `customJestConfig` object to restart workers before memory exhaustion
    - Change `maxWorkers: '50%'` to `maxWorkers: 2` to limit total concurrent memory footprint
    - Add `maxConcurrency: 5` to limit concurrent tests within each worker (optional but recommended)
    - Add comments explaining the memory management settings for future maintainers
    - Consider adding Node.js flags via package.json if needed: `NODE_OPTIONS='--expose-gc --max-old-space-size=4096'`
    - _Bug_Condition: isBugCondition(input) where input.testSuiteSize >= 303 AND input.executionDuration >= 20 minutes AND input.config.workerIdleMemoryLimit IS NULL_
    - _Expected_Behavior: For all test executions where isBugCondition holds, result.allTestsCompleted = true AND result.memoryErrors = 0_
    - _Preservation: All test logic, assertions, coverage thresholds (80%), test results, and environment setup must remain unchanged_
    - _Requirements: 1.1, 1.2, 1.3, 1.4, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_

  - [-] 3.2 Verify bug condition exploration test now passes
    - **Property 1: Expected Behavior** - Full Test Suite Completes Without Memory Crashes
    - **IMPORTANT**: Re-run the SAME test from task 1 - do NOT write a new test
    - The test from task 1 encodes the expected behavior
    - When this test passes, it confirms the expected behavior is satisfied
    - Run bug condition exploration test from step 1 with the FIXED jest.config.js
    - **EXPECTED OUTCOME**: Test PASSES - all 39 test suites complete without "Jest worker ran out of memory" errors
    - Verify: allTestsCompleted = true, memoryErrors = 0, failedSuites = 0 (due to memory)
    - Monitor worker memory usage to confirm it stays below workerIdleMemoryLimit (512MB)
    - _Requirements: 2.1, 2.2, 2.3, 2.4_

  - [ ] 3.3 Verify preservation tests still pass
    - **Property 2: Preservation** - Test Behavior and Results Unchanged
    - **IMPORTANT**: Re-run the SAME tests from task 2 - do NOT write new tests
    - Run preservation property tests from step 2 with the FIXED jest.config.js
    - **EXPECTED OUTCOME**: Tests PASS (confirms no regressions)
    - Verify test results are identical for all test subsets
    - Verify coverage percentages remain at or above 80% for branches, functions, lines, statements
    - Verify property-based tests produce same results with same seed
    - Verify test environment (jsdom, module resolution, setup files) works identically
    - Confirm all tests still pass after fix (no regressions)
    - _Requirements: 3.1, 3.2, 3.3, 3.4_

- [ ] 4. Checkpoint - Ensure all tests pass
  - Run the full test suite with the fixed configuration: `npm test`
  - Verify all 39 test suites complete successfully without memory crashes
  - Verify test execution time is acceptable (may be slightly longer due to reduced workers)
  - Verify no "Jest worker ran out of memory" errors in output
  - If any issues arise, ask the user for guidance before proceeding
