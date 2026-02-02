#!/bin/bash
BINARY="pm"

if [ -f "./$BINARY" ]; then
    echo "Installing $BINARY to /usr/local/bin..."
    sudo cp "./$BINARY" /usr/local/bin/
    sudo chmod +x /usr/local/bin/$BINARY
    echo "Done! You can now use 'pm' from any terminal."
else
    echo "Error: Binary '$BINARY' not found. Make sure you are in the folder with the downloaded file."
fi
