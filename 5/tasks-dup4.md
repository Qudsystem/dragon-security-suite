# Implementation Plan - Scripts Organization

- [ ] 1. Set up project structure and core data models
  - Create directory structure for the organization tool
  - Implement ScriptFile, ScriptCategory, FileOperation, and other data models
  - Set up pytest and Hypothesis testing frameworks
  - _Requirements: 1.1, 1.2, 1.3, 4.1, 4.2, 4.3, 4.4_

- [ ] 1.1 Write property test for data model validation
  - **Property 11: Backup metadata completeness**
  - **Validates: Requirements 8.2**

- [ ] 2. Implement FileScanner component
  - Create FileScanner class with scan_directory method
  - Implement get_file_info to extract metadata (path, size, hash, modified date)
  - Implement compare_files for content comparison using hash
  - _Requirements: 1.2, 8.2_

- [ ] 2.1 Write unit tests for FileScanner
  - Test scanning directories with .bat and .ps1 files
  - Test file metadata extraction
  - Test file comparison with identical and different files
  - _Requirements: 1.2, 8.2_

- [ ] 2.2 Write property test for content verification
  - **Property 2: Content verification before deletion**
  - **Validates: Requirements 1.2**

- [ ] 3. Implement DuplicateDetector component
  - Create DuplicateDetector class
  - Implement find_duplicates to group duplicate files by hash
  - Implement recommend_keeper with location priority logic (prefer Cerberus/scripts/)
  - _Requirements: 1.1, 1.2, 1.3_

- [ ] 3.1 Write unit tests for DuplicateDetector
  - Test duplicate detection with known duplicate files
  - Test keeper recommendation logic
  - Test handling of non-duplicate files
  - _Requirements: 1.1, 1.2, 1.3_

- [ ] 3.2 Write property test for duplicate removal
  - **Property 1: Duplicate removal preserves unique scripts**
  - **Validates: Requirements 1.1, 1.3**

- [ ] 4. Implement script categorization logic
  - Create categorize_script function based on filename and location patterns
  - Map categories to target directories
  - Handle special cases (entry points, gateway scripts)
  - _Requirements: 4.1, 4.2, 4.3, 4.4, 3.1, 3.2_

- [ ] 4.1 Write unit tests for categorization
  - Test categorization of testing scripts
  - Test categorization of maintenance scripts
  - Test categorization of startup scripts
  - Test categorization of gateway scripts
  - Test categorization of entry point scripts
  - _Requirements: 4.1, 4.2, 4.3, 4.4_

- [ ] 4.2 Write property test for categorization mapping
  - **Property 3: Script categorization determines location**
  - **Validates: Requirements 4.1, 4.2, 4.3, 4.4**

- [ ] 4.3 Write property test for entry point preservation
  - **Property 12: Entry point preservation**
  - **Validates: Requirements 3.3, 3.4**

- [ ] 5. Implement ReferenceChecker component
  - Create ReferenceChecker class
  - Implement find_references to parse scripts for references (CALL, CD, path references)
  - Implement validate_references to check if referenced files exist
  - Implement update_reference to modify script content with new paths
  - _Requirements: 6.1, 6.2, 6.3, 2.4_

- [ ] 5.1 Write unit tests for ReferenceChecker
  - Test reference finding in BAT files (CALL, CD commands)
  - Test reference finding in PS1 files (dot sourcing, paths)
  - Test reference validation
  - Test reference updating
  - _Requirements: 6.1, 6.2, 6.3_

- [ ] 5.2 Write property test for reference discovery
  - **Property 7: Reference discovery completeness**
  - **Validates: Requirements 6.1**

- [ ] 5.3 Write property test for reference updates
  - **Property 5: Reference update completeness**
  - **Validates: Requirements 2.4, 6.2**

- [ ] 5.4 Write property test for no broken references
  - **Property 6: No broken references after organization**
  - **Validates: Requirements 6.3**

- [ ] 6. Implement FileOperations component
  - Create FileOperations class
  - Implement move_file with error handling and validation
  - Implement delete_file with confirmation and logging
  - Implement create_directory for new subdirectories
  - Implement merge_files for consolidating gateway scripts
  - _Requirements: 1.1, 2.1, 2.2, 4.1, 4.2, 4.3, 4.4_

- [ ] 6.1 Write unit tests for FileOperations
  - Test move operations with mocked file system
  - Test delete operations with mocked file system
  - Test directory creation
  - Test merge operations
  - Test error handling (permission denied, file not found)
  - _Requirements: 1.1, 2.1, 2.2_

- [ ] 7. Implement Validator component
  - Create Validator class
  - Implement validate_structure to check target directory structure
  - Implement verify_no_duplicates to ensure cleanup was successful
  - Implement check_all_references_valid for final verification
  - _Requirements: 1.3, 1.4, 6.3, 4.5_

- [ ] 7.1 Write unit tests for Validator
  - Test structure validation with valid and invalid structures
  - Test duplicate verification
  - Test reference validation
  - _Requirements: 1.3, 6.3, 4.5_

- [ ] 7.2 Write property test for directory depth
  - **Property 4: Directory depth constraint**
  - **Validates: Requirements 4.5**

- [ ] 8. Implement backup and documentation system
  - Create BackupManager class
  - Implement create_backup_documentation to record current state
  - Record file paths, sizes, modification dates, and hashes
  - Store backup documentation in SCRIPTS-ANALYSIS.md format
  - _Requirements: 8.1, 8.2, 8.3_

- [ ] 8.1 Write unit tests for BackupManager
  - Test backup documentation creation
  - Test metadata recording
  - Test documentation storage
  - _Requirements: 8.1, 8.2, 8.3_

- [ ] 9. Implement documentation generator
  - Create DocumentationGenerator class
  - Implement generate_organization_doc to create SCRIPTS-ORGANIZATION.md
  - Include directory structure diagram
  - List all remaining scripts with purposes
  - Document deleted scripts with reasons
  - Include usage examples for key scripts
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 9.1 Write unit tests for DocumentationGenerator
  - Test documentation generation with sample data
  - Test directory structure diagram creation
  - Test script listing
  - Test deletion documentation
  - Test usage example inclusion
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 9.2 Write property test for documentation completeness
  - **Property 8: Documentation completeness**
  - **Validates: Requirements 5.2**

- [ ] 9.3 Write property test for deletion documentation
  - **Property 9: Deletion documentation**
  - **Validates: Requirements 5.4**

- [ ] 9.4 Write property test for usage examples
  - **Property 10: Usage examples for key scripts**
  - **Validates: Requirements 5.5**

- [ ] 10. Implement OrganizationPlanner
  - Create OrganizationPlanner class to coordinate all components
  - Implement create_plan to analyze current structure and generate operations
  - Identify duplicates using DuplicateDetector
  - Categorize scripts and determine target locations
  - Identify files to merge (gateway scripts)
  - Generate list of operations (move, delete, merge, create_dir)
  - _Requirements: 1.1, 2.1, 4.1, 4.2, 4.3, 4.4_

- [ ] 10.1 Write unit tests for OrganizationPlanner
  - Test plan creation with sample directory structure
  - Test duplicate identification
  - Test categorization
  - Test operation generation
  - _Requirements: 1.1, 2.1, 4.1, 4.2, 4.3, 4.4_

- [ ] 11. Implement OrganizationExecutor
  - Create OrganizationExecutor class to execute the plan
  - Implement execute_plan to perform operations in correct order
  - Create directories first
  - Move files second
  - Merge files third
  - Delete duplicates last
  - Update references after moves
  - Log all operations
  - _Requirements: 1.1, 2.1, 2.4, 4.1, 4.2, 4.3, 4.4, 6.2_

- [ ] 11.1 Write unit tests for OrganizationExecutor
  - Test execution order
  - Test operation logging
  - Test error handling during execution
  - _Requirements: 1.1, 2.1, 2.4_

- [ ] 12. Checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.

- [ ] 13. Implement gateway script merger
  - Create merge_gateway_scripts function
  - Read run_server.bat and START-SERVER.bat
  - Combine functionality into unified start-server.bat
  - Preserve all commands and error handling from both
  - Add header comment explaining the merge
  - _Requirements: 2.1, 2.2, 2.3_

- [ ] 13.1 Write unit test for gateway merger
  - Test merging with sample gateway scripts
  - Verify all functionality is preserved
  - Verify output naming
  - _Requirements: 2.1, 2.2, 2.3_

- [ ] 14. Implement rollback mechanism
  - Create RollbackManager class
  - Implement rollback_operation to reverse a single operation
  - Implement rollback_all to restore original structure
  - Use backup documentation to guide rollback
  - _Requirements: 8.4_

- [ ] 14.1 Write unit tests for RollbackManager
  - Test single operation rollback
  - Test full rollback
  - Test rollback with missing backup data
  - _Requirements: 8.4_

- [ ] 15. Create main CLI interface
  - Create main.py with command-line interface
  - Add commands: analyze, plan, execute, rollback, verify
  - Add --dry-run flag for safe testing
  - Add --verbose flag for detailed logging
  - Implement interactive confirmation before destructive operations
  - _Requirements: All_

- [ ] 15.1 Write integration tests for CLI
  - Test analyze command
  - Test plan command
  - Test execute command with dry-run
  - Test rollback command
  - Test verify command
  - _Requirements: All_

- [ ] 16. Execute organization on actual Dragon-R1 project
  - Run analyze command to scan current structure
  - Review generated plan
  - Create backup documentation
  - Execute organization with confirmation
  - Verify results
  - _Requirements: All_

- [ ] 17. Verify organization results
  - Run validator to check for duplicates
  - Run validator to check for broken references
  - Verify directory structure matches target
  - Verify file count reduction (should be ~16 files)
  - Test entry point scripts manually
  - _Requirements: 1.3, 1.4, 6.3, 4.5_

- [ ] 18. Generate final documentation
  - Run documentation generator
  - Review SCRIPTS-ORGANIZATION.md for completeness
  - Ensure all scripts are documented
  - Ensure usage examples are clear
  - Ensure deletion reasons are documented
  - _Requirements: 5.1, 5.2, 5.3, 5.4, 5.5_

- [ ] 19. Final checkpoint - Ensure all tests pass
  - Ensure all tests pass, ask the user if questions arise.
