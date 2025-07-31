# Commits to Main

## Purpose
Maintain code quality and project stability by controlling what gets merged to main.

## Rule
No direct commits to main branch. All changes must go through feature branches and pull requests.

## Workflow
1. Create feature branch from main
2. Make changes and commit with proper messages
3. Write/update tests to maintain ≥80% coverage
4. Ensure all tests pass
5. Create pull request with description
6. Code review and approval required
7. Merge to main only after approval

## Branch Naming
- Feature branches: `feature/RAE-XXX-description`
- Bug fixes: `fix/RAE-XXX-description`
- Documentation: `docs/RAE-XXX-description`
- Refactoring: `refactor/RAE-XXX-description`

## Pull Request Requirements
- Descriptive title and description
- Reference work item ID (RAE-XXX)
- List changes made
- Include test coverage report
- Link to relevant documentation updates

## Merge Criteria
- All tests passing
- ≥80% test coverage maintained
- Code review approved
- No linting errors
- Schema validation passes
- Security checks pass

## Protection
- Main branch protected from direct pushes
- Required status checks must pass
- Required pull request reviews
- No force pushes to main 