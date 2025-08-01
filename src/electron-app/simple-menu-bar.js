const { app, Tray, Menu, nativeImage } = require('electron');
const { spawn } = require('child_process');
const path = require('path');

console.log('🚀 Starting Rae Agent menu bar app...');

let tray = null;
let statusCheckInterval = null;
let customIcon = null;
let scheduledJobs = [];
let schedulerStatus = 'unknown';

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

function updateSchedulerStatus(status) {
  if (schedulerStatus === status) return;
  
  schedulerStatus = status;
  console.log(`Scheduler status changed to: ${status}`);
  
  // Update the menu to reflect new scheduler status
  updateMenu();
}

function loadScheduledJobs() {
  console.log('Loading scheduled jobs...');
  
  const listProcess = spawn(RUST_CLI_PATH, ['scheduler', 'list'], {
    stdio: 'pipe'
  });
  
  let output = '';
  
  listProcess.stdout.on('data', (data) => {
    output += data.toString();
  });
  
  listProcess.stderr.on('data', (data) => {
    console.error('Scheduler list error:', data.toString());
  });
  
  listProcess.on('close', (code) => {
    if (code === 0) {
      // Parse the job list output
      const lines = output.split('\n');
      scheduledJobs = [];
      
      for (const line of lines) {
        if (line.includes(' - ') && !line.includes('Scheduled Jobs:')) {
          const parts = line.split(' - ');
          if (parts.length >= 3) {
            const jobId = parts[0].trim();
            const jobName = parts[1].trim();
            const jobStatus = parts[2].trim();
            
            scheduledJobs.push({
              id: jobId,
              name: jobName,
              status: jobStatus
            });
          }
        }
      }
      
      console.log(`Loaded ${scheduledJobs.length} scheduled jobs`);
      updateSchedulerStatus('healthy');
      updateMenu();
    } else {
      console.error('Failed to load scheduled jobs');
      updateSchedulerStatus('error');
      updateMenu();
    }
  });
}

function toggleJob(jobId, enabled) {
  console.log(`${enabled ? 'Enabling' : 'Disabling'} job: ${jobId}`);
  
  const action = enabled ? 'enable' : 'disable';
  const toggleProcess = spawn(RUST_CLI_PATH, ['scheduler', action, jobId], {
    stdio: 'pipe'
  });
  
  toggleProcess.on('close', (code) => {
    if (code === 0) {
      console.log(`Job ${action} successful`);
      // Reload jobs to get updated status
      loadScheduledJobs();
    } else {
      console.error(`Failed to ${action} job`);
    }
  });
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
  
  // Build scheduled jobs menu items
  const scheduledJobsMenu = [];
  
  if (scheduledJobs.length > 0) {
    scheduledJobs.forEach(job => {
      const isEnabled = job.status === 'Scheduled' || job.status === 'Running';
      const statusIcon = isEnabled ? '🟢' : '🔴';
      
      scheduledJobsMenu.push({
        label: `${statusIcon} ${job.name} - ${job.status}`,
        click: () => {
          toggleJob(job.id, !isEnabled);
        }
      });
    });
    
    scheduledJobsMenu.push({ type: 'separator' });
  }
  
  scheduledJobsMenu.push({
    label: '➕ Add New Job...',
    click: () => {
      console.log('Opening job creation...');
      spawn(RUST_CLI_PATH, ['scheduler', 'add'], {
        stdio: 'inherit'
      });
    }
  });
  
  scheduledJobsMenu.push({
    label: '📋 View History...',
    click: () => {
      console.log('Opening job history...');
      spawn(RUST_CLI_PATH, ['scheduler', 'status'], {
        stdio: 'inherit'
      });
    }
  });
  
  scheduledJobsMenu.push({
    label: '🔄 Refresh Jobs',
    click: () => {
      loadScheduledJobs();
    }
  });
  
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
      label: '📅 Scheduled Jobs',
      submenu: scheduledJobsMenu
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
    
    // Load scheduled jobs
    loadScheduledJobs();
    
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