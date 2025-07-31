# All Modules Must Include Tests

## Purpose
Ensure code quality and reliability through comprehensive testing coverage.

## Rule
All modules must include tests with ≥80% coverage as specified in `rae_engineering_standards.md`.

## Test Requirements

### Unit Tests
- Test all public functions and methods
- Test error conditions and edge cases
- Test schema validation functions
- Mock external dependencies

### Integration Tests
- Test module interactions
- Test API endpoints
- Test protocol compliance (A2A/MCP)
- Test CLI functionality

### Schema Tests
- Validate all data structures against schemas
- Test schema evolution and backward compatibility
- Test schema documentation accuracy

### Security Tests
- Test module sandboxing
- Test privilege isolation
- Test data privacy protection
- Test protocol security mechanisms

## Test Structure
```
src/agent/tests/
├── unit/
│   ├── core/
│   ├── modules/
│   └── schemas/
├── integration/
│   ├── api/
│   ├── protocols/
│   └── cli/
└── security/
    ├── sandboxing/
    └── privacy/
```

## Coverage Enforcement
- CI/CD pipeline enforces ≥80% coverage
- Coverage reports generated for all test runs
- Failing coverage blocks merge to main
- Coverage metrics tracked over time

## Test Quality
- Tests must be meaningful and not just for coverage
- Tests should be fast and reliable
- Use appropriate mocking and test data
- Test both success and failure scenarios 