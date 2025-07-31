# Strict Schema Enforcement

## Purpose
Ensure data consistency and type safety across all modules and interfaces.

## Rule
All data structures must be defined by schemas in `/schemas/` and validated at runtime.

## Schema Requirements
- All schemas stored in `/schemas/` directory
- Schemas must be versioned and backward compatible
- All schemas must be validated at runtime
- Schema documentation must be accurate and complete

## Schema Categories
```
/schemas/
├── core/           # Core system schemas
├── modules/        # Module-specific schemas
├── protocols/      # A2A/MCP protocol schemas
├── security/       # Security and validation schemas
├── testing/        # Test data schemas
└── ui/            # UI component schemas
```

## Validation Points
- **Module Inputs**: All module inputs validated against schemas
- **Module Outputs**: All module outputs validated before storage
- **API Requests**: All API requests validated against schemas
- **API Responses**: All API responses validated before sending
- **Event Messages**: All event bus messages validated
- **Storage Data**: All stored data validated on read/write

## Schema Evolution
- New schemas must be backward compatible
- Breaking changes require major version bump
- Migration scripts for schema updates
- Deprecation warnings for old schemas

## Runtime Validation
```rust
// Example validation
let data: ActivityData = serde_json::from_str(&json)?;
validate_against_schema(&data, "activity.json")?;
```

## Error Handling
- Validation errors must be descriptive
- Failed validation prevents data processing
- Validation errors logged for debugging
- User-friendly error messages

## Testing
- Unit tests for all schema validation
- Integration tests for schema compliance
- Tests for schema evolution scenarios
- Tests for validation error handling

## Documentation
- All schemas must be self-documenting
- Include examples in schema comments
- Document validation rules and constraints
- Maintain schema change log 