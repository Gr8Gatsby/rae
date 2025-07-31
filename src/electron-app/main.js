const { app, Tray, Menu, shell, dialog } = require('electron');
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

let tray = null;

// Path to the Rust CLI binary
const RUST_CLI_PATH = path.join(__dirname, '../agent/target/release/rae-agent');

function createMenuBar() {
  console.log('🍎 Creating macOS menu bar icon...');
  
  // For macOS menu bar, we need a proper icon
  // Let's try a simple approach first
  try {
    // Use a simple text icon - this should work on macOS
    tray = new Tray('R');
    console.log('✅ Menu bar icon created with "R"');
  } catch (error) {
    console.error('❌ Failed to create menu bar icon:', error.message);
    
    try {
      // Try with a different text
      tray = new Tray('Rae');
      console.log('✅ Menu bar icon created with "Rae"');
    } catch (textError) {
      console.error('❌ Text icon also failed:', textError.message);
      
      try {
        // Try with a simple emoji
        tray = new Tray('🤖');
        console.log('✅ Menu bar icon created with emoji');
      } catch (emojiError) {
        console.error('❌ All icon approaches failed');
        return;
      }
    }
  }
  
  tray.setToolTip('Rae Agent');

  // Create context menu
  const contextMenu = Menu.buildFromTemplate([
    {
      label: '📖 View Today\'s Summary',
      click: () => {
        console.log('Opening today\'s summary...');
        spawn(RUST_CLI_PATH, ['summary'], {
          stdio: 'inherit'
        });
      }
    },
    {
      label: '⚙️  Open Configuration',
      click: () => {
        console.log('Opening configuration...');
        spawn(RUST_CLI_PATH, ['config'], {
          stdio: 'inherit'
        });
      }
    },
    { type: 'separator' },
    {
      label: '📊 Check Status',
      click: () => {
        console.log('Checking status...');
        spawn(RUST_CLI_PATH, ['status'], {
          stdio: 'inherit'
        });
      }
    },
    { type: 'separator' },
    {
      label: '🚪 Quit Rae',
      click: () => {
        app.quit();
      }
    }
  ]);

  tray.setContextMenu(contextMenu);

  // Handle menu bar icon click
  tray.on('click', () => {
    // Show status on click
    spawn(RUST_CLI_PATH, ['status'], {
      stdio: 'inherit'
    });
  });
  
  console.log('🎉 Menu bar ready! Look for the icon in your menu bar.');
}

// App lifecycle
app.whenReady().then(() => {
  console.log('🚀 Rae Agent starting...');
  
  if (process.platform === 'darwin') {
    // macOS specific setup
    console.log('🍎 Configuring for macOS menu bar...');
    
    // Hide dock icon - this is crucial for menu bar apps
    app.dock.hide();
    console.log('📱 Dock icon hidden');
    
    // Create menu bar icon
    createMenuBar();
  } else {
    // Windows/Linux system tray
    console.log('🖥️  Configuring for system tray...');
    createMenuBar();
  }

  // Start the Rust agent in background mode
  console.log('🔧 Starting Rust backend...');
  const rustProcess = spawn(RUST_CLI_PATH, ['start'], {
    stdio: 'pipe',
    detached: true
  });

  rustProcess.stdout.on('data', (data) => {
    console.log(`Rust: ${data}`);
  });

  rustProcess.stderr.on('data', (data) => {
    console.error(`Rust error: ${data}`);
  });

  rustProcess.on('close', (code) => {
    console.log(`Rust process exited with code ${code}`);
  });
  
  console.log('✅ Rae Agent is ready! Check your menu bar for the "R" icon.');
});

app.on('window-all-closed', () => {
  // Keep app running when windows are closed
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('before-quit', () => {
  // Clean up tray
  if (tray) {
    tray.destroy();
  }
}); 