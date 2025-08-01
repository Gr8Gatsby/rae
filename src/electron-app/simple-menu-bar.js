const { app, Tray, Menu, nativeImage, dialog, BrowserWindow } = require('electron');
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

async function showAddJobDialog() {
  console.log('Opening job creation form...');
  
  // Create a simple form dialog
  const result = await dialog.showMessageBox({
    type: 'question',
    title: 'Add New Scheduled Job',
    message: 'Create a new scheduled job',
    detail: 'This will open a form where you can enter the job details directly in the UI.',
    buttons: ['Open Form', 'Use Terminal', 'Cancel'],
    defaultId: 0,
    cancelId: 2
  });
  
  if (result.response === 0) {
    // Open form window
    createJobFormWindow();
  } else if (result.response === 1) {
    // Open terminal with the scheduler add command
    console.log('Opening terminal for job creation...');
    spawn(RUST_CLI_PATH, ['scheduler', 'add'], {
      stdio: 'inherit'
    });
  }
}

function createJobFormWindow() {
  const formWindow = new BrowserWindow({
    width: 500,
    height: 400,
    webPreferences: {
      nodeIntegration: true,
      contextIsolation: false
    },
    resizable: false,
    minimizable: false,
    maximizable: false,
    title: 'Add New Job',
    show: false
  });

  // Create HTML content for the form
  const htmlContent = `
    <!DOCTYPE html>
    <html>
    <head>
      <title>Add New Job</title>
      <style>
        body {
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
          margin: 20px;
          background: #f5f5f5;
        }
        .form-container {
          background: white;
          padding: 20px;
          border-radius: 8px;
          box-shadow: 0 2px 10px rgba(0,0,0,0.1);
        }
        .form-group {
          margin-bottom: 15px;
        }
        label {
          display: block;
          margin-bottom: 5px;
          font-weight: 500;
          color: #333;
        }
        input, textarea {
          width: 100%;
          padding: 8px 12px;
          border: 1px solid #ddd;
          border-radius: 4px;
          font-size: 14px;
          box-sizing: border-box;
        }
        textarea {
          height: 80px;
          resize: vertical;
        }
        .help-text {
          font-size: 12px;
          color: #666;
          margin-top: 4px;
        }
        .buttons {
          margin-top: 20px;
          text-align: right;
        }
        button {
          padding: 8px 16px;
          border: none;
          border-radius: 4px;
          font-size: 14px;
          cursor: pointer;
          margin-left: 10px;
        }
        .btn-primary {
          background: #007AFF;
          color: white;
        }
        .btn-secondary {
          background: #6c757d;
          color: white;
        }
        .btn-primary:hover {
          background: #0056b3;
        }
        .btn-secondary:hover {
          background: #545b62;
        }
      </style>
    </head>
    <body>
      <div class="form-container">
        <h2>Add New Scheduled Job</h2>
        <form id="jobForm">
          <div class="form-group">
            <label for="jobName">Job Name:</label>
            <input type="text" id="jobName" placeholder="e.g., daily-backup" required>
            <div class="help-text">A descriptive name for your job</div>
          </div>
          
          <div class="form-group">
            <label for="schedule">Schedule (Cron Expression):</label>
            <input type="text" id="schedule" placeholder="e.g., 0 2 * * *" required>
            <div class="help-text">Cron expression: minute hour day month weekday</div>
          </div>
          
          <div class="form-group">
            <label for="command">Command:</label>
            <textarea id="command" placeholder="e.g., backup-script.sh" required></textarea>
            <div class="help-text">The command to execute when the job runs</div>
          </div>
          
          <div class="buttons">
            <button type="button" class="btn-secondary" onclick="window.close()">Cancel</button>
            <button type="submit" class="btn-primary">Create Job</button>
          </div>
        </form>
      </div>
      
      <script>
        document.getElementById('jobForm').addEventListener('submit', function(e) {
          e.preventDefault();
          
          const jobName = document.getElementById('jobName').value.trim();
          const schedule = document.getElementById('schedule').value.trim();
          const command = document.getElementById('command').value.trim();
          
          if (!jobName || !schedule || !command) {
            alert('Please fill in all fields');
            return;
          }
          
          // Send data to main process
          const { ipcRenderer } = require('electron');
          ipcRenderer.send('create-job', { jobName, schedule, command });
        });
      </script>
    </body>
    </html>
  `;

  formWindow.loadURL('data:text/html;charset=utf-8,' + encodeURIComponent(htmlContent));
  
  formWindow.once('ready-to-show', () => {
    formWindow.show();
  });

  // Handle form submission
  const { ipcMain } = require('electron');
  ipcMain.once('create-job', (event, data) => {
    console.log('Creating job with data:', data);
    
    // Call the Rust CLI with the form data
    const addProcess = spawn(RUST_CLI_PATH, [
      'scheduler', 'add',
      '--name', data.jobName,
      '--schedule', data.schedule,
      '--command', data.command
    ], {
      stdio: 'pipe'
    });

    let output = '';
    addProcess.stdout.on('data', (data) => {
      output += data.toString();
    });

    addProcess.stderr.on('data', (data) => {
      output += data.toString();
    });

    addProcess.on('close', (code) => {
      if (code === 0) {
        dialog.showMessageBox(formWindow, {
          type: 'info',
          title: 'Success',
          message: 'Job Created Successfully',
          detail: 'The job has been added to the scheduler.',
          buttons: ['OK']
        });
        formWindow.close();
        // Reload jobs in the main menu
        loadScheduledJobs();
      } else {
        dialog.showMessageBox(formWindow, {
          type: 'error',
          title: 'Error',
          message: 'Failed to Create Job',
          detail: output || 'An error occurred while creating the job.',
          buttons: ['OK']
        });
      }
    });
  });
}

async function showJobHistoryDialog() {
  console.log('Opening job history dialog...');
  
  const result = await dialog.showMessageBox({
    type: 'info',
    title: 'Job History',
    message: 'Scheduler Status',
    detail: 'This will open a terminal window showing detailed job history and status information.',
    buttons: ['Open Terminal', 'Cancel'],
    defaultId: 0,
    cancelId: 1
  });
  
  if (result.response === 0) {
    console.log('Opening terminal for job history...');
    spawn(RUST_CLI_PATH, ['scheduler', 'status'], {
      stdio: 'inherit'
    });
  }
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
      showAddJobDialog();
    }
  });
  
  scheduledJobsMenu.push({
    label: '📋 View History...',
    click: () => {
      showJobHistoryDialog();
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