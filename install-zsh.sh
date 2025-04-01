#!/bin/bash

# Create a folder for the application in the user's .config directory
CONFIG_DIR="$HOME/.config/stoac"
if [ ! -d "$CONFIG_DIR" ]; then
    echo "Creating $CONFIG_DIR directory..."
    mkdir -p "$CONFIG_DIR"
else
    echo "$CONFIG_DIR already exists."
fi

FILE_URL="https://github.com/Maxsak-dev/stoac/raw/main/stoac.zsh"
DEST_FILE="$CONFIG_DIR/stoac.zsh"

echo "Downloading zsh file from GitHub..."
curl -L "$FILE_URL" -o "$DEST_FILE"

# Check if the download was successful
if [ $? -eq 0 ]; then
    echo "File downloaded successfully to $DEST_FILE"
else
    echo "Failed to download the file."
fi

echo "Fetching the latest release information from GitHub..."
LATEST_RELEASE_INFO=$(curl -s https://api.github.com/repos/Maxsak-dev/stoac/releases/latest)
DOWNLOAD_URL=$(echo "$LATEST_RELEASE_INFO" | grep -o '"browser_download_url": "[^"]*' | grep -E 'stoac' | sed 's/"browser_download_url": "//')

# Check if the download URL was found
if [ -z "$DOWNLOAD_URL" ]; then
    echo "Failed to find the binary download URL."
    exit 1
fi

echo "Downloading the latest binary from GitHub release..."
curl -L "$DOWNLOAD_URL" -o "$(pwd)/stoac"

echo "Installing the binary..."
sudo install -m 755 "$(pwd)/stoac" /usr/local/bin/stoac

echo "Cleaning up: removing the binary from the current directory..."
rm -f "$(pwd)/stoac"

# Verify the installation
if command -v stoac &>/dev/null; then
    echo "Binary installed successfully. You can run it using 'stoac'."
else
    echo "Failed to install the binary."
fi

# Add the line to the .zshrc file
ZSHRC_FILE="$HOME/.zshrc"
if ! grep -q "echo hello" "$ZSHRC_FILE"; then
    echo "Adding reference to $ZSHRC_FILE..."
    echo "source '$DEST_FILE'" >> "$ZSHRC_FILE"
else
    echo "Reference already in $ZSHRC_FILE."
fi
