# Design Document - Scripts Organization

## Overview

This design document outlines the technical approach for reorganizing the Dragon-R1 project's script files. The system will eliminate duplicate scripts, consolidate similar functionality, and create a clear hierarchical structure that improves maintainability and developer experience.

The reorganization will be performed through a series of file system operations (move, delete, merge) with careful validation at each step to ensure no functionality is lost and all references remain valid.

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Script Organization System                │
├─────────────────────────────────────────────────────────────┤
│                                                               │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │   Analysis   │→ │ Validation   │→ │ Execution    │      │
│  │   Module     │  │   Module     │  │   Module     │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│         ↓                  ↓                  ↓              │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐      │
│  │ File Scanner │  │ Reference    │  │ File System  │      │
│  │              │  │ Checker      │  │ Operations   │      │
│  └──────────────┘  └──────────────┘  └──────────────┘      │
│                                                               │
└─────────────────────────────────────────────────────────────┘
```

### Component Interaction Flow

```
1. Scan → 2. Analyze → 3. Validate → 4. Execute → 5. Verify
   ↓         ↓           ↓            ↓           ↓
 Files    Duplicates  References   Operations  Results
```

## Components and Interfaces

### 1. File Scanner Component

**Purpose:** Discover and catalog all script files in the project

**Interface:**
```python
class FileScanner:
    def scan_directory(path: str, extensions: List[str]) -> List[ScriptFile]
    def get_file_info(path: str) -> FileInfo
    def compare_files(file1: str, file2: str) -> bool
```

**Responsibilities:**
- Recursively scan directories for .bat and .ps1 files
- Extract file metadata (size, modification date, hash)
- Compare file contents to identify duplicates


### 2. Duplicate Detector Component

**Purpose:** Identify duplicate scripts across directories

**Interface:**
```python
class DuplicateDetector:
    def find_duplicates(files: List[ScriptFile]) -> Dict[str, List[str]]
    def calculate_similarity(file1: ScriptFile, file2: ScriptFile) -> float
    def recommend_keeper(duplicates: List[str]) -> str
```

**Responsibilities:**
- Compare file contents using hash comparison
- Group duplicate files together
- Recommend which copy to keep based on location priority

### 3. Reference Checker Component

**Purpose:** Find and validate references between scripts

**Interface:**
```python
class ReferenceChecker:
    def find_references(script: str, target: str) -> List[Reference]
    def validate_references(files: List[ScriptFile]) -> List[BrokenReference]
    def update_reference(file: str, old_path: str, new_path: str) -> bool
```

**Responsibilities:**
- Parse script files for references to other scripts
- Identify broken references after file operations
- Update references to point to new locations

### 4. File Operations Component

**Purpose:** Execute file system operations safely

**Interface:**
```python
class FileOperations:
    def move_file(source: str, destination: str) -> OperationResult
    def delete_file(path: str) -> OperationResult
    def merge_files(files: List[str], output: str) -> OperationResult
    def create_directory(path: str) -> OperationResult
```

**Responsibilities:**
- Perform file system operations with error handling
- Validate operations before execution
- Rollback on failure

### 5. Validation Component

**Purpose:** Ensure operations maintain system integrity

**Interface:**
```python
class Validator:
    def validate_structure(target_structure: Dict) -> ValidationResult
    def verify_no_duplicates(directory: str) -> bool
    def check_all_references_valid(files: List[str]) -> List[str]
```

**Responsibilities:**
- Validate target directory structure
- Verify no duplicates remain
- Check all script references are valid

## Data Models

### ScriptFile
```python
@dataclass
class ScriptFile:
    path: str
    name: str
    extension: str  # .bat or .ps1
    size: int
    modified_date: datetime
    content_hash: str
    category: ScriptCategory  # startup, testing, maintenance, gateway
```

### ScriptCategory
```python
class ScriptCategory(Enum):
    ENTRY_POINT = "entry_point"
    STARTUP = "startup"
    TESTING = "testing"
    MAINTENANCE = "maintenance"
    GATEWAY = "gateway"
    TEMPLATE = "template"
```

### FileOperation
```python
@dataclass
class FileOperation:
    operation_type: OperationType  # MOVE, DELETE, MERGE
    source_path: str
    destination_path: Optional[str]
    reason: str
    status: OperationStatus
```

### OperationType
```python
class OperationType(Enum):
    MOVE = "move"
    DELETE = "delete"
    MERGE = "merge"
    CREATE_DIR = "create_directory"
```

### Reference
```python
@dataclass
class Reference:
    source_file: str
    target_file: str
    line_number: int
    reference_type: ReferenceType  # CALL, CD, PATH
```

### OrganizationPlan
```python
@dataclass
class OrganizationPlan:
    operations: List[FileOperation]
    new_structure: Dict[str, List[str]]
    files_to_delete: List[str]
    files_to_move: Dict[str, str]
    files_to_merge: List[Tuple[List[str], str]]
```


## Correctness Properties

*A property is a characteristic or behavior that should hold true across all valid executions of a system—essentially, a formal statement about what the system should do. Properties serve as the bridge between human-readable specifications and machine-verifiable correctness guarantees.*

### Property 1: Duplicate removal preserves unique scripts
*For any* set of duplicate scripts, after removal operations, exactly one copy should remain in the designated location (Cerberus/scripts/)
**Validates: Requirements 1.1, 1.3**

### Property 2: Content verification before deletion
*For any* pair of files marked as duplicates, their content hashes must match before deletion is performed
**Validates: Requirements 1.2**

### Property 3: Script categorization determines location
*For any* script file with a given category (testing, maintenance, startup, gateway), its final location must match the designated directory for that category
**Validates: Requirements 4.1, 4.2, 4.3, 4.4**

### Property 4: Directory depth constraint
*For any* script file in the organized structure, the directory depth from project root must not exceed 3 levels
**Validates: Requirements 4.5**

### Property 5: Reference update completeness
*For any* script that is moved or renamed, all references to that script in other files must be updated to the new path
**Validates: Requirements 2.4, 6.2**

### Property 6: No broken references after organization
*For any* script file in the final structure, all references it contains to other scripts must point to existing files
**Validates: Requirements 6.3**

### Property 7: Reference discovery completeness
*For any* script file that is moved or deleted, the system must identify all files that reference it before the operation
**Validates: Requirements 6.1**

### Property 8: Documentation completeness
*For any* script file in the final structure, the documentation must contain an entry describing its purpose and location
**Validates: Requirements 5.2**

### Property 9: Deletion documentation
*For any* script file that is deleted, the documentation must contain an entry explaining why it was removed
**Validates: Requirements 5.4**

### Property 10: Usage examples for key scripts
*For any* script categorized as "entry_point" or "startup", the documentation must include usage examples
**Validates: Requirements 5.5**

### Property 11: Backup metadata completeness
*For any* script file in the original structure, the backup documentation must record its path, size, and modification date
**Validates: Requirements 8.2**

### Property 12: Entry point preservation
*For any* script categorized as "entry_point", after organization it must remain in its original location
**Validates: Requirements 3.3, 3.4**


## Error Handling

### File Operation Errors

**Error Type:** File not found during move/delete
- **Detection:** Check file existence before operation
- **Recovery:** Log error, skip operation, continue with remaining operations
- **User Notification:** Report missing file in error log

**Error Type:** Permission denied
- **Detection:** Catch OS-level permission errors
- **Recovery:** Request elevated permissions or skip operation
- **User Notification:** Clear message about permission requirements

**Error Type:** File in use
- **Detection:** Catch file lock errors
- **Recovery:** Retry after delay, or skip and report
- **User Notification:** List files that couldn't be modified

### Validation Errors

**Error Type:** Duplicate files with different content
- **Detection:** Hash comparison shows mismatch
- **Recovery:** Mark as non-duplicate, keep both files
- **User Notification:** Report files that appear duplicate but have different content

**Error Type:** Broken references detected
- **Detection:** Reference checker finds non-existent target
- **Recovery:** Log broken reference, optionally attempt to fix
- **User Notification:** List all broken references with source and target

**Error Type:** Invalid directory structure
- **Detection:** Target structure validation fails
- **Recovery:** Abort operation before making changes
- **User Notification:** Explain what's wrong with the structure

### Rollback Strategy

If critical errors occur during execution:
1. Stop all pending operations
2. Reverse completed operations in reverse order
3. Restore files from backup documentation
4. Generate rollback report

## Testing Strategy

### Unit Testing

Unit tests will verify individual components work correctly:

**FileScanner Tests:**
- Test scanning directories with various file types
- Test file comparison with identical and different files
- Test metadata extraction

**DuplicateDetector Tests:**
- Test duplicate detection with known duplicate files
- Test similarity calculation
- Test keeper recommendation logic

**ReferenceChecker Tests:**
- Test reference finding in various script syntaxes
- Test reference validation
- Test reference updating

**FileOperations Tests:**
- Test move operations (with mocks)
- Test delete operations (with mocks)
- Test merge operations
- Test directory creation

### Property-Based Testing

Property-based tests will verify correctness properties hold across many inputs. We will use **Hypothesis** (Python) for property-based testing, configured to run a minimum of 100 iterations per test.

Each property-based test will be tagged with a comment explicitly referencing the correctness property from this design document using the format: **Feature: scripts-organization, Property {number}: {property_text}**

**Test 1: Duplicate Removal Invariant**
- Generate random sets of duplicate files
- Execute removal operation
- Verify exactly one copy remains
- **Feature: scripts-organization, Property 1: Duplicate removal preserves unique scripts**

**Test 2: Content Verification**
- Generate pairs of files with varying similarity
- Attempt duplicate detection
- Verify only truly identical files are marked as duplicates
- **Feature: scripts-organization, Property 2: Content verification before deletion**

**Test 3: Categorization Mapping**
- Generate scripts with random categories
- Execute organization
- Verify each script is in the correct directory for its category
- **Feature: scripts-organization, Property 3: Script categorization determines location**

**Test 4: Directory Depth**
- Generate various directory structures
- Verify no script exceeds 3 levels of depth
- **Feature: scripts-organization, Property 4: Directory depth constraint**

**Test 5: Reference Updates**
- Generate scripts with references to other scripts
- Move referenced scripts
- Verify all references are updated
- **Feature: scripts-organization, Property 5: Reference update completeness**

**Test 6: No Broken References**
- Generate random script structures with references
- Execute organization
- Verify no broken references exist
- **Feature: scripts-organization, Property 6: No broken references after organization**

**Test 7: Reference Discovery**
- Generate scripts with various reference patterns
- Test reference discovery finds all references
- **Feature: scripts-organization, Property 7: Reference discovery completeness**

**Test 8: Documentation Completeness**
- Generate random script sets
- Create documentation
- Verify all scripts are documented
- **Feature: scripts-organization, Property 8: Documentation completeness**

**Test 9: Deletion Documentation**
- Generate random deletion operations
- Create documentation
- Verify all deletions are documented with reasons
- **Feature: scripts-organization, Property 9: Deletion documentation**

**Test 10: Usage Examples**
- Generate scripts with various categories
- Create documentation
- Verify entry_point and startup scripts have usage examples
- **Feature: scripts-organization, Property 10: Usage examples for key scripts**

**Test 11: Backup Metadata**
- Generate random script files
- Create backup documentation
- Verify all metadata fields are present
- **Feature: scripts-organization, Property 11: Backup metadata completeness**

**Test 12: Entry Point Preservation**
- Generate organization plans with entry point scripts
- Execute organization
- Verify entry points remain in original locations
- **Feature: scripts-organization, Property 12: Entry point preservation**

### Integration Testing

Integration tests will verify the complete workflow:

1. **End-to-End Organization Test**
   - Start with a known directory structure
   - Execute complete organization
   - Verify final structure matches expected
   - Verify all scripts still work

2. **Rollback Test**
   - Execute organization
   - Trigger rollback
   - Verify original structure is restored

3. **Reference Update Test**
   - Create scripts with cross-references
   - Execute organization
   - Verify all references still work

### Manual Testing

Manual verification steps:
1. Run entry point scripts to verify they launch correctly
2. Spot-check moved scripts to verify they're in correct locations
3. Review documentation for completeness and clarity
4. Verify no duplicate files remain

## Implementation Notes

### Technology Stack
- **Language:** Python 3.8+
- **Testing Framework:** pytest
- **Property Testing:** Hypothesis
- **File Operations:** pathlib, shutil
- **Hashing:** hashlib

### Performance Considerations
- File scanning should be efficient for large directory trees
- Hash calculation should be cached to avoid redundant computation
- Operations should be batched where possible

### Security Considerations
- Validate all file paths to prevent directory traversal
- Require confirmation before deleting files
- Log all operations for audit trail
- Preserve file permissions during moves

### Extensibility
- Plugin system for custom categorization rules
- Configurable directory structure
- Support for additional script types beyond .bat and .ps1
