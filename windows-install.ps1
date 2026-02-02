$binary = "pm.exe"

if (Test-Path ".\$binary") {
    $binDir = "$HOME\.cargo\bin"
    if (!(Test-Path $binDir)) { New-Item -ItemType Directory -Path $binDir }

    Write-Host "Installing $binary to $binDir..." -ForegroundColor Cyan
    Copy-Item ".\$binary" "$binDir"
    Write-Host "Done! Restart your terminal and type 'pm'." -ForegroundColor Green
} else {
    Write-Host "Error: $binary not found in this folder." -ForegroundColor Red
}
