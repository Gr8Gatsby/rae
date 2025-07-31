# Rae Electron App

Cross-platform system tray interface for the Rae agent, built with Electron.

## Architecture

```
Electron App (UI Layer)
  ↓ spawn()
Rust CLI (Backend)
  ↓ executes
Core Logic
```

## Features

- ✅ **Cross-platform tray/menu bar support**
- ✅ **Native menu items**: View Summary, Open Config, Check Status, Quit
- ✅ **Background Rust process**: Keeps agent running
- ✅ **macOS menu bar**: Proper integration on macOS
- ✅ **Windows/Linux system tray**: Standard tray behavior

## Setup

1. **Build Rust CLI**:
   ```bash
   cd src/agent
   cargo build --release
   ```

2. **Install Electron dependencies**:
   ```bash
   cd src/electron-app
   npm install
   ```

3. **Run the app**:
   ```bash
   npm start
   ```

## Development

```bash
# Development mode with hot reload
npm run dev

# Build distributable
npm run build
```

## Menu Items

- **View Today's Summary**: Opens today's summary file
- **Open Configuration**: Opens the Rae config file
- **Check Status**: Shows agent status and health
- **Quit Rae**: Exits the application

## File Structure

```
src/electron-app/
├── main.js          # Main Electron process
├── package.json     # Dependencies and build config
├── assets/          # Icons and resources
└── README.md        # This file
```

## Integration with Rust

The Electron app calls the Rust CLI using `child_process.spawn()`:

```javascript
const rustProcess = spawn('./rae-agent', ['summary']);
```

This provides:
- **Separation of concerns**: UI vs backend logic
- **Performance**: Fast Rust backend
- **Maintainability**: Clear boundaries
- **Cross-platform**: Electron handles platform differences 