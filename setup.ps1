# PowerShell setup script for Brain Rot Tracker

# Change to frontend directory and install dependencies
Set-Location frontend
npm install
Set-Location ..

# Change to backend directory and install dependencies
Set-Location backend
npm install
Set-Location ..

# Check if Docker Compose is installed
try {
    $dockerComposeVersion = docker-compose version
    Write-Host "Docker Compose is installed. Starting services..." -ForegroundColor Green
    docker-compose up --build
}
catch {
    Write-Host "Docker Compose is not installed. Please install Docker Desktop with Docker Compose." -ForegroundColor Red
    Write-Host "Alternatively, you can manually run the services:" -ForegroundColor Yellow
    Write-Host "1. Start MongoDB" -ForegroundColor Cyan
    Write-Host "2. Start Redis" -ForegroundColor Cyan
    Write-Host "3. Run 'npm run dev' in backend directory" -ForegroundColor Cyan
    Write-Host "4. Run 'npm run dev' in frontend directory" -ForegroundColor Cyan
} 