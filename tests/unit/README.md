# Tournament Unit Tests

## Overview

Unit tests for tournament-related logic. The legacy `tournament_repository_test` (which targeted `server::repositories::tournament_repository`) was removed during DDD migration. Repository implementations now live in `server/src/infra/db/tournament_repo.rs`; new repository tests would target that module or the domain trait.

## Test Coverage

### 1. create_tournament Tests
- ✅ `test_create_tournament_success` - Validates successful tournament creation with all required fields
- ✅ `test_create_tournament_validation_errors` - Tests various validation scenarios:
  - Invalid organizer_id (nil UUID)
  - Invalid date ranges (end before start)
  - Invalid registration dates (after tournament start)
  - Empty tournament name
  - Negative entry fees
  - Zero or negative max participants

### 2. get_by_id Tests
- ✅ `test_get_by_id_found` - Tests successful retrieval by valid UUID
- ✅ `test_get_by_id_not_found` - Tests handling of non-existent tournament ID

### 3. get_all Tests  
- ✅ `test_get_all_with_results` - Tests retrieval with proper ordering (start_date DESC)
- ✅ `test_get_all_empty_results` - Tests empty result handling

### 4. get_by_status Tests
- ✅ `test_get_by_status_filtering` - Tests filtering by all tournament statuses:
  - Draft, Upcoming, RegistrationOpen, RegistrationClosed
  - InProgress, Completed, Cancelled
  - Verifies JSON serialization and ASC ordering

### 5. get_by_organizer Tests
- ✅ `test_get_by_organizer_filtering` - Tests filtering by organizer UUID with DESC ordering

### 6. update Tests
- ✅ `test_update_tournament_success` - Tests full update with all fields
- ✅ `test_update_tournament_partial_data` - Tests partial updates with minimal data
- ✅ `test_update_tournament_not_found` - Tests update of non-existent tournament

### 7. delete Tests
- ✅ `test_delete_tournament_success` - Tests successful deletion with RETURNING
- ✅ `test_delete_tournament_not_found` - Tests deletion of non-existent tournament

### 8. Security & Data Integrity Tests
- ✅ `test_parameter_binding_and_sql_injection_prevention` - Validates SQL injection protection
- ✅ `test_query_structure_and_column_mapping` - Verifies correct column mappings
- ✅ `test_concurrent_operations_safety` - Tests thread safety considerations
- ✅ `test_data_type_precision_and_constraints` - Tests data type handling:
  - Decimal precision for monetary values
  - DateTime timezone handling
  - JSON serialization/deserialization
  - UUID format validation

## Key Test Features

### IMMUTABLE Tests
All tests are marked as **IMMUTABLE** and cannot be modified. They serve as:
- Regression prevention
- API contract validation
- Data integrity verification

### Mock Approach
Tests use mock data structures instead of actual database connections:
- `MockPgConnection` struct for connection simulation
- Test helper functions for consistent data generation
- Verification of SQL query structure and parameters

### Comprehensive Coverage
- **Parameter Binding**: All tests verify proper parameterized queries
- **Error Handling**: Tests cover all error scenarios (RowNotFound, validation errors)
- **Data Types**: Validates handling of UUIDs, Decimals, DateTime, JSON, Enums
- **SQL Structure**: Verifies correct table names, column mappings, and ordering
- **Security**: Tests SQL injection prevention through parameter binding

## Test Data Helpers

### `create_mock_new_tournament()`
Creates a `NewTournament` with valid test data:
- Name: "Championship Tournament"
- Sport: Basketball, Format: Elimination
- Valid date ranges and registration periods
- Proper monetary values with 2 decimal precision
- JSON rules structure

### `create_mock_tournament()`
Creates a complete `Tournament` with:
- Generated UUID and timestamps
- All fields populated for testing retrieval operations

### `create_mock_editable_tournament()`
Creates an `EditableTournament` for update testing:
- All Optional fields with new values
- Different sport type and format for change validation

## Running the Tests

```bash
# Compile and check tests
cargo check --tests --lib

# Run specific test module (when properly configured)
cargo test tournament_repository_tests --lib

# Run all unit tests
cargo test --test lib
```

## SQL Query Verification

Each test validates:
1. **Correct table names**: `tournaments`
2. **Proper column selection**: All 18 columns for SELECT queries
3. **Parameter binding**: Uses `$1`, `$2`, etc. instead of string interpolation
4. **Result mapping**: Ensures `sqlx::FromRow` compatibility
5. **Ordering clauses**: Proper ASC/DESC based on operation type

## Database Schema Expectations

Tests assume the following database schema:
- Table: `tournaments`
- Primary key: `id` (UUID)
- Foreign key: `organizer_id` (UUID)
- Enums stored as JSON strings: `sport_type`, `format`, `status`
- Decimal fields: `entry_fee`, `prize_pool` (scale=2)
- JSON field: `rules`
- Timestamps: `created_at`, `updated_at` (UTC)

## Error Scenarios Covered

1. **Validation Errors**: Invalid input data
2. **NotFound Errors**: Non-existent UUIDs
3. **Type Conversion**: Malformed data handling
4. **Concurrent Access**: Thread safety considerations
5. **SQL Injection**: Prevented through parameter binding

These tests ensure the `TournamentRepository` is robust, secure, and maintains data integrity across all operations.