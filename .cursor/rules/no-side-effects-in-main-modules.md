# No Side Effects in Main Modules

## Purpose
Ensure core logic is pure and testable by separating side effects from business logic.

## Rule
Main modules must be pure functions with no side effects. Side effects must be isolated in dedicated modules.

## Architecture Pattern
```
Core Logic (Pure) → Side Effect Handlers → External Systems
```

## Pure Functions
- No file system access
- No network calls
- No database operations
- No logging (use dependency injection)
- No time-dependent operations (inject time)

## Side Effect Isolation
- **File Operations**: Isolated in `storage/` module
- **Network Calls**: Isolated in `protocols/` module
- **System Calls**: Isolated in `system/` module
- **Logging**: Isolated in `logging/` module

## Dependency Injection
- Inject dependencies rather than importing directly
- Use traits/interfaces for external dependencies
- Mock dependencies in tests
- Pass configuration as parameters

## Example Structure
```rust
// Pure function (core logic)
pub fn process_activity(data: ActivityData) -> ProcessedActivity {
    // Pure business logic only
}

// Side effect handler (separate module)
pub async fn save_activity(activity: ProcessedActivity) -> Result<(), Error> {
    // File system operations here
}
```

## Testing Benefits
- Pure functions are easy to test
- No need to mock file system or network
- Tests are fast and reliable
- Can test edge cases easily

## Enforcement
- Static analysis to detect side effects
- Code review focuses on separation
- Tests verify pure function behavior
- Documentation clearly separates concerns 