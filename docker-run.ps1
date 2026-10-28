<#
.SYNOPSIS
    Builds and starts the blog application using Docker Compose.
.DESCRIPTION
    Checks if Docker Desktop is running, builds the containers, and launches
    both backend (port 3000) and frontend (port 8080).
#>

Write-Host ""
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host " 🐳 Launching Git-Backed Markdown Blog in Docker      " -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host ""

# Verify docker CLI exists
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Error "Docker was not found in PATH. Please install Docker Desktop."
    exit 1
}

# Verify docker daemon is active
Write-Host "🔍 Checking Docker daemon..." -ForegroundColor Yellow
$dockerInfo = docker info 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host ""
    Write-Host "❌ Docker daemon is not running!" -ForegroundColor Red
    Write-Host "👉 Please launch Docker Desktop, wait for it to start, and re-run this script." -ForegroundColor Yellow
    Write-Host ""
    exit 1
}

Write-Host "🚀 Building and starting containers with Docker Compose..." -ForegroundColor Green
Write-Host "   - Frontend will be available at: http://localhost:8080" -ForegroundColor Cyan
Write-Host "   - Backend API will be available at: http://localhost:3000/api/posts" -ForegroundColor Cyan
Write-Host "💡 Press Ctrl+C to stop the containers." -ForegroundColor Gray
Write-Host ""

docker compose up --build
