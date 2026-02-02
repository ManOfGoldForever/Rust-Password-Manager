#!/bin/bash
set -e

echo "Starting build process..."

echo "Cleaning up old builds..."
rm -f pm-linux-mac.tar.gz pm-windows.zip

cargo build --release

cargo build --release --target x86_64-pc-windows-gnu

echo "Packaging Linux..."
mkdir -p pm-linux-mac
cp target/release/pm linux-mac-install.sh linux-mac-uninstall.sh pm-linux-mac/
tar -czvf pm-linux-mac.tar.gz pm-linux-mac/

echo "Packaging Windows..."
mkdir -p pm-windows
cp target/x86_64-pc-windows-gnu/release/pm.exe windows-install.ps1 windows-uninstall.ps1 pm-windows/
zip -r pm-windows.zip pm-windows/

rm -rf pm-linux-mac pm-windows
echo "Done! Upload pm-linux-mac.tar.gz and pm-windows.zip to GitHub."
