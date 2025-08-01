# Scheduler CLI and UI Integration

**ID:** `HIGH-INTEGRATION-001-scheduler-cli-integration`  
**Status:** `PLANNING`  
**Priority:** `HIGH`  
**Created:** `2024-12-19`  
**Updated:** `2024-12-19`  
**Functional Spec Section:** `Section 11 - Scheduling & Automation Engine` (Integration)

## 📋 **Overview**

**Description:** Integrate the completed core scheduler system with the existing CLI and Electron UI, enabling users to manage scheduled jobs through both command-line and graphical interfaces.

**Purpose:** Connect the scheduler foundation to user-facing interfaces, making the automation capabilities accessible and usable. This bridges the gap between the core scheduler implementation and user interaction.

**Functional Spec Compliance:** Implements the integration layer for Section 11 - Scheduling & Automation Engine, enabling user access to scheduled jobs and automation features.

## 🎯 **Requirements**

### **CLI Integration Requirements:**
- [ ] Add `scheduler` subcommand to `rae-agent` CLI
- [ ] Implement `scheduler add` command with job creation
- [ ] Implement `scheduler list` command with job status
- [ ] Implement `scheduler remove` command for job deletion
- [ ] Implement `scheduler status` command for job monitoring
- [ ] Implement `scheduler enable/disable` commands
- [ ] Add help and documentation for all scheduler commands
- [ ] Integrate with existing CLI argument parsing

### **Electron UI Integration Requirements:**
- [ ] Add "Scheduled Jobs" menu item to Electron menu bar
- [ ] Display list of scheduled jobs in menu
- [ ] Show job status (enabled/disabled, next run time)
- [ ] Allow job enable/disable from menu
- [ ] Add job creation dialog or form
- [ ] Show job execution history
- [ ] Integrate with existing Electron IPC system
- [ ] Update status indicator to show scheduler health

### **Cross-Platform Requirements:**
- [ ] CLI commands work consistently across macOS, Linux, Windows
- [ ] Electron UI displays correctly on all supported platforms
- [ ] Job scheduling respects platform-specific timezone handling
- [ ] Error messages are platform-appropriate
- [ ] File paths are handled cross-platform

### **Schema Requirements:**
- **Schema File:** `/schemas/scheduler/cli.json`
- **Schema Description:** Defines CLI command structure and job management interface
- **Schema Version:** `1.0.0`

## 🏗️ **Design**

### **CLI Architecture:**
```
rae-agent scheduler [subcommand] [options]
├── add     - Create new scheduled job
├── list    - List all scheduled jobs
├── remove  - Delete scheduled job
├── status  - Show job status and details
├── enable  - Enable disabled job
├── disable - Disable enabled job
└── help    - Show command help
```

### **Electron UI Architecture:**
```
Menu Bar
├── Status: [Scheduler Health]
├── Scheduled Jobs
│   ├── [Job Name] - [Status] - [Next Run]
│   ├── [Job Name] - [Status] - [Next Run]
│   └── Add New Job...
└── Job History
    └── [Recent Executions]
```

### **Integration Points:**
- **CLI → Scheduler:** Direct Rust function calls to scheduler API
- **Electron → Scheduler:** IPC calls to Rust CLI commands
- **Status Updates:** Real-time job status propagation
- **Error Handling:** Consistent error reporting across interfaces

## 📁 **Implementation Plan**

### **Phase 1: CLI Integration**
- [ ] Add scheduler module to CLI argument parsing
- [ ] Implement basic scheduler commands (add, list, remove)
- [ ] Add job status and management commands
- [ ] Integrate with existing CLI error handling
- [ ] Add comprehensive help and documentation

### **Phase 2: Electron UI Integration**
- [ ] Add scheduler menu items to Electron app
- [ ] Implement job listing and status display
- [ ] Add job enable/disable functionality
- [ ] Create job management dialogs
- [ ] Integrate with existing status indicator

### **Phase 3: Advanced Features**
- [ ] Add job creation wizard/form
- [ ] Implement job execution history
- [ ] Add scheduler health monitoring
- [ ] Create job template system
- [ ] Add bulk job operations

## 🔧 **Technical Details**

### **Dependencies:**
- `clap` - CLI argument parsing (already in use)
- `serde` - JSON serialization for job data
- `tokio` - Async runtime for job operations
- `tracing` - Logging for scheduler operations

### **Files to Create/Modify:**
- `src/agent/src/main.rs` - Add scheduler CLI commands
- `src/agent/src/cli.rs` - CLI argument parsing for scheduler
- `src/electron-app/simple-menu-bar.js` - Add scheduler menu items
- `src/agent/src/scheduler/cli.rs` - CLI-specific scheduler functions

### **API Surface:**
```rust
// CLI scheduler commands
#[derive(Subcommand)]
enum SchedulerCommand {
    Add {
        name: String,
        schedule: String,
        command: String,
        #[arg(long)]
        timezone: Option<String>,
    },
    List {
        #[arg(long)]
        verbose: bool,
    },
    Remove {
        job_id: String,
    },
    Status {
        job_id: Option<String>,
    },
    Enable {
        job_id: String,
    },
    Disable {
        job_id: String,
    },
}
```

## 🧪 **Testing Strategy**

### **CLI Tests:**
- Command argument parsing tests
- Job creation and management tests
- Error handling tests
- Cross-platform compatibility tests

### **Electron UI Tests:**
- Menu item functionality tests
- Job status display tests
- IPC communication tests
- UI responsiveness tests

### **Integration Tests:**
- End-to-end job lifecycle tests
- CLI ↔ Electron communication tests
- Cross-platform integration tests

## 📊 **Success Criteria**

### **Functional Success:**
- [ ] Users can create jobs via CLI: `rae-agent scheduler add --name "daily" --schedule "0 18 * * *" --command "rae-agent summary"`
- [ ] Users can list jobs via CLI: `rae-agent scheduler list`
- [ ] Users can manage jobs via Electron menu
- [ ] Job status updates in real-time
- [ ] Error messages are clear and actionable

### **Technical Success:**
- [ ] All CLI commands work across platforms
- [ ] Electron UI displays correctly on all platforms
- [ ] Job operations complete in < 1 second
- [ ] Memory usage remains under 100MB
- [ ] No memory leaks in long-running operations

## 🔗 **Related Work Items**

### **Dependencies:**
- `HIGH-CORE-001-scheduler-implementation` - Required for scheduler functionality
- `HIGH-UI-001-electron-menu-bar-app` - Required for UI integration
- `HIGH-RUST-001-core-cli-backend` - Required for CLI integration

### **Dependents:**
- `HIGH-CORE-002-module-runner` - This work enables module scheduling
- `MEDIUM-MODULE-001-built-in-modules` - This work enables module automation

## 📝 **Implementation Notes**

### **Design Decisions:**
- **CLI-first approach:** Implement CLI commands first, then connect to UI
- **Consistent API:** Use same job management interface for CLI and UI
- **Error propagation:** Ensure errors from scheduler reach user interfaces
- **Status synchronization:** Keep CLI and UI status displays in sync

### **Trade-offs:**
- **CLI complexity:** Rich CLI vs simple UI (chose rich CLI for power users)
- **UI responsiveness:** Real-time updates vs polling (chose polling for simplicity)
- **Error handling:** Detailed errors vs user-friendly messages (chose both)

## 🚀 **Deployment**

### **Release Plan:**
- Phase 1: Basic CLI integration with core commands
- Phase 2: Electron UI integration with job management
- Phase 3: Advanced features and optimizations

### **Rollback Plan:**
- Feature flag to disable scheduler integration
- Fallback to manual job management
- Preserve existing CLI and UI functionality

## 📚 **Documentation**

### **User Documentation:**
- [ ] CLI scheduler command reference
- [ ] Electron UI scheduler guide
- [ ] Job management best practices

### **Developer Documentation:**
- [ ] Integration architecture guide
- [ ] CLI command development guide
- [ ] Electron UI integration guide

## 🔄 **Progress Tracking**

### **Current Status:**
**⏳ PLANNING:**
- Requirements defined
- Design architecture planned
- Implementation approach determined

**🔄 NEXT STEPS:**
- Create CLI command structure
- Implement basic scheduler commands
- Add Electron menu integration

### **Commits:**
- `[pending]` - Initial CLI integration
- `[pending]` - Basic scheduler commands
- `[pending]` - Electron UI integration

### **Milestones:**
- [ ] Design approved
- [ ] CLI integration complete
- [ ] Electron UI integration complete
- [ ] Testing complete
- [ ] Documentation complete
- [ ] Ready for review 