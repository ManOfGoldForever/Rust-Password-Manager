#!/bin/bash
echo "Removing pm binary and encrypted data..."
sudo rm -f /usr/local/bin/pm
rm -rf ~/.my_pass_manager
echo "Uninstalled successfully."
