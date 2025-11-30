# Zere CLI Installation Script for Windows
# Usage: irm https://raw.githubusercontent.com/umitkavala/zeredata-cli/main/install.ps1 | iex

$ErrorActionPreference = 'Stop'

$REPO = "umitkavala/zeredata-cli"
$BINARY_NAME = "zere.exe"
$INSTALL_DIR = "$env:LOCALAPPDATA\Zere"

Write-Host "Installing Zere CLI..." -ForegroundColor Green

# Detect architecture
$ARCH = if ([Environment]::Is64BitOperatingSystem) { "amd64" } else { "386" }
$BINARY_FILE = "zere-windows-$ARCH.exe"
$DOWNLOAD_URL = "https://github.com/$REPO/releases/latest/download/$BINARY_FILE"

Write-Host "Detected: windows-$ARCH" -ForegroundColor Yellow
Write-Host "Download URL: $DOWNLOAD_URL" -ForegroundColor Yellow

# Create installation directory
if (-not (Test-Path $INSTALL_DIR)) {
    New-Item -ItemType Directory -Path $INSTALL_DIR -Force | Out-Null
}

# Download binary
Write-Host "Downloading Zere CLI..." -ForegroundColor Yellow
$BINARY_PATH = Join-Path $INSTALL_DIR $BINARY_NAME

try {
    Invoke-WebRequest -Uri $DOWNLOAD_URL -OutFile $BINARY_PATH -UseBasicParsing
} catch {
    Write-Host "Error downloading binary: $_" -ForegroundColor Red
    exit 1
}

# Add to PATH if not already there
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$INSTALL_DIR*") {
    Write-Host "Adding $INSTALL_DIR to PATH..." -ForegroundColor Yellow
    [Environment]::SetEnvironmentVariable(
        "Path",
        "$CurrentPath;$INSTALL_DIR",
        "User"
    )
    $env:Path = "$env:Path;$INSTALL_DIR"
}

# Verify installation
Write-Host ""
Write-Host "✓ Zere CLI installed successfully!" -ForegroundColor Green
Write-Host "Location: $BINARY_PATH" -ForegroundColor Green
Write-Host ""
Write-Host "Get started:" -ForegroundColor Yellow
Write-Host "  zere login              # Login to your account"
Write-Host "  zere --interactive      # Launch TUI mode"
Write-Host "  zere --help             # Show all commands"
Write-Host ""
Write-Host "Note: You may need to restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
