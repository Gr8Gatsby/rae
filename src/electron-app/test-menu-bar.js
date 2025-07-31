const { app, Tray, Menu } = require('electron');

console.log('🧪 Testing macOS Menu Bar...');

let tray = null;

function createTestMenuBar() {
  console.log('🍎 Creating test menu bar icon...');
  
  try {
    // Try different approaches for macOS menu bar
    const approaches = [
      'R',           // Simple text
      'Rae',         // App name
      '🤖',          // Emoji
      '📱',          // Phone emoji
      '⚙️',          // Settings emoji
    ];
    
    for (const approach of approaches) {
      try {
        console.log(`Trying: "${approach}"`);
        tray = new Tray(approach);
        console.log(`✅ Success with: "${approach}"`);
        break;
      } catch (error) {
        console.log(`❌ Failed with: "${approach}" - ${error.message}`);
      }
    }
    
    if (!tray) {
      console.log('❌ All approaches failed');
      return;
    }
    
    tray.setToolTip('Rae Agent Test');
    
    const contextMenu = Menu.buildFromTemplate([
      {
        label: '🎉 Test Click',
        click: () => {
          console.log('✅ Menu item clicked successfully!');
        }
      },
      { type: 'separator' },
      {
        label: '🚪 Quit Test',
        click: () => {
          app.quit();
        }
      }
    ]);
    
    tray.setContextMenu(contextMenu);
    tray.on('click', () => {
      console.log('✅ Menu bar icon clicked!');
    });
    
    console.log('🎉 Test menu bar created! Look for the icon in your menu bar.');
    
  } catch (error) {
    console.error('❌ Failed to create test menu bar:', error);
  }
}

app.whenReady().then(() => {
  console.log('🚀 Test app starting...');
  
  if (process.platform === 'darwin') {
    app.dock.hide();
    console.log('📱 Dock hidden for menu bar app');
  }
  
  createTestMenuBar();
  console.log('✅ Test app ready!');
});

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
}); 