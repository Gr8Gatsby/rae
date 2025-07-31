# Rae Engineering Process & Development Standards

## 📘 Documentation Standards
- All specifications are written in **Markdown**
- Stored in a dedicated `/docs/` folder
- Central **Functional Specification** is versioned, structured, and always up to date

## 🔌 Extension API Requirements
- All extension points must have:
  - Clean, typed API definitions (JSON Schema, TypeScript types, or Rust traits)
  - Versioned separately
  - Backward compatibility strategy documented

## 🧱 Schema & Messaging
- All data storage and message formats must follow **strict schemas**
- Schemas are:
  - Typed and validated
  - Stored in `/schemas`
  - Required in all messaging and module boundaries

## ✅ Development Rules
- Design-first: No feature development without an approved **design document**
- All specs must include purpose, interface, schema, and constraints
- All modules must be documented, testable, and isolated

## 📋 Work Tracking & History
- Log all **epics, stories, bugs** to maintain history beyond Git
- Each Git commit must reference a corresponding **work item summary**
- All changes should be traceable to project goals

## 🔒 Privacy by Design
- Rae must never upload content (URLs, filenames, etc.)
- Aggregated usage data must:
  - Be anonymized
  - Be user-visible before sending
  - Require explicit opt-in

## 🧪 Testing Standards
- **80% minimum test coverage** required before merging a feature
- All new code must include:
  - Unit tests
  - Integration tests
  - Platform-specific tests (macOS/Linux first, Windows optional)
- All tests must **pass before merging into `main`**

## 🌿 Branching & CI Rules
- **Feature branches only** for development
- No direct commits to `main`
- CI must verify:
  - Lint + formatting
  - Schema validation
  - 80%+ test coverage
  - Passing test suite
  - No TODOs or console logs in production code

## 📦 Dependency Governance
- Keep dependencies **minimal and necessary**
- Maintain `/docs/DEPENDENCIES.md` with:
  - Purpose and justification
  - License type
  - Update policy
- Avoid large frameworks or dynamic eval libraries

## 🔐 Security Ruleset
- No remote execution or upload features without approval
- Lint/CI checks for unsafe patterns (e.g., unescaped shell calls)
- Modules must run in isolated, permission-scoped environments

## ✨ Feature Flags
- All new modules behind configurable **feature flags**
- Controlled via `rae.config.json`
- Clearly marked as optional, experimental, or stable

## 🚀 Release Strategy
- Semantic versioning for:
  - Core system (`rae-core@1.0.0`)
  - Each module individually
  - Schema and API versions
- Release must include:
  - Changelog
  - Binary fingerprint
  - Regression test results
  - Breaking changes summary

## 🛠 Dev Manual (Recommended)
Create `/docs/engineering-principles.md` including:
- Naming conventions
- Folder/module layout
- How to add a new module
- Test expectations
- Contribution and commit guidelines
- CI/release checklist

---

Rae is a scalable, privacy-respecting, AI-enabled assistant platform. This process ensures it remains high-quality, auditable, and extensible.