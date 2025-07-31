# Rae - Local-First AI Assistant Platform

Rae is a **local-first, privacy-respecting AI assistant platform** that observes user work patterns to provide intelligent, contextual assistance while maintaining complete data sovereignty. All processing occurs locally unless explicitly permitted by the user.

## 🎯 Core Mission

Rae helps privacy-conscious working professionals by:
- **Observing** browser activity, file operations, and system usage patterns
- **Summarizing** daily/weekly activities and providing insights
- **Automating** helpful tasks based on observed patterns
- **Assisting** with contextual information and shortcuts
- **Protecting** user privacy through local-first design

## 🏗️ Project Structure

### Directory Overview

```
rae/
├── src/                    # All source code
│   ├── agent/             # Core Rust agent (CLI, scheduler, modules)
│   ├── browser-extension/ # Future browser monitoring plugin
│   ├── plugin-devkit/     # Tooling for building extension modules
│   └── experimental-ui-shell/ # Optional standalone GUI shell
├── docs/                  # Engineering documentation and specifications
│   ├── specs/            # Functional specifications and design docs
│   └── engineering/      # Engineering standards and processes
├── .cursor/              # Cursor IDE rules and AI prompts
│   └── rules/           # Development rules and constraints
└── .gitignore           # System files and tooling noise
```

### Source Code Organization (`src/`)

#### `agent/` - Core Rust Agent ⭐ **Active**
The heart of Rae, written in Rust for performance and security:
- **CLI Interface**: Command-line tools for power users
- **Core Logic**: Scheduler, module runner, and storage engine
- **API Layer**: REST and WebSocket APIs for local communication
- **Module System**: Extensible module architecture with sandboxing
- **Schema Validation**: Strict schema enforcement for all data
- **UI Components**: Native Web Components for the interface
- **Test Suite**: Comprehensive testing with ≥80% coverage

#### `browser-extension/` - Browser Monitoring 🔮 **Future**
Optional browser extension for enhanced activity monitoring:
- **Privacy-First**: Only collects anonymized usage patterns
- **Local Processing**: All data processed locally
- **User Control**: Granular permission controls
- **Cross-Browser**: Support for major browsers

#### `plugin-devkit/` - Module Development Tools 🔧 **Future**
Tooling for building and validating Rae modules:
- **Module Templates**: Scaffolding for new modules
- **Validation Tools**: Schema and security validation
- **Testing Framework**: Module testing utilities
- **Documentation Generator**: Auto-generated module docs

#### `experimental-ui-shell/` - Standalone GUI 🎨 **Future**
Optional standalone GUI application (e.g., Tauri):
- **Native Performance**: Desktop-native UI experience
- **Cross-Platform**: Windows, macOS, Linux support
- **Web Components**: Consistent with core UI philosophy
- **Offline-First**: Full functionality without internet

### Documentation (`docs/`)

#### `specs/` - Functional Specifications
- **`functional_spec.md`**: Complete system specification
- **`rae_installer_spec.md`**: Installer build and behavior
- **`protocol_specs/`**: A2A and MCP protocol specifications

#### `engineering/` - Engineering Standards
- **`rae_engineering_standards.md`**: Core development standards
- **`DEPENDENCIES.md`**: Dependency management and justification
- **`CONTRIBUTING.md`**: Development workflow and guidelines

### Development Rules (`.cursor/rules/`)

AI-powered development constraints and guidelines:
- **Commit Message Standards**: Conventional commit format with work item IDs
- **Test Coverage**: ≥80% coverage requirement for all modules
- **Main Branch Protection**: No direct commits, pull request workflow
- **Minimal Dependencies**: Strict dependency evaluation and justification
- **Pure Functions**: No side effects in main modules
- **Schema Enforcement**: Strict schema validation for all data

## 🔧 Key Features

### Privacy-First Design
- **Local Processing**: All sensitive data processed locally
- **Data Sovereignty**: User maintains complete control over their data
- **Granular Permissions**: Fine-grained permission system
- **Audit Trail**: Complete logging of all operations

### Modular Architecture
- **Schema-First**: All data structures defined by strict schemas
- **Sandboxed Modules**: Isolated execution environments
- **Protocol Support**: A2A and MCP protocol integration
- **Extensible**: Easy to add new modules and capabilities

### Security Hardening
- **Code Signing**: All modules digitally signed
- **Checksum Validation**: Module integrity verification
- **Privilege Isolation**: Principle of least privilege
- **Security Testing**: Comprehensive security validation

### Testing Strategy
- **≥80% Coverage**: Comprehensive test coverage requirement
- **Unit Testing**: Core logic and module testing
- **Integration Testing**: Protocol and API testing
- **Security Testing**: Sandboxing and privacy validation

## 🚀 Development

### Prerequisites
- Rust 1.70+ (for core agent)
- Node.js 18+ (for UI components)
- Git (for version control)

### Quick Start
```bash
# Clone the repository
git clone https://github.com/rae-project/rae.git
cd rae

# Build the core agent
cd src/agent
cargo build

# Run tests
cargo test

# Start development server
cargo run -- dev start
```

### Development Workflow
1. **Feature Branches**: Create feature branches from main
2. **Schema-First**: Define schemas before implementation
3. **Test Coverage**: Maintain ≥80% test coverage
4. **Code Review**: All changes require review
5. **Pull Requests**: No direct commits to main

### Testing
```bash
# Run all tests
cargo test

# Run with coverage
cargo tarpaulin --out Html

# Run specific test categories
cargo test --test unit
cargo test --test integration
cargo test --test security
```

## 📋 Project Status

### Phase 1: Core System (Weeks 1-4) 🚧 **In Progress**
- [x] Project structure and documentation
- [ ] Core scheduler and module runner
- [ ] Basic storage and messaging
- [ ] CLI interface
- [ ] Essential built-in modules
- [ ] Installer implementation

### Phase 2: UI and Extensions (Weeks 5-8) 📋 **Planned**
- [ ] Web Components UI
- [ ] Module extension system
- [ ] Privacy controls
- [ ] Local API surface

### Phase 3: Protocol Integration (Weeks 9-12) 📋 **Planned**
- [ ] A2A protocol implementation
- [ ] MCP protocol integration
- [ ] Protocol security and privacy controls
- [ ] External agent and tool coordination

### Phase 4: Advanced Features (Weeks 13-16) 📋 **Planned**
- [ ] Advanced automation engine
- [ ] Third-party module support
- [ ] Enhanced privacy features
- [ ] Performance optimization

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](docs/engineering/CONTRIBUTING.md) for details on:

- **Development Standards**: Following `rae_engineering_standards.md`
- **Code Quality**: ≥80% test coverage and schema validation
- **Security**: Security-first development practices
- **Documentation**: Comprehensive documentation requirements

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🔗 Links

- **Documentation**: [docs/](docs/)
- **Engineering Standards**: [docs/engineering/rae_engineering_standards.md](docs/engineering/rae_engineering_standards.md)
- **Functional Specification**: [docs/specs/functional_spec.md](docs/specs/functional_spec.md)
- **Issues**: [GitHub Issues](https://github.com/rae-project/rae/issues)

---

**Rae** - Empowering users with local-first, privacy-respecting AI assistance. 