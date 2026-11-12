# Requirements Document - Scripts Organization

## Introduction

This document defines the requirements for organizing and consolidating the BAT and PS1 script files in the Dragon-R1 project. The project currently contains 25 script files with significant duplication across multiple directories. This reorganization will eliminate redundancy, improve maintainability, and create a clear, professional structure.

## Glossary

- **Script File**: A BAT (Batch) or PS1 (PowerShell) file used for automation
- **Dragon-R1 System**: The autonomous security platform being organized
- **Cerberus Module**: The main security module within Dragon-R1
- **Gateway Component**: The API gateway service within Cerberus
- **Duplicate Script**: A script file with identical or near-identical content in multiple locations
- **Entry Point Script**: A primary script used to launch the system

## Requirements

### Requirement 1: Eliminate Script Duplication

**User Story:** As a developer, I want to eliminate duplicate script files, so that I only need to maintain one version of each script.

#### Acceptance Criteria

1. WHEN duplicate scripts exist in both `Cerberus/` root and `Cerberus/scripts/` THEN the Dragon-R1 System SHALL remove the duplicates from `Cerberus/` root and retain only the versions in `Cerberus/scripts/`
2. WHEN a script file is identified as duplicate THEN the Dragon-R1 System SHALL verify content similarity before deletion
3. WHEN duplicate scripts are removed THEN the Dragon-R1 System SHALL maintain at least one working copy of each unique script
4. WHEN the cleanup is complete THEN the Dragon-R1 System SHALL reduce the total script count by at least 7 files

### Requirement 2: Consolidate Gateway Scripts

**User Story:** As a developer, I want to consolidate redundant gateway startup scripts, so that there is one clear way to start the gateway service.

#### Acceptance Criteria

1. WHEN multiple gateway startup scripts exist with similar functionality THEN the Dragon-R1 System SHALL merge them into a single unified script
2. WHEN merging `run_server.bat` and `START-SERVER.bat` THEN the Dragon-R1 System SHALL preserve all functionality from both scripts
3. WHEN the consolidated script is created THEN the Dragon-R1 System SHALL name it clearly (e.g., `start-server.bat`)
4. WHEN old gateway scripts are removed THEN the Dragon-R1 System SHALL update any references to point to the new unified script

### Requirement 3: Maintain Entry Point Scripts

**User Story:** As a user, I want clear entry point scripts in the root directory, so that I can easily start the Dragon-R1 system.

#### Acceptance Criteria

1. WHEN organizing scripts THEN the Dragon-R1 System SHALL preserve `Start-Dragon-When-Open.bat` in the root directory
2. WHEN organizing scripts THEN the Dragon-R1 System SHALL preserve `Start-Dragon-When-Open.ps1` in the root directory
3. WHEN entry point scripts are preserved THEN the Dragon-R1 System SHALL ensure they remain functional
4. WHEN the organization is complete THEN the Dragon-R1 System SHALL verify that entry points correctly reference other scripts

### Requirement 4: Organize Scripts by Function

**User Story:** As a developer, I want scripts organized by their function, so that I can quickly find the script I need.

#### Acceptance Criteria

1. WHEN organizing scripts THEN the Dragon-R1 System SHALL place all testing scripts in `Cerberus/scripts/`
2. WHEN organizing scripts THEN the Dragon-R1 System SHALL place all maintenance scripts in `Cerberus/scripts/`
3. WHEN organizing scripts THEN the Dragon-R1 System SHALL place all startup scripts in `Cerberus/` root
4. WHEN organizing scripts THEN the Dragon-R1 System SHALL place all gateway-specific scripts in `Cerberus/gateway/`
5. WHEN the organization is complete THEN the Dragon-R1 System SHALL have a maximum of 3 directory levels for scripts

### Requirement 5: Create Organization Documentation

**User Story:** As a developer, I want clear documentation of the script organization, so that I understand the purpose and location of each script.

#### Acceptance Criteria

1. WHEN the organization is complete THEN the Dragon-R1 System SHALL create a `SCRIPTS-ORGANIZATION.md` file
2. WHEN creating documentation THEN the Dragon-R1 System SHALL list all remaining scripts with their purposes
3. WHEN creating documentation THEN the Dragon-R1 System SHALL include a directory structure diagram
4. WHEN creating documentation THEN the Dragon-R1 System SHALL document which scripts were removed and why
5. WHEN creating documentation THEN the Dragon-R1 System SHALL provide usage examples for key scripts

### Requirement 6: Verify Script References

**User Story:** As a developer, I want to ensure no broken references exist after reorganization, so that all scripts continue to work correctly.

#### Acceptance Criteria

1. WHEN a script is moved or deleted THEN the Dragon-R1 System SHALL identify all files that reference it
2. WHEN references are found THEN the Dragon-R1 System SHALL update them to point to the new location
3. WHEN the organization is complete THEN the Dragon-R1 System SHALL verify that no broken script references exist
4. WHEN verification is complete THEN the Dragon-R1 System SHALL report any unresolved references

### Requirement 7: Preserve Script Functionality

**User Story:** As a user, I want all scripts to continue working after reorganization, so that I can use the system without interruption.

#### Acceptance Criteria

1. WHEN scripts are moved or consolidated THEN the Dragon-R1 System SHALL preserve all original functionality
2. WHEN testing reorganized scripts THEN the Dragon-R1 System SHALL verify that startup scripts launch services correctly
3. WHEN testing reorganized scripts THEN the Dragon-R1 System SHALL verify that test scripts execute tests correctly
4. WHEN testing reorganized scripts THEN the Dragon-R1 System SHALL verify that maintenance scripts perform their functions correctly

### Requirement 8: Create Backup Before Changes

**User Story:** As a developer, I want a backup of the original script structure, so that I can recover if something goes wrong.

#### Acceptance Criteria

1. WHEN starting the reorganization THEN the Dragon-R1 System SHALL document the current state of all scripts
2. WHEN documenting the current state THEN the Dragon-R1 System SHALL record file paths, sizes, and modification dates
3. WHEN the backup is complete THEN the Dragon-R1 System SHALL store the documentation in a safe location
4. WHEN changes are made THEN the Dragon-R1 System SHALL allow rollback to the original structure if needed
