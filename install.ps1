# PowerShell script to install project dependencies

# Function to check and install dependencies
function Install-Dependencies {
    param (
        [string]$Directory
    )

    Set-Location $Directory
    Write-Host "Installing dependencies in $Directory..." -ForegroundColor Green
    
    try {
        npm install
        if ($LASTEXITCODE -ne 0) {
            throw "npm install failed"
        }
    }
    catch {
        Write-Host "Error installing dependencies in $Directory" -ForegroundColor Red
        Write-Host $_.Exception.Message -ForegroundColor Red
    }
    
    Set-Location ..
}

# Ensure npm is available
try {
    $npmVersion = npm --version
    Write-Host "npm version: $npmVersion" -ForegroundColor Green
}
catch {
    Write-Host "npm is not installed. Please install Node.js and npm." -ForegroundColor Red
    exit 1
}

# Install frontend dependencies
Install-Dependencies "frontend"

# Install backend dependencies
Install-Dependencies "backend"

Write-Host "All dependencies installed successfully!" -ForegroundColor Green 