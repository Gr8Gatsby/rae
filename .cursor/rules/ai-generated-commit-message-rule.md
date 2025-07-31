# AI-Generated Commit Message Rule

## Purpose
Ensure all commits have meaningful, consistent commit messages that follow conventional commit standards.

## Rule
All commit messages must follow the conventional commit format:

```
type(scope): description

[optional body]

[optional footer]
```

## Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

## Scopes
- `core`: Core agent functionality
- `cli`: Command line interface
- `api`: API endpoints
- `ui`: User interface components
- `modules`: Module system
- `schemas`: Schema definitions
- `security`: Security features
- `protocols`: A2A/MCP protocol support
- `docs`: Documentation
- `ci`: Continuous integration

## Examples
```
feat(core): add scheduler for automated tasks
fix(api): resolve authentication token validation
docs(schemas): update privacy model schema
test(modules): add unit tests for browser monitor
```

## Work Item References
Include work item IDs when applicable:
```
feat(protocols): implement A2A protocol support

RAE-001: Add Agent2Agent protocol integration
```

## Enforcement
- All commits must have descriptive messages
- No generic messages like "update" or "fix"
- Include scope when relevant
- Reference work items when applicable 