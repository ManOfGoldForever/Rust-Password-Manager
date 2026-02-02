Write-Host "Uninstalling pm..." -ForegroundColor Cyan
Remove-Item "$HOME\.cargo\bin\pm.exe" -ErrorAction SilentlyContinue
Remove-Item "$HOME\.my_pass_manager" -Recurse -Force -ErrorAction SilentlyContinue
Write-Host "pm and all encrypted data have been removed." -ForegroundColor Yellow
