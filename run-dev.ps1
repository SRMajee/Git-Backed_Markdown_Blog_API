<#
.SYNOPSIS
    Runs both the Rust/Axum backend and React/Vite frontend concurrently.
.DESCRIPTION
    Starts the Axum backend at http://127.0.0.1:3000 and the Vite dev server at http://localhost:5173.
    Pressing Ctrl+C cleanly terminates both processes.
#>

Write-Host ""
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host " 🦀 Starting Git-Backed Markdown Blog Development Env  " -ForegroundColor Cyan
Write-Host "=======================================================" -ForegroundColor Cyan
Write-Host ""

$scriptDir = $PSScriptRoot
if (-not $scriptDir) { $scriptDir = (Get-Location).Path }

# Verify dependencies
if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "cargo was not found in PATH. Please install Rust from https://rustup.rs"
    exit 1
}
if (-not (Get-Command pnpm -ErrorAction SilentlyContinue)) {
    Write-Error "pnpm was not found in PATH. Please install pnpm."
    exit 1
}

# Ensure frontend dependencies are installed
if (-not (Test-Path "$scriptDir\frontend\node_modules")) {
    Write-Host "📦 Installing frontend dependencies..." -ForegroundColor Yellow
    pnpm --dir "$scriptDir\frontend" install
}

# Start backend process
Write-Host "🚀 Starting Rust/Axum backend on http://127.0.0.1:3000..." -ForegroundColor Green
$backendProcess = Start-Process -FilePath "cargo" -ArgumentList "run" -WorkingDirectory "$scriptDir\backend" -PassThru

# Start frontend in foreground
Write-Host "⚡ Starting React/Vite frontend on http://localhost:5173..." -ForegroundColor Green
Write-Host "💡 Press Ctrl+C at any time to stop both servers." -ForegroundColor Gray
Write-Host ""

try {
    Set-Location "$scriptDir\frontend"
    pnpm dev
}
finally {
    Write-Host ""
    Write-Host "🛑 Shutting down backend..." -ForegroundColor Yellow
    if ($backendProcess -and -not $backendProcess.HasExited) {
        Stop-Process -Id $backendProcess.Id -Force -ErrorAction SilentlyContinue
    }
    # Also clean up compiled binary if cargo launched it as child
    Get-Process "git-blog-api" -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
    Write-Host "✅ All processes stopped." -ForegroundColor Green
}

