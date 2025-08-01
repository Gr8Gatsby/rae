# HIGH-UI-002: Scheduler Electron UI Integration

**Status:** IN_PROGRESS  
**Priority:** HIGH  
**Type:** UI Integration  
**Functional Spec Section:** 11 (Scheduling & Automation Engine) + 12 (UI Philosophy)  
**Dependencies:** HIGH-INTEGRATION-001-scheduler-cli-integration (COMPLETE)  

## 📋 **Overview**

Integrate the scheduler functionality into the Electron menu bar app, allowing users to manage scheduled jobs through a graphical interface while maintaining the existing CLI capabilities.

## 🎯 **Requirements**

### **Core Requirements**
- Add "Scheduled Jobs" menu item to Electron app
- Display list of scheduled jobs with status and next run time
- Allow job enable/disable from menu interface
- Show job execution history and results
- Integrate with existing Electron IPC system
- Update status indicator to show scheduler health
- Maintain cross-platform compatibility (macOS, Linux, Windows)

### **UI Requirements**
- Follow existing menu bar design patterns
- Provide clear job status indicators (enabled/disabled, running/failed)
- Show next execution time for scheduled jobs
- Allow quick job management actions
- Display job execution history with timestamps
- Integrate with existing status indicator system

### **Integration Requirements**
- Use existing `child_process.spawn()` pattern for Rust CLI calls
- Maintain current menu structure and navigation
- Preserve existing functionality (summary, config, status)
- Add scheduler-specific error handling
- Ensure proper error display in UI

### **Cross-Platform Requirements**
- **macOS**: Native menu bar integration with proper icon handling
- **Linux**: System tray integration with appropriate menu structure
- **Windows**: System tray integration with Windows-specific styling
- **Timezone Display**: Show job times in user's local timezone
- **Platform-Specific Features**: Adapt to platform conventions

## 🏗️ **Design**

### **Architecture**
```
Electron App (UI Layer)
├── Menu Bar Interface
│   ├── Status Indicator (scheduler health)
│   ├── Scheduled Jobs Menu
│   │   ├── Job List with Status
│   │   ├── Enable/Disable Actions
│   │   ├── Job History View
│   │   └── Add New Job Dialog
│   └── Existing Menu Items
└── IPC Layer
    └── child_process.spawn() → Rust CLI
        └── Scheduler Commands
            ├── scheduler list
            ├── scheduler status
            ├── scheduler add
            ├── scheduler remove
            └── scheduler enable/disable
```

### **Menu Structure**
```
Rae Agent
├── Status: [Online/Offline/Starting...]
├── ──────────────────────
├── 📊 Today's Summary
├── ⚙️  Configuration
├── ──────────────────────
├── 📅 Scheduled Jobs
│   ├── [Job Name] - [Status] - [Next Run]
│   ├── [Job Name] - [Status] - [Next Run]
│   ├── ──────────────────────
│   ├── ➕ Add New Job...
│   ├── 📋 View History...
│   └── ⚙️  Manage Jobs...
└── ──────────────────────
└── Quit
```

### **Integration Points**
- **Status Indicator**: Show scheduler health (green=healthy, yellow=warning, red=error)
- **Job List**: Real-time display of scheduled jobs with status
- **Job Actions**: Enable/disable jobs directly from menu
- **History View**: Show recent job executions and results
- **Add Job Dialog**: Simple form for creating new scheduled jobs

## 📝 **Implementation Plan**

### **Phase 1: Core Integration (Week 1)**
- [x] Update Electron app to include scheduler menu items
- [x] Implement `scheduler list` command integration
- [x] Display job status and next run time in menu
- [x] Add basic job enable/disable functionality
- [x] Update status indicator to show scheduler health

### **Phase 2: Enhanced Features (Week 2)**
- [ ] Implement job history display
- [ ] Add "Add New Job" dialog/form
- [ ] Create job management submenu
- [ ] Add job execution status indicators
- [ ] Implement error handling and user feedback

### **Phase 3: Polish & Testing (Week 3)**
- [ ] Cross-platform testing and fixes
- [ ] UI/UX improvements and refinements
- [ ] Performance optimization
- [ ] Documentation updates
- [ ] User testing and feedback integration

## 🔧 **Technical Details**

### **Electron Integration**
- **File**: `src/electron-app/simple-menu-bar.js`
- **Pattern**: Extend existing menu structure
- **IPC**: Use `child_process.spawn()` for Rust CLI calls
- **Error Handling**: Display errors in menu or notifications

### **Rust CLI Commands**
- **List Jobs**: `rae-agent scheduler list`
- **Job Status**: `rae-agent scheduler status <job-id>`
- **Add Job**: `rae-agent scheduler add <name> <schedule> <command>`
- **Remove Job**: `rae-agent scheduler remove <job-id>`
- **Enable/Disable**: `rae-agent scheduler enable/disable <job-id>`

### **Data Flow**
1. **User Action**: Click menu item or submenu
2. **Electron**: Spawn Rust CLI process with command
3. **Rust CLI**: Execute scheduler command and return JSON
4. **Electron**: Parse JSON response and update UI
5. **UI Update**: Refresh menu items with new data

### **Error Handling**
- **CLI Errors**: Display in menu or notification
- **Network Issues**: Show offline status
- **Permission Errors**: Guide user to fix permissions
- **Validation Errors**: Show specific error messages

## 🧪 **Testing Strategy**

### **Unit Tests**
- [ ] Menu item creation and display
- [ ] CLI command integration
- [ ] JSON parsing and error handling
- [ ] Cross-platform menu structure

### **Integration Tests**
- [ ] End-to-end job management workflow
- [ ] Status indicator updates
- [ ] Error handling scenarios
- [ ] Cross-platform compatibility

### **User Testing**
- [ ] Job creation and management
- [ ] Status monitoring and alerts
- [ ] Error recovery and feedback
- [ ] Cross-platform user experience

## ✅ **Success Criteria**

### **Functional Requirements**
- [ ] Users can view all scheduled jobs in menu
- [ ] Job status (enabled/disabled, next run time) is clearly displayed
- [ ] Users can enable/disable jobs from menu
- [ ] Job execution history is accessible
- [ ] New jobs can be created through UI
- [ ] Status indicator shows scheduler health
- [ ] All existing menu functionality is preserved

### **Technical Requirements**
- [ ] Cross-platform compatibility (macOS, Linux, Windows)
- [ ] Proper error handling and user feedback
- [ ] Performance: Menu updates in < 1 second
- [ ] Integration with existing Electron IPC system
- [ ] No breaking changes to existing functionality

### **User Experience Requirements**
- [ ] Intuitive menu navigation
- [ ] Clear status indicators and feedback
- [ ] Consistent with existing UI patterns
- [ ] Responsive to user actions
- [ ] Helpful error messages and guidance

## 🔗 **Related Work Items**

### **Dependencies**
- `HIGH-INTEGRATION-001-scheduler-cli-integration.md` (COMPLETE)
- `HIGH-CORE-001-scheduler-implementation.md` (COMPLETE)

### **Future Dependencies**
- `HIGH-CORE-002-module-runner.md` (PLANNING)
- `MEDIUM-MODULE-001-built-in-modules.md` (BACKLOG)

## 📝 **Notes**

### **Design Decisions**
- **Menu Integration**: Extend existing menu rather than create separate window
- **CLI Integration**: Use existing `child_process.spawn()` pattern for consistency
- **Status Display**: Integrate with existing status indicator system
- **Error Handling**: Provide clear feedback without disrupting user workflow

### **Technical Considerations**
- **Performance**: Menu updates should be fast and non-blocking
- **Memory**: Efficient handling of job lists and history
- **Cross-Platform**: Ensure consistent behavior across platforms
- **Security**: Validate all user inputs before passing to CLI

### **Future Enhancements**
- **Job Templates**: Pre-defined job types for common tasks
- **Advanced Scheduling**: More complex scheduling options
- **Job Dependencies**: Chain jobs together
- **Notifications**: System notifications for job events

## 🚀 **Deployment**

### **Build Requirements**
- **Electron**: Update to latest stable version
- **Node.js**: Ensure compatibility with current version
- **Rust**: No additional dependencies required
- **Cross-Platform**: Test on macOS, Linux, Windows

### **Distribution**
- **macOS**: Update `.dmg` package with new features
- **Linux**: Update `.deb` and `.rpm` packages
- **Windows**: Future distribution when Windows support is added

## 📚 **Documentation**

### **User Documentation**
- [ ] Update `docs/user/scheduler-guide.md` with UI instructions
- [ ] Add screenshots and examples
- [ ] Include troubleshooting section
- [ ] Document menu navigation

### **Developer Documentation**
- [ ] Update `docs/developer/scheduler-architecture.md`
- [ ] Document Electron integration patterns
- [ ] Add cross-platform development notes
- [ ] Include testing guidelines

## 📊 **Progress Tracking**

### **Current Status: IN_PROGRESS**
- [x] Requirements gathering
- [x] Design planning
- [x] Technical architecture defined
- [x] Implementation started
- [x] Core scheduler integration completed
- [x] Menu structure implemented
- [x] Job list display working
- [x] Enable/disable functionality added

### **Next Steps**
1. ✅ Create feature branch
2. ✅ Implement core menu integration
3. ✅ Add scheduler command integration
4. Test Electron app functionality
5. Add job history and management features
6. Polish UI and user experience

### **Estimated Timeline**
- **Phase 1**: 1 week
- **Phase 2**: 1 week  
- **Phase 3**: 1 week
- **Total**: 3 weeks

### **Risk Assessment**
- **Low Risk**: Menu integration (established pattern)
- **Medium Risk**: Cross-platform compatibility
- **Low Risk**: CLI integration (proven approach)
- **Medium Risk**: User experience and feedback

---

**Created:** 2024-12-19  
**Last Updated:** 2024-12-19  
**Assigned To:** Development Team  
**Reviewers:** UI/UX Team, Platform Team 