#!/bin/sh

REPO="mitty1293/oboegaki"
BIN_NAME="obo"

# Create download URL
URL="https://github.com/$REPO/releases/latest/download/$BIN_NAME"

# Download the binary
sudo curl -L $URL -o /usr/local/bin/$BIN_NAME

# Grant execution permission
sudo chmod +x /usr/local/bin/$BIN_NAME

echo "$BIN_NAME has been installed to /usr/local/bin"
