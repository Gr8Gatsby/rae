# Rae Functional Specification

**Version:** 1.0.0  
**Status:** Draft  
**Last Updated:** 2024-12-19  
**Work Item:** RAE-001  

## Table of Contents

1. [Overview & Goals](#overview--goals)
2. [Supported Platforms](#supported-platforms)
3. [System Architecture](#system-architecture)
4. [Module System](#module-system)
5. [Extension Points & Versioning](#extension-points--versioning)
6. [Security Hardening](#security-hardening)
7. [Testing Strategy](#testing-strategy)
8. [Protocol Support](#protocol-support)
9. [Privacy Model](#privacy-model)
10. [AI Behavior Guidelines (Future Phase)](#ai-behavior-guidelines-future-phase)
11. [Scheduling & Automation Engine](#scheduling--automation-engine)
12. [UI Philosophy](#ui-philosophy)
13. [Storage & Message Schema](#storage--message-schema)
14. [CLI & Local API Surface](#cli--local-api-surface)

---

## 1. Overview & Goals

### Target Users
Rae is designed for **privacy-conscious working professionals** who:
- Work with sensitive documents and data
- Need productivity assistance without cloud dependencies
- Value local-first solutions that respect their data sovereignty
- Want AI assistance that learns from their work patterns without compromising privacy

### Core Mission
Rae observes user work patterns (browser activity, file operations, system usage) to provide intelligent, contextual assistance while maintaining complete data privacy. All processing occurs locally unless explicitly permitted by the user.

### Primary Capabilities
- **Activity Monitoring**: Tracks browser history, file operations, and system usage patterns
- **Intelligent Summarization**: Generates daily/weekly activity summaries and insights
- **Task Automation**: Suggests and executes helpful tasks based on observed patterns
- **Contextual Assistance**: Provides relevant information and shortcuts based on current work context
- **Privacy-First Design**: All data remains local unless user explicitly opts to share

---

## 2. Supported Platforms

### Target Operating Systems
- **Primary**: macOS 12.0+ (Apple Silicon and Intel)
- **Secondary**: Linux (Ubuntu 20.04+, Debian 11+, Fedora 35+)
- **Future**: Windows 11+ (Phase 2)

### Cross-Platform Architecture Requirements
- **Core Components**: All core components must be cross-platform compatible
- **Scheduler Engine**: Platform-agnostic job scheduling with platform-specific adapters
- **File Operations**: Cross-platform file system operations using Rust's standard library
- **Process Management**: Native process spawning and management across platforms
- **Time Handling**: Timezone-aware scheduling with proper DST handling
- **Logging**: Platform-appropriate logging integration (syslog, Event Log, Console)
- **Background Services**: Platform-appropriate daemon/service management

### Platform-Specific Features
- **macOS**: Native menu bar integration, system keychain encryption
- **Linux**: systemd integration, syslog logging, standard Unix tools
- **Windows**: Windows Task Scheduler integration, Event Log, registry integration

### Installation Method
- **macOS**: Native `.dmg` installer with proper code signing
- **Linux**: `.deb` and `.rpm` packages with systemd integration
- **Distribution**: Direct download from official website, no app stores required
- **Installer Specification**: Build and behavior defined in `docs/specs/rae_installer_spec.md`

### System Requirements
- **Minimum**: 4GB RAM, 2GB storage, macOS 12.0+ / Ubuntu 20.04+
- **Recommended**: 8GB RAM, 5GB storage, SSD storage
- **Network**: Internet connection for initial setup and optional updates

---

## 3. System Architecture

### Core Components

#### Agent Scheduler (`rae-core/scheduler`)
- Manages all scheduled jobs and automation triggers
- Implements cron-like syntax for recurring tasks
- Handles job queuing, retry logic, and error recovery
- **Schema**: `/schemas/scheduler/job.json`

#### Module Runner (`rae-core/runner`)
- Isolated execution environment for all modules
- Sandboxed permissions and resource limits
- Handles module lifecycle (load, execute, cleanup)
- **Schema**: `/schemas/runner/module.json`

#### Storage Engine (`rae-core/storage`)
- Local file-based storage with versioned schemas
- Encrypted sensitive data storage
- Automatic backup and recovery mechanisms
- **Schema**: `/schemas/storage/container.json`

#### Local UI Layer (`rae-core/ui`)
- Native Web Components with Declarative Shadow DOM
- Minimal, responsive interface
- Real-time updates via local WebSocket
- **Schema**: `/schemas/ui/component.json`

#### Protocol Bridge (`rae-core/protocols`)
- A2A (Agent2Agent) protocol implementation as primary interface
- MCP (Model Context Protocol) integration for tool access
- Protocol translation and routing layer
- **Schema**: `/schemas/protocols/bridge.json`

### Communication Architecture

#### Message Bus (`rae-core/messaging`)
- Event-driven communication between modules
- Type-safe message routing with schema validation
- Support for both synchronous and asynchronous patterns
- **Schema**: `/schemas/messaging/event.json`

#### Protocol Communication Layer
- **A2A Server**: Rae acts as an A2A-compliant agent server
- **A2A Client**: Rae can communicate with other A2A agents
- **MCP Integration**: Rae uses MCP to access external tools and resources
- **Protocol Translation**: Seamless routing between internal modules and external protocols
- **Schema**: `/schemas/protocols/communication.json`

#### Module Communication Patterns
1. **Direct Calls**: For synchronous, high-performance operations
2. **Event Bus**: For loose coupling and async operations
3. **Shared Storage**: For persistent data exchange
4. **Local API**: For UI rendering and debugging
5. **Protocol APIs**: For external agent and tool communication

### Local-First Model
- **Data Sovereignty**: All user data remains on local machine
- **System Permissions**: Granular permission system for each module
- **Offline Capability**: Full functionality without internet connection
- **Encryption**: Sensitive data encrypted at rest using system keychain
- **Protocol Isolation**: External protocols only access permitted data through controlled interfaces

---

## 4. Module System

### Built-in Modules

#### Browser Monitor (`modules/browser-monitor`)
- **Purpose**: Track browser activity and visited sites
- **Permissions**: Browser history access, tab monitoring
- **Output**: Aggregated browsing patterns, site categories
- **Schema**: `/schemas/browser/activity.json`
- **Privacy**: Never stores URLs, only categories and time patterns

#### File Activity Monitor (`modules/file-monitor`)
- **Purpose**: Monitor file system changes and document activity
- **Permissions**: File system access, document type detection
- **Output**: File operation patterns, document categories
- **Schema**: `/schemas/file/activity.json`
- **Privacy**: Never stores file contents, only metadata and patterns

#### Daily Digest Generator (`modules/digest-generator`)
- **Purpose**: Create daily/weekly activity summaries
- **Input**: Aggregated data from other modules
- **Output**: Formatted summaries and insights
- **Schema**: `/schemas/digest/summary.json`
- **Schedule**: Daily at 6:00 PM, weekly on Sundays

#### System Usage Monitor (`modules/system-monitor`)
- **Purpose**: Track application usage and system patterns
- **Permissions**: Application usage statistics, system metrics
- **Output**: Application patterns, productivity insights
- **Schema**: `/schemas/system/usage.json`
- **Privacy**: Application names only, no content analysis

### User Modules (Extensions)

#### Module Definition Format
```json
{
  "name": "custom-module",
  "version": "1.0.0",
  "description": "Custom functionality",
  "permissions": ["file.read", "network.local"],
  "entry": "index.js",
  "schema": "/schemas/custom/input.json",
  "protocols": {
    "a2a": {
      "capabilities": ["message/send", "tasks/get"],
      "skills": ["file-analysis", "data-processing"]
    },
    "mcp": {
      "tools": ["file-reader", "data-processor"]
    }
  }
}
```

#### Loading and Execution Model
- **Dynamic Loading**: Modules loaded at runtime from `/modules/` directory
- **Sandboxed Execution**: Each module runs in isolated environment
- **Schema Validation**: All inputs/outputs validated against defined schemas
- **Error Isolation**: Module failures don't affect core system
- **Protocol Integration**: Modules can expose A2A and MCP capabilities

---

## 5. Extension Points & Versioning

### Module Registration
- **Discovery**: Automatic scanning of `/modules/` directory
- **Validation**: Schema and permission validation on load
- **Registration**: Modules register capabilities and permissions
- **Protocol Registration**: Modules register A2A and MCP capabilities
- **Schema**: `/schemas/module/registry.json`

### Sandboxing Rules
- **Resource Limits**: CPU, memory, and storage quotas per module
- **Permission Scopes**: Granular permissions (file.read, network.local, etc.)
- **Network Access**: Local-only by default, remote requires explicit permission
- **File System**: Restricted to designated directories
- **Protocol Access**: Controlled access to A2A and MCP capabilities based on permissions
- **Process Isolation**: Each module runs in isolated process space
- **System Call Filtering**: Restricted system call access based on module permissions

### Schema Validation
- **Input Validation**: All module inputs validated against schemas
- **Output Validation**: All module outputs validated before storage
- **Version Compatibility**: Schema versioning for backward compatibility
- **Schema Location**: All schemas stored in `/schemas/` directory

### Versioning Strategy
- **Core System**: Semantic versioning (major.minor.patch)
- **Module Versioning**: Independent versioning per module
- **Schema Versioning**: Versioned schemas with migration support
- **API Compatibility**: Backward compatibility for stable APIs

---

## 6. Security Hardening

### Module Security Model

#### Untrusted Module Protection
- **Code Signing**: All modules must be signed with valid certificates
- **Checksum Validation**: Module integrity verified using SHA-256 checksums
- **Signature Validation**: Digital signatures verified before module execution
- **Schema**: `/schemas/security/module-validation.json`

#### Privilege Escalation Prevention
- **Principle of Least Privilege**: Modules receive minimal required permissions
- **Capability-Based Security**: Permissions granted based on declared capabilities
- **Runtime Isolation**: Modules cannot access system resources beyond declared scope
- **Process Boundaries**: Strict process isolation prevents privilege escalation
- **Schema**: `/schemas/security/permissions.json`

#### Sandboxing Strategies
- **Container Isolation**: Optional container-based isolation for high-risk modules
- **System Call Interception**: Monitor and restrict system calls based on permissions
- **Memory Protection**: Isolated memory spaces prevent cross-module data access
- **Network Isolation**: Controlled network access with explicit allowlists
- **Schema**: `/schemas/security/sandbox.json`

### Security Validation Pipeline
- **Static Analysis**: Code analysis for security vulnerabilities before module load
- **Dynamic Analysis**: Runtime behavior monitoring for suspicious activities
- **Dependency Scanning**: Security audit of module dependencies
- **Schema**: `/schemas/security/validation.json`

### Incident Response
- **Automatic Suspension**: Immediate suspension of modules exhibiting malicious behavior
- **Audit Logging**: Complete security event logging for forensic analysis
- **User Notification**: Immediate notification of security incidents
- **Schema**: `/schemas/security/incident.json`

---

## 7. Testing Strategy

### Test Coverage Requirements
- **Minimum Coverage**: ≥80% test coverage required for all modules (per engineering standards)
- **Coverage Metrics**: Line coverage, branch coverage, and function coverage
- **Coverage Reporting**: Automated coverage reports for all test runs
- **Schema**: `/schemas/testing/coverage.json`

### Unit Testing
- **Core Logic**: Comprehensive unit tests for all core system components
- **Module Logic**: Individual unit tests for each module's business logic
- **Schema Validation**: Unit tests for all schema validation functions
- **Error Handling**: Unit tests for all error conditions and edge cases
- **Schema**: `/schemas/testing/unit.json`

### Integration Testing
- **Protocol Integration**: End-to-end tests for A2A and MCP protocol compliance
- **CLI Testing**: Integration tests for all command-line interface functionality
- **API Testing**: Comprehensive API endpoint testing with various scenarios
- **Module Interaction**: Tests for inter-module communication and data flow
- **Schema**: `/schemas/testing/integration.json`

### Schema Validation Testing
- **Schema Compliance**: Automated validation of all data structures against schemas
- **Schema Evolution**: Tests for backward compatibility during schema updates
- **Schema Documentation**: Tests ensuring schema documentation accuracy
- **Schema**: `/schemas/testing/schema.json`

### Security Testing
- **Module Security**: Tests for module sandboxing and privilege isolation
- **Protocol Security**: Tests for A2A and MCP security mechanisms
- **Data Privacy**: Tests ensuring sensitive data is properly protected
- **Schema**: `/schemas/testing/security.json`

### Performance Testing
- **Load Testing**: Performance tests under various load conditions
- **Memory Testing**: Memory usage and leak detection tests
- **Response Time**: Tests for acceptable response times across all interfaces
- **Schema**: `/schemas/testing/performance.json`

### Test Automation
- **Continuous Integration**: Automated test execution on all code changes
- **Test Environment**: Isolated test environments for reproducible results
- **Test Data**: Comprehensive test data sets for various scenarios
- **Schema**: `/schemas/testing/automation.json`

---

## 8. Protocol Support

### A2A (Agent2Agent) Protocol Integration

#### Rae as A2A Server
- **Agent Card**: Rae exposes an A2A-compliant agent card at `/api/a2a/card`
- **Core Methods**: Implements required A2A methods (`message/send`, `tasks/get`, `tasks/cancel`)
- **Optional Methods**: Supports streaming (`message/stream`) and push notifications
- **Transport**: JSON-RPC 2.0 over HTTP with optional gRPC support
- **Schema**: `/schemas/protocols/a2a/server.json`

#### Rae as A2A Client
- **Agent Discovery**: Can discover and connect to other A2A-compliant agents
- **Task Delegation**: Can delegate tasks to external agents when appropriate
- **Result Integration**: Integrates external agent results into local workflows
- **Schema**: `/schemas/protocols/a2a/client.json`

#### A2A Capabilities
- **File Analysis**: Accept file processing requests from external agents
- **Data Processing**: Provide data analysis and summarization services
- **Task Coordination**: Coordinate multi-agent workflows
- **Privacy Controls**: Strict data filtering before external communication

### MCP (Model Context Protocol) Integration

#### Rae as MCP Server
- **Tool Exposure**: Exposes local tools and capabilities via MCP
- **Resource Access**: Provides controlled access to local files and data
- **Tool Discovery**: Implements MCP tool discovery and description
- **Schema**: `/schemas/protocols/mcp/server.json`

#### Rae as MCP Client
- **External Tools**: Uses MCP to access external tools and APIs
- **Resource Integration**: Integrates external resources into local workflows
- **Tool Coordination**: Manages multiple MCP connections for complex tasks
- **Schema**: `/schemas/protocols/mcp/client.json`

#### MCP Tool Categories
- **File Tools**: File reading, writing, and analysis capabilities
- **Data Tools**: Data processing, transformation, and analysis
- **System Tools**: System monitoring and control capabilities
- **Network Tools**: Controlled network access for external resources

### Protocol Security & Privacy

#### A2A Security
- **Authentication**: Bearer token authentication for all A2A interactions
- **Authorization**: Granular permission control for external agent access
- **Data Filtering**: Automatic filtering of sensitive data before external transmission
- **Audit Logging**: Complete audit trail of all external agent interactions

#### MCP Security
- **Tool Permissions**: Granular permissions for each MCP tool
- **Data Access Control**: Controlled access to local data through MCP
- **Resource Limits**: Quotas and rate limiting for external tool usage
- **Privacy Validation**: Automatic validation of data privacy before tool usage

#### Privacy-Preserving Protocol Design
- **Local Processing**: All sensitive processing remains local
- **Anonymized Outputs**: Only anonymized, aggregated data shared externally
- **User Consent**: Explicit user approval for any external data sharing
- **Audit Trail**: Complete logging of all external protocol interactions

---

## 9. Privacy Model

### What Rae Never Sends
- **URLs**: Browser URLs and visited web addresses
- **File Names**: Specific document and file names
- **File Contents**: Actual content of documents
- **Personal Data**: Names, addresses, or identifiable information
- **System Details**: Hardware identifiers or system specifics

### What Rae May Summarize (with Permission)
- **Activity Patterns**: Time spent on different types of sites
- **Document Categories**: Types of documents worked on
- **Productivity Metrics**: Application usage patterns
- **Aggregated Statistics**: Non-identifying usage statistics

### User Approval System
- **Granular Permissions**: Per-module permission requests
- **Explicit Opt-in**: No data sharing without explicit consent
- **Transparency**: Clear indication of what data is collected
- **Revocation**: Users can revoke permissions at any time
- **Schema**: `/schemas/privacy/permissions.json`

### Data Retention
- **Local Storage**: All data stored locally with encryption
- **Retention Policies**: Configurable data retention periods
- **Data Export**: User-controlled data export capabilities
- **Data Deletion**: Complete data removal on uninstall

---

## 10. AI Behavior Guidelines (Future Phase)

### Autonomous Action Conditions
- **Scheduled Triggers**: Actions triggered by predefined schedules (daily digests, weekly summaries)
- **Pattern Recognition**: Actions based on observed user behavior patterns
- **Threshold-Based**: Actions triggered when usage metrics exceed defined thresholds
- **Schema**: `/schemas/ai/triggers.json`

### User Notification Mechanisms
- **Pre-Action Notification**: User notified before any autonomous action is taken
- **Action Confirmation**: Critical actions require explicit user confirmation
- **Post-Action Summary**: User receives summary of actions taken on their behalf
- **Opt-Out Options**: Users can disable specific autonomous behaviors
- **Schema**: `/schemas/ai/notifications.json`

### Trust Maintenance
- **Transparency**: All AI decisions and actions are logged and explainable
- **User Control**: Users maintain ultimate control over all automated actions
- **Audit Trail**: Complete audit trail of all AI-initiated actions
- **Reversibility**: Actions can be undone or corrected when possible
- **Schema**: `/schemas/ai/trust.json`

### AI Action Categories
- **Informational**: Providing insights and summaries (always safe)
- **Organizational**: File organization and categorization (user confirmable)
- **Communicational**: Drafting messages or responses (user reviewable)
- **System**: System optimization and maintenance (user controllable)
- **Schema**: `/schemas/ai/actions.json`

---

## 11. Scheduling & Automation Engine

### Cross-Platform Requirements

#### Platform-Specific Scheduling
- **macOS**: Native cron-like scheduling with timezone support
- **Linux**: Standard cron syntax with systemd integration
- **Windows**: Windows Task Scheduler adapter for future compatibility
- **Timezone Handling**: Automatic timezone detection and conversion
- **Daylight Saving Time**: Proper DST handling across all platforms

#### Platform-Agnostic Features
- **JSON Job Storage**: Platform-independent job persistence
- **Async Job Execution**: Tokio runtime for cross-platform threading
- **Error Recovery**: Platform-agnostic retry and failure handling
- **Resource Limits**: Configurable memory and CPU limits per platform
- **Process Management**: Native Rust process spawning across platforms

#### Platform-Specific Adaptations
- **File System Monitoring**: Platform-specific file system event APIs
- **System Event Triggers**: Native system event integration per platform
- **Background Process Management**: Platform-appropriate daemon/service handling
- **Logging Integration**: Native logging systems (syslog, Event Log, Console)

### Scheduled Jobs
- **Daily Digest**: Generated at 6:00 PM daily
- **Weekly Summary**: Generated Sundays at 5:00 PM
- **File Scan**: Hourly file system activity monitoring
- **System Cleanup**: Weekly cleanup of temporary data
- **Schema**: `/schemas/scheduler/job.json`

### Conditions and Triggers
- **Time-based**: Cron-like scheduling for recurring tasks
- **Event-based**: Triggers based on system events
- **Pattern-based**: Actions triggered by observed patterns
- **Manual**: User-initiated task execution

### Automation Examples
- **"If visited X daily, offer digest"**: Pattern-based digest generation
- **"When working on project Y, suggest related files"**: Context-aware suggestions
- **"After 2 hours of coding, suggest break"**: Productivity reminders
- **"Weekly file organization"**: Automated file categorization

### Manual vs. Auto-run Jobs
- **Manual Jobs**: User-initiated tasks (CLI commands, UI actions)
- **Auto-run Jobs**: Background tasks based on schedules or triggers
- **Hybrid Jobs**: Scheduled jobs that can be manually triggered

---

## 12. UI Philosophy

### Native Web Components
- **Framework**: Pure Web Components with Declarative Shadow DOM
- **Styling**: CSS custom properties for theming
- **Accessibility**: WCAG 2.1 AA compliance
- **Performance**: Minimal JavaScript, fast rendering

### Declarative Shadow DOM
- **Purpose**: Eliminate layout shift and flickering
- **Implementation**: Server-side rendering of shadow DOM
- **Benefits**: Improved performance and user experience
- **Compatibility**: Progressive enhancement for older browsers

### UI Design Principles
- **Minimal Interface**: Clean, uncluttered design
- **Summary Views**: High-level activity summaries
- **One-click Tasks**: Quick actions for common tasks
- **Progressive Disclosure**: Details available on demand
- **Responsive Design**: Works on various screen sizes

### UI Components
- **Dashboard**: Main activity overview and quick actions
- **Module Manager**: Module configuration and status
- **Settings**: Privacy and system configuration
- **Activity Feed**: Real-time activity stream
- **Schema**: `/schemas/ui/layout.json`

---

## 13. Storage & Message Schema

### Schema-First Architecture
- **All Data**: Must conform to defined schemas in `/schemas/`
- **Version Control**: Schemas versioned and backward compatible
- **Validation**: Runtime validation of all data structures
- **Documentation**: Self-documenting schemas with examples

### Storage Strategy
- **Local File System**: JSON files organized by module and type
- **Versioned Formats**: All data includes schema version
- **Encryption**: Sensitive data encrypted using system keychain
- **Backup**: Automatic backup with user control

### Message Schema Requirements
- **Module Inputs**: All module inputs must have defined schemas
- **Module Outputs**: All module outputs must have defined schemas
- **Event Messages**: All event bus messages must have defined schemas
- **API Responses**: All API responses must have defined schemas

### Schema Categories
- **Core Schemas**: System-level data structures
- **Module Schemas**: Module-specific data structures
- **UI Schemas**: Interface and layout definitions
- **Privacy Schemas**: Permission and consent structures

---

## 14. CLI & Local API Surface

### Command Line Interface

#### Core Commands
```bash
rae status                    # Show system status and module health
rae run <module> [args]      # Manually run a module
rae digest [--weekly]        # Generate digest (daily or weekly)
rae modules list             # List all installed modules
rae modules install <name>   # Install a module
rae modules uninstall <name> # Uninstall a module
rae config get <key>         # Get configuration value
rae config set <key> <value> # Set configuration value
```

#### Development Commands
```bash
rae dev start               # Start development mode
rae dev test <module>       # Run tests for specific module
rae dev build <module>      # Build a module
rae dev validate <schema>   # Validate a schema file
rae dev protocols a2a       # Test A2A protocol compliance
rae dev protocols mcp       # Test MCP protocol compliance
```

### Local API Surface

#### REST API (Optional)
- **Port**: 3000 (configurable)
- **Authentication**: Local token-based auth
- **CORS**: Disabled (local-only access)
- **Schema**: `/schemas/api/endpoints.json`

#### API Endpoints
```
GET  /api/status           # System status
GET  /api/modules          # List modules
POST /api/modules/<name>   # Run module
GET  /api/data/<type>      # Get data by type
POST /api/config           # Update configuration

# A2A Protocol Endpoints
GET  /api/a2a/card         # A2A agent card
POST /api/a2a/message/send # A2A message endpoint
GET  /api/a2a/tasks/<id>   # A2A task status
POST /api/a2a/tasks/cancel # A2A task cancellation

# MCP Protocol Endpoints
GET  /api/mcp/tools        # MCP tool discovery
POST /api/mcp/tools/call   # MCP tool execution
GET  /api/mcp/resources    # MCP resource listing
```

#### WebSocket API
- **Real-time Updates**: Module status and activity feed
- **Event Streaming**: Live event stream for UI updates
- **Schema**: `/schemas/api/websocket.json`

### API Security
- **Local Only**: No external network access
- **Token Authentication**: Simple token-based auth
- **Rate Limiting**: Prevent abuse of local API
- **Schema Validation**: All API inputs/outputs validated

---

## Implementation Phases

### Phase 1: Core System (Weeks 1-4)
- Core scheduler and module runner
- Basic storage and messaging
- CLI interface
- Essential built-in modules
- Installer implementation (specified in `docs/specs/rae_installer_spec.md`)

### Phase 2: UI and Extensions (Weeks 5-8)
- Web Components UI
- Module extension system
- Privacy controls
- Local API surface

### Phase 3: Protocol Integration (Weeks 9-12)
- A2A protocol implementation
- MCP protocol integration
- Protocol security and privacy controls
- External agent and tool coordination

### Phase 4: Advanced Features (Weeks 13-16)
- Advanced automation engine
- Third-party module support
- Enhanced privacy features
- Performance optimization

---

## Compliance with Engineering Standards

This specification complies with all requirements from `rae_engineering_standards.md`:

- ✅ **Schema-first architecture**: All data structures defined in `/schemas/`
- ✅ **80%+ test coverage**: Testing requirements specified for all modules
- ✅ **Commit message conventions**: Work item IDs included (RAE-001)
- ✅ **No side effects**: Core logic isolated from side effects
- ✅ **Minimal dependencies**: Focus on native Web Components and minimal frameworks
- ✅ **Design-first approach**: Specification completed before implementation
- ✅ **Protocol compliance**: A2A and MCP protocol support integrated into architecture

---

**Work Item:** RAE-001  
**Status:** Ready for Review  
**Next Steps:** Technical design review and implementation planning 