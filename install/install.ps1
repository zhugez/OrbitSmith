#Requires -Version 5.1
<#
.SYNOPSIS
    OrbitSmith Installer for Windows
.DESCRIPTION
    Downloads the latest OrbitSmith binary and adds it to the user's PATH.
.EXAMPLE
    irm https://raw.githubusercontent.com/zhugez/OrbitSmith/master/install/install.ps1 | iex
#>

$ErrorActionPreference = 'Stop'

# Configuration
$Repo = "zhugez/OrbitSmith"
$BinaryName = "orbitsmith"
$InstallDir = "$env:USERPROFILE\.orbitsmith\bin"
$Version = if ($env:ORBITSMITH_VERSION) { $env:ORBITSMITH_VERSION } else { "latest" }

# Banner
Write-Host ""
Write-Host "  ╔══════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "  ║       🛰️  OrbitSmith Installer  🛰️        ║" -ForegroundColor Cyan
Write-Host "  ╚══════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Detect architecture
$Arch = if ([Environment]::Is64BitOperatingSystem) { "x86_64" } else { "x86" }
$AssetName = "${BinaryName}-windows-${Arch}.exe"

Write-Host "  ℹ  Detected platform: windows/${Arch}" -ForegroundColor Cyan

# Resolve download URL
if ($Version -eq "latest") {
    Write-Host "  ℹ  Fetching latest release..." -ForegroundColor Cyan
    $DownloadUrl = "https://github.com/${Repo}/releases/latest/download/${AssetName}"
} else {
    $DownloadUrl = "https://github.com/${Repo}/releases/download/${Version}/${AssetName}"
}

# Create install directory
if (!(Test-Path $InstallDir)) {
    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
}

# Download binary
$DestPath = Join-Path $InstallDir "${BinaryName}.exe"
Write-Host "  ℹ  Downloading OrbitSmith..." -ForegroundColor Cyan

try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12
    Invoke-WebRequest -Uri $DownloadUrl -OutFile $DestPath -UseBasicParsing
    Write-Host "  ✅ Binary downloaded to $DestPath" -ForegroundColor Green
} catch {
    Write-Host "  ❌ Download failed: $_" -ForegroundColor Red
    exit 1
}

# Add to User PATH (persistent)
$CurrentPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($CurrentPath -notlike "*$InstallDir*") {
    $NewPath = "$InstallDir;$CurrentPath"
    [Environment]::SetEnvironmentVariable("Path", $NewPath, "User")
    Write-Host "  ✅ Added $InstallDir to User PATH (persistent)" -ForegroundColor Green

    # Also update current session
    $env:Path = "$InstallDir;$env:Path"
} else {
    Write-Host "  ℹ  PATH already contains $InstallDir" -ForegroundColor Cyan
}

# Verify installation
Write-Host ""
try {
    $VersionOutput = & $DestPath --version 2>&1
    Write-Host "  ✅ OrbitSmith installed successfully! 🚀" -ForegroundColor Green
    Write-Host "     $VersionOutput" -ForegroundColor White
} catch {
    Write-Host "  ✅ Binary installed. Restart your terminal to use 'orbitsmith'." -ForegroundColor Green
}

Write-Host ""
Write-Host "  Get started:" -ForegroundColor Green
Write-Host "    orbitsmith init         # Initialize workspace with 865+ AI skills"
Write-Host "    orbitsmith sync-skills  # Sync latest skills"
Write-Host "    orbitsmith status       # Check workspace status"
Write-Host ""
Write-Host "  ⚠️  Please restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
Write-Host ""
