# Minimal Dependencies

## Purpose
Keep the project lightweight, secure, and maintainable by minimizing external dependencies.

## Rule
Only add dependencies that are absolutely necessary and well-justified.

## Dependency Evaluation Criteria
- **Essential**: Required for core functionality
- **Well-maintained**: Active development and security updates
- **Lightweight**: Minimal impact on binary size and performance
- **Secure**: No known security vulnerabilities
- **Licensed**: Compatible with project license

## Approval Process
1. Document why the dependency is needed
2. Research alternatives and justify choice
3. Review security implications
4. Update `/docs/DEPENDENCIES.md` with justification
5. Get approval from maintainers

## Dependency Categories

### Core Dependencies (Always Allowed)
- Runtime (tokio, serde)
- Security (ring, sha2)
- Schema validation (jsonschema)

### Optional Dependencies (Case-by-case)
- UI frameworks (only if native Web Components insufficient)
- Protocol libraries (A2A/MCP specific)
- Development tools (testing, linting)

### Forbidden Dependencies
- Large frameworks (React, Vue, Angular)
- Dynamic evaluation libraries
- Unmaintained or insecure packages
- Duplicate functionality

## Documentation
All dependencies must be documented in `/docs/DEPENDENCIES.md` with:
- Purpose and justification
- License type
- Update policy
- Security considerations

## Regular Review
- Monthly dependency audit
- Security vulnerability scanning
- Unused dependency removal
- Version update planning 