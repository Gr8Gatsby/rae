# Development Setup

## 🚀 **Quick Start**

### **Prerequisites:**
- Node.js 18+
- Rust 1.70+
- Git

### **Setup Commands:**
```bash
# Clone and setup
git clone <repo>
cd rae

# Install dependencies
npm run install:all

# Build everything
npm run build

# Start development
npm run dev
```

## 🏗️ **Project Structure**

```
rae/
├── src/
│   ├── agent/              # Rust CLI backend
│   │   ├── src/
│   │   │   ├── main.rs     # CLI entry point
│   │   │   └── tray.rs     # File operations
│   │   └── Cargo.toml
│   └── electron-app/       # Electron UI frontend
│       ├── simple-menu-bar.js  # Main app
│       ├── assets/         # Icons and resources
│       └── package.json
├── docs/
│   └── engineering/        # Development docs
└── package.json           # Root scripts
```

## 🎯 **Development Workflow**

### **Branch Strategy:**
1. **Create feature branch:** `git checkout -b feature/[name]`
2. **Make changes:** Edit code, test locally
3. **Commit regularly:** `git commit -m "feat: description"`
4. **Push branch:** `git push origin feature/[name]`
5. **Create PR:** Merge to `develop` first, then `main`

### **Available Scripts:**
```bash
# Development
npm run dev              # Start both Rust and Electron
npm run dev:rust         # Start Rust CLI only
npm run dev:electron     # Start Electron only

# Building
npm run build            # Build both components
npm run build:rust       # Build Rust CLI
npm run build:electron   # Build Electron app

# Testing
npm run test             # Run all tests
npm run test:rust        # Run Rust tests
npm run test:electron    # Run Electron tests

# Running
npm start                # Start the menu bar app
```

## 🔧 **Development Tips**

### **Rust Development:**
```bash
cd src/agent
cargo run -- summary      # Test summary command
cargo run -- status       # Test status command
cargo test               # Run tests
```

### **Electron Development:**
```bash
cd src/electron-app
npm start                # Start menu bar app
npm run dev              # Development mode with hot reload
```

### **Debugging:**
- **Rust:** Use `println!` or `tracing` for logging
- **Electron:** Check console output in terminal
- **Integration:** Monitor `spawn()` calls between layers

## 📋 **Work Tracking**

### **Current Work Items:**
- See `docs/engineering/work-items.md` for detailed tracking
- Use conventional commit messages: `feat:`, `fix:`, `docs:`, etc.
- Update work items as you complete tasks

### **Commit Message Format:**
```
type(scope): description

feat(ui): add status indicator to menu
fix(rust): resolve file path issue
docs(setup): update development guide
```

## 🚀 **Deployment**

### **Local Testing:**
```bash
npm run build
npm start
```

### **Distribution:**
```bash
cd src/electron-app
npm run build
```

## 🐛 **Troubleshooting**

### **Common Issues:**
1. **Electron not found:** Run `npm run install:all`
2. **Rust build errors:** Run `cargo clean && cargo build`
3. **Icon not showing:** Check `assets/icon.png` exists
4. **Menu not working:** Verify Rust CLI is built

### **Getting Help:**
- Check work items in `docs/engineering/work-items.md`
- Review development setup in this file
- Check console output for errors 