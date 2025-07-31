const { app, Tray, Menu, nativeImage } = require('electron');
const { spawn } = require('child_process');
const path = require('path');

console.log('🚀 Starting Rae Agent menu bar app...');

let tray = null;
let statusCheckInterval = null;
let customIcon = null;

// Path to the Rust CLI binary
const RUST_CLI_PATH = path.join(__dirname, '../agent/target/release/rae-agent');

let currentStatus = 'offline';

function updateStatusDisplay(status) {
  if (currentStatus === status) return;
  
  currentStatus = status;
  console.log(`Status changed to: ${status}`);
  
  // Update tooltip to show status
  let statusText = 'Rae Agent';
  if (status === 'online') {
    statusText = 'Rae Agent - Online';
  } else if (status === 'offline') {
    statusText = 'Rae Agent - Offline';
  } else if (status === 'starting') {
    statusText = 'Rae Agent - Starting...';
  }
  
  tray.setToolTip(statusText);
  
  // Keep the custom icon, just update tooltip for status
  if (customIcon) {
    tray.setImage(customIcon);
  }
  
  // Update the menu to reflect new status
  updateMenu();
}

function updateMenu() {
  let statusLabel = 'Status: Unknown';
  if (currentStatus === 'online') {
    statusLabel = 'Status: Online';
  } else if (currentStatus === 'offline') {
    statusLabel = 'Status: Offline';
  } else if (currentStatus === 'starting') {
    statusLabel = 'Status: Starting...';
  }
  
  const contextMenu = Menu.buildFromTemplate([
    {
      label: statusLabel,
      enabled: false
    },
    { type: 'separator' },
    {
      label: 'View Today\'s Summary',
      click: () => {
        console.log('Opening today\'s summary...');
        spawn(RUST_CLI_PATH, ['summary'], {
          stdio: 'inherit'
        });
      }
    },
    {
      label: 'Open Configuration',
      click: () => {
        console.log('Opening configuration...');
        spawn(RUST_CLI_PATH, ['config'], {
          stdio: 'inherit'
        });
      }
    },
    { type: 'separator' },
    {
      label: 'Quit Rae',
      click: () => {
        app.quit();
      }
    }
  ]);
  
  tray.setContextMenu(contextMenu);
}

function checkRaeStatus() {
  const statusProcess = spawn(RUST_CLI_PATH, ['status'], {
    stdio: 'pipe'
  });
  
  statusProcess.stdout.on('data', (data) => {
    const output = data.toString();
    if (output.includes('✅ Agent is running') || output.includes('Status: Operational')) {
      updateStatusDisplay('online');
    } else {
      updateStatusDisplay('offline');
    }
  });
  
  statusProcess.stderr.on('data', (data) => {
    console.error('Status check error:', data.toString());
    updateStatusDisplay('offline');
  });
  
  statusProcess.on('close', (code) => {
    if (code !== 0) {
      updateStatusDisplay('offline');
    }
  });
}

app.whenReady().then(() => {
  console.log('✅ App ready, creating tray...');
  
  try {
    // Use custom icon from assets
    const iconPath = path.join(__dirname, 'assets/icon.png');
    
    if (require('fs').existsSync(iconPath)) {
      customIcon = nativeImage.createFromPath(iconPath);
      console.log('✅ Using custom icon from assets/icon.png');
    } else {
      // Fallback to simple image
      customIcon = nativeImage.createFromDataURL('data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==');
      console.log('📝 Using fallback icon (add assets/icon.png for custom icon)');
    }
    
    tray = new Tray(customIcon);
    console.log('✅ Tray created successfully');
    
    tray.setToolTip('Rae Agent');
    
    // Start status checking
    updateStatusDisplay('starting');
    checkRaeStatus();
    
    // Check status every 30 seconds
    statusCheckInterval = setInterval(checkRaeStatus, 30000);
    
    // Hide dock on macOS
    if (process.platform === 'darwin') {
      app.dock.hide();
      console.log('📱 Dock hidden');
    }
    
    console.log('🎉 Rae Agent ready! Status shown in menu and tooltip');
    
  } catch (error) {
    console.error('❌ Failed to create tray:', error.message);
    app.quit();
  }
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('before-quit', () => {
  if (statusCheckInterval) {
    clearInterval(statusCheckInterval);
  }
}); 